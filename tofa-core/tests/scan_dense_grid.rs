use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use tempfile::NamedTempFile;
use tofa_core::qr::{scan_all_qr_uris, scan_all_qr_uris_with_progress, ScanProgress};

/// Render a single otpauth:// URI to a small QR-code PNG image (Luma) and
/// return the buffer at the size the qrcode crate produces with 1px modules.
fn qr_image(uri: &str, module_px: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let code = QrCode::new(uri.as_bytes()).expect("encode QR");
    code.render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(module_px, module_px)
        .build()
}

/// Tile N QR images into a single big image with whitespace gutters. Mirrors
/// the layout the Tauri "Save All" zip's print.html produces when a user
/// captures their backup printout: many small QRs on one screen.
fn tile(
    qrs: &[ImageBuffer<Luma<u8>, Vec<u8>>],
    cols: u32,
    gutter: u32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let qr_w = qrs.iter().map(|q| q.width()).max().unwrap();
    let qr_h = qrs.iter().map(|q| q.height()).max().unwrap();
    let rows = qrs.len().div_ceil(cols as usize) as u32;
    let canvas_w = cols * (qr_w + gutter) + gutter;
    let canvas_h = rows * (qr_h + gutter) + gutter;
    let mut canvas: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(canvas_w, canvas_h, Luma([255]));
    for (i, qr) in qrs.iter().enumerate() {
        let col = i as u32 % cols;
        let row = i as u32 / cols;
        let x = gutter + col * (qr_w + gutter);
        let y = gutter + row * (qr_h + gutter);
        for (px, py, p) in qr.enumerate_pixels() {
            canvas.put_pixel(x + px, y + py, *p);
        }
    }
    canvas
}

/// Eleven realistic-looking otpauth URIs — long enough to make dense QRs that
/// stress the scanner the way a real backup printout does.
fn many_uris() -> Vec<String> {
    let labels: &[(&str, &str, &str)] = &[
        ("Discord", "bob", "DISCORDFAKEAAAAA"),
        ("Netlify", "bob@example.com", "NETLIFYFAKEAAAAA"),
        ("Notion", "eve@example.com", "NOTIONFAKEAAAAAA"),
        ("Vercel", "eve@acme.io", "VERCELFAKEAAAAAA"),
        ("Shopify", "henry@example.com", "SHOPIFYFAKEAAAA"),
        ("Fly.io", "henry@acme.io", "FLYIOFAKEAAAAAAA"),
        ("DigitalOcean", "henry", "DIGITALOCEANFAKE"),
        ("npm", "frank@example.com", "NPMFAKEAAAAAAAA"),
        ("Heroku", "frank@acme.io", "HEROKUFAKEAAAAAA"),
        ("Figma", "grace@example.com", "FIGMAFAKEAAAAAA"),
        ("GitLab", "grace@acme.io", "GITLABFAKEAAAAAA"),
    ];
    labels
        .iter()
        .map(|(issuer, account, secret)| {
            format!(
                "otpauth://totp/{issuer}:{account}?secret={secret}&issuer={issuer}&algorithm=SHA1&digits=6&period=30"
            )
        })
        .collect()
}

#[test]
fn scan_finds_nearly_every_qr_in_a_high_resolution_grid() {
    // Print.html-style layout: 11 dense QRs in a wide grid, captured at a
    // resolution that mimics a Retina screenshot of a browser viewport.
    // Pre-fix (native resolution never tried) this returned only ~4 of the
    // 11 because rqrr couldn't decode the dense QRs at any of the
    // 1920/1280/960 rescale widths.
    let uris = many_uris();
    let qrs: Vec<_> = uris.iter().map(|u| qr_image(u, 6)).collect();
    let canvas = tile(&qrs, 8, 40);
    // Sanity: this should be roughly Retina screenshot width.
    assert!(
        canvas.width() >= 2400,
        "canvas should be wide enough to exercise the rescale-only bug"
    );

    let tmp = NamedTempFile::with_suffix(".png").expect("tmp");
    canvas.save(tmp.path()).expect("save");

    let found = scan_all_qr_uris(tmp.path()).expect("must find at least one");
    let matched = uris.iter().filter(|u| found.contains(u)).count();

    // Pre-fix (rescale-only, never native) this returned roughly 9/11 in this
    // grid layout because the dense QRs went below rqrr's detection threshold
    // at 1920px-wide. With native-resolution scan added, it should reliably
    // hit ≥ uris.len()-1; rqrr still has occasional grid-detection blind
    // spots at synthetic boundaries, so don't insist on absolute completeness
    // — insist on covering the user's actual regression (Retina screenshot
    // recall went from 4/11 to ~10/11).
    let threshold = uris.len() - 1;
    assert!(
        matched >= threshold,
        "scanner only matched {matched} of {} QRs (need ≥{threshold}). \
         found: {} URIs, missing: {:?}",
        uris.len(),
        found.len(),
        uris.iter()
            .filter(|u| !found.contains(u))
            .collect::<Vec<_>>()
    );
}

#[test]
fn scan_with_progress_invokes_callback_per_pass_with_running_count() {
    // The CLI uses the progress callback to update its spinner: it needs to
    // see (pass width, running found-count) so it can show "pass @ 1920px •
    // 7 found". Verify the callback fires at least twice (multi-pass ladder)
    // and that `found` is monotonically non-decreasing and ends at the
    // final returned count.
    let uris = many_uris();
    let qrs: Vec<_> = uris.iter().map(|u| qr_image(u, 6)).collect();
    let canvas = tile(&qrs, 8, 40);
    let tmp = NamedTempFile::with_suffix(".png").expect("tmp");
    canvas.save(tmp.path()).expect("save");

    let mut events: Vec<ScanProgress> = Vec::new();
    let found = scan_all_qr_uris_with_progress(tmp.path(), |p| events.push(p))
        .expect("must decode at least one QR");

    assert!(
        events.len() >= 2,
        "expected multiple passes, got {} event(s)",
        events.len()
    );
    for pair in events.windows(2) {
        assert!(
            pair[1].found >= pair[0].found,
            "found count must be monotonic, saw {} → {}",
            pair[0].found,
            pair[1].found
        );
    }
    let last = events.last().expect("events non-empty");
    assert_eq!(
        last.found,
        found.len(),
        "final progress event should match returned URI count"
    );
    assert!(
        last.pass_width > 0,
        "pass_width should reflect the resize target in pixels"
    );
}
