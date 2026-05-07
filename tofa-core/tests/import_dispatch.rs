//! Tests for the unified `import::parse_file(path)` dispatcher.
//!
//! Goal: any file the user can hand us — image, zip-of-images, JSON
//! export, CSV, plain-text URI list — should route through one entry
//! point and return `Vec<OtpSecret>`. CLI / TUI / desktop app all share
//! this dispatcher so format support is identical across surfaces.

use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use std::io::Write;
use tempfile::NamedTempFile;
use tofa_core::import::parse_file;

fn qr_image(uri: &str, module_px: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let code = QrCode::new(uri.as_bytes()).expect("encode QR");
    code.render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(module_px, module_px)
        .build()
}

fn save_png(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> NamedTempFile {
    let f = NamedTempFile::with_suffix(".png").expect("tmp");
    img.save(f.path()).expect("save");
    f
}

const URI_DISCORD: &str =
    "otpauth://totp/Discord:bob?secret=DISCORDFAKEAAAAA&issuer=Discord&algorithm=SHA1&digits=6&period=30";
const URI_NETLIFY: &str =
    "otpauth://totp/Netlify:bob@example.com?secret=NETLIFYFAKEAAAAA&issuer=Netlify&algorithm=SHA1&digits=6&period=30";
const URI_VERCEL: &str =
    "otpauth://totp/Vercel:eve@acme.io?secret=VERCELFAKEAAAAAA&issuer=Vercel&algorithm=SHA1&digits=6&period=30";

#[test]
fn parse_file_dispatches_png_with_single_qr() {
    let img = qr_image(URI_DISCORD, 8);
    let png = save_png(&img);
    let secrets = parse_file(png.path()).expect("must parse single-QR png");
    assert_eq!(secrets.len(), 1);
    assert_eq!(secrets[0].secret, "DISCORDFAKEAAAAA");
}

#[test]
fn parse_file_dispatches_png_with_multiple_qrs() {
    // Mirrors the `print.html` layout: multiple QRs tiled into one PNG.
    // Without parse_file's image dispatch using scan_all_qr_uris, this
    // would only return one of the three.
    let qrs: Vec<_> = [URI_DISCORD, URI_NETLIFY, URI_VERCEL]
        .iter()
        .map(|u| qr_image(u, 8))
        .collect();
    // Different URIs encode to different QR versions, so each image may
    // have a different module count. Size the cells to the largest.
    let cell_w = qrs.iter().map(|q| q.width()).max().unwrap();
    let cell_h = qrs.iter().map(|q| q.height()).max().unwrap();
    let gutter = 30u32;
    let canvas_w = qrs.len() as u32 * (cell_w + gutter) + gutter;
    let canvas_h = cell_h + 2 * gutter;
    let mut canvas: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(canvas_w, canvas_h, Luma([255]));
    for (i, qr) in qrs.iter().enumerate() {
        let x = gutter + i as u32 * (cell_w + gutter);
        let y = gutter;
        for (px, py, p) in qr.enumerate_pixels() {
            canvas.put_pixel(x + px, y + py, *p);
        }
    }
    let png = save_png(&canvas);

    let secrets = parse_file(png.path()).expect("must parse multi-QR png");
    let secrets_set: std::collections::HashSet<_> =
        secrets.iter().map(|s| s.secret.as_str()).collect();
    assert!(secrets_set.contains("DISCORDFAKEAAAAA"));
    assert!(secrets_set.contains("NETLIFYFAKEAAAAA"));
    assert!(secrets_set.contains("VERCELFAKEAAAAAA"));
}

#[test]
fn parse_file_dispatches_zip_of_qr_images() {
    // Mirrors what `save_qr_zip` produces in the desktop app: a .zip
    // containing one PNG per account. Importing that zip back should
    // round-trip every secret. This is the loop closure: users export a
    // backup printout, and we accept it back without manual unzip.
    let pngs: Vec<_> = [URI_DISCORD, URI_NETLIFY, URI_VERCEL]
        .iter()
        .map(|u| qr_image(u, 8))
        .collect();

    let zip_file = NamedTempFile::with_suffix(".zip").expect("tmp zip");
    let mut writer = zip::ZipWriter::new(zip_file.reopen().expect("reopen"));
    let opts: zip::write::SimpleFileOptions =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (i, img) in pngs.iter().enumerate() {
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Png)
            .expect("encode");
        writer
            .start_file(format!("qr-{i}.png"), opts)
            .expect("start");
        writer.write_all(&buf.into_inner()).expect("write");
    }
    writer.finish().expect("finish zip");

    let secrets = parse_file(zip_file.path()).expect("must parse zip");
    let secrets_set: std::collections::HashSet<_> =
        secrets.iter().map(|s| s.secret.as_str()).collect();
    assert!(secrets_set.contains("DISCORDFAKEAAAAA"));
    assert!(secrets_set.contains("NETLIFYFAKEAAAAA"));
    assert!(secrets_set.contains("VERCELFAKEAAAAAA"));
}

#[test]
fn parse_file_dispatches_text_with_otpauth_uris() {
    let text = format!("{URI_DISCORD}\n{URI_NETLIFY}\n");
    let mut f = NamedTempFile::with_suffix(".txt").expect("tmp");
    f.write_all(text.as_bytes()).expect("write");
    let secrets = parse_file(f.path()).expect("must parse text uris");
    assert_eq!(secrets.len(), 2);
    let secrets_set: std::collections::HashSet<_> =
        secrets.iter().map(|s| s.secret.as_str()).collect();
    assert!(secrets_set.contains("DISCORDFAKEAAAAA"));
    assert!(secrets_set.contains("NETLIFYFAKEAAAAA"));
}

#[test]
fn parse_file_rejects_unsupported_extension() {
    let mut f = NamedTempFile::with_suffix(".bin").expect("tmp");
    f.write_all(b"random bytes").expect("write");
    assert!(parse_file(f.path()).is_err());
}

#[test]
fn parse_file_zip_ignores_non_image_entries() {
    // A zip with a README plus one QR image should still import the
    // single QR — non-image entries are skipped, not errored on.
    let zip_file = NamedTempFile::with_suffix(".zip").expect("tmp zip");
    let mut writer = zip::ZipWriter::new(zip_file.reopen().expect("reopen"));
    let opts: zip::write::SimpleFileOptions =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    writer.start_file("README.txt", opts).expect("start");
    writer.write_all(b"Backup printout\n").expect("write");

    let img = qr_image(URI_DISCORD, 8);
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)
        .expect("encode");
    writer.start_file("discord.png", opts).expect("start");
    writer.write_all(&buf.into_inner()).expect("write");
    writer.finish().expect("finish");

    let secrets = parse_file(zip_file.path()).expect("must parse zip");
    assert_eq!(secrets.len(), 1);
    assert_eq!(secrets[0].secret, "DISCORDFAKEAAAAA");
}
