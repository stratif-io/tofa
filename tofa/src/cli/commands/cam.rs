use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use tofa_core::totp::{format_code, generate_code_now, seconds_remaining_now};

#[derive(Args)]
pub struct CamArgs {
    /// Camera index passed to the browser (default: 0)
    #[arg(long, default_value = "0", value_name = "INDEX")]
    pub camera: u32,
    /// Override the account name (default: derived from QR metadata)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
}

/// Minimal single-use HTTP server.
/// Serves GET / → HTML page; waits for POST /result with the decoded URI body.
struct MiniServer {
    listener: TcpListener,
    port: u16,
}

impl MiniServer {
    fn bind() -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        Ok(Self { listener, port })
    }

    fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    /// Blocks until a POST /result arrives; returns the body.
    fn wait_for_result(self) -> std::io::Result<String> {
        let result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let result2 = result.clone();

        // Accept connections in a thread so we can serve the GET + wait for POST
        let handle = thread::spawn(move || {
            for stream in self.listener.incoming() {
                let Ok(stream) = stream else { continue };
                if let Some(r) = handle_conn(stream) {
                    *result2.lock().unwrap() = Some(r);
                    break;
                }
            }
        });

        handle.join().ok();
        let r = result.lock().unwrap().take().unwrap_or_default();
        Ok(r)
    }
}

fn handle_conn(mut stream: TcpStream) -> Option<String> {
    let mut reader = BufReader::new(stream.try_clone().ok()?);

    // Read request line + headers
    let mut request_line = String::new();
    reader.read_line(&mut request_line).ok()?;

    let mut content_length: usize = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(v) = trimmed.strip_prefix("Content-Length:") {
            content_length = v.trim().parse().unwrap_or(0);
        }
    }

    let is_post = request_line.starts_with("POST /result");
    let is_get = request_line.starts_with("GET /");

    if is_get {
        let html = build_html();
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        );
        stream.write_all(resp.as_bytes()).ok();
        return None; // Not the result yet
    }

    if is_post && content_length > 0 {
        let mut body = vec![0u8; content_length];
        use std::io::Read;
        reader.read_exact(&mut body).ok()?;
        let uri = String::from_utf8_lossy(&body).to_string();

        let resp = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(resp.as_bytes()).ok();

        return Some(uri);
    }

    // CORS preflight or other
    let resp =
        "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 0\r\n\r\n";
    stream.write_all(resp.as_bytes()).ok();
    None
}

fn build_html() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>tofa — scan QR</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body {
    background: #0d1117;
    color: #e6edf3;
    font-family: 'SF Mono', 'JetBrains Mono', monospace;
    display: flex; flex-direction: column;
    align-items: center; justify-content: center;
    height: 100vh; gap: 1.5rem;
  }
  h1 { color: #58a6ff; font-size: 1.5rem; letter-spacing: 0.05em; }
  #wrap {
    position: relative;
    width: min(480px, 90vw);
    aspect-ratio: 4/3;
    border-radius: 12px;
    overflow: hidden;
    background: #010409;
    box-shadow: 0 0 0 1px #21262d;
  }
  video { width: 100%; height: 100%; object-fit: cover; display: block; transform: scaleX(-1); }
  #overlay {
    position: absolute; inset: 0;
    display: flex; align-items: center; justify-content: center;
  }
  #finder {
    width: 52%; aspect-ratio: 1;
    position: relative;
  }
  /* Corner marks */
  #finder::before, #finder::after,
  #finder > span::before, #finder > span::after {
    content: '';
    position: absolute;
    width: 22%; height: 22%;
    border-color: #58a6ff;
    border-style: solid;
    border-width: 0;
  }
  #finder::before  { top:0; left:0;  border-top-width:3px; border-left-width:3px;  border-radius: 4px 0 0 0; }
  #finder::after   { top:0; right:0; border-top-width:3px; border-right-width:3px; border-radius: 0 4px 0 0; }
  #finder > span::before { bottom:0; left:0;  border-bottom-width:3px; border-left-width:3px;  border-radius: 0 0 0 4px; }
  #finder > span::after  { bottom:0; right:0; border-bottom-width:3px; border-right-width:3px; border-radius: 0 0 4px 0; }
  /* Scan line */
  #scanline {
    position: absolute; left: 0; right: 0;
    height: 2px; background: linear-gradient(90deg, transparent, #58a6ff, transparent);
    animation: scan 2s linear infinite;
    top: 0;
  }
  @keyframes scan { 0%{top:0} 100%{top:100%} }
  #status {
    font-size: 0.85rem;
    color: #6e7681;
    text-align: center;
    min-height: 1.5em;
    transition: color 0.3s;
  }
  #status.ok   { color: #58a6ff; }
  #status.warn { color: #d29922; }
  #status.err  { color: #f85149; }
  #hint { font-size: 0.75rem; color: #30363d; text-align: center; max-width: 360px; }
  #done {
    display: flex; flex-direction: column;
    align-items: center; justify-content: center;
    position: fixed; inset: 0;
    background: rgba(13,17,23,0.85);
    animation: fadein 0.3s ease;
  }
  #check {
    font-size: 40vmin;
    color: rgba(63,185,80,0.85);
    line-height: 1;
    animation: pop 0.4s cubic-bezier(0.34,1.56,0.64,1);
  }
  #done p { color: #3fb950; font-size: 1.4rem; margin-top: 1rem; letter-spacing: 0.05em; }
  @keyframes fadein { from{opacity:0} to{opacity:1} }
  @keyframes pop { from{transform:scale(0.3);opacity:0} to{transform:scale(1);opacity:1} }
</style>
</head>
<body>
<h1>TOFA</h1>
<div id="wrap">
  <video id="video" autoplay playsinline muted></video>
  <div id="overlay">
    <div id="finder">
      <span></span>
      <div id="scanline"></div>
    </div>
  </div>
</div>
<p id="status">Starting camera…</p>
<p id="hint">Point a QR code at the square above</p>

<script>
const video   = document.getElementById('video');
const status  = document.getElementById('status');
const hint    = document.getElementById('hint');

let steadyCount = 0;
let sent = false;

async function start() {
  try {
    const stream = await navigator.mediaDevices.getUserMedia({
      video: { facingMode: 'environment', width: { ideal: 1280 } }
    });
    video.srcObject = stream;
    await video.play();
    setStatus('', 'Scanning for QR code…', 'Point it at the square');
    tick();
  } catch(e) {
    setStatus('err', 'Camera access denied — allow it in your browser', '');
  }
}

function tick() {
  if (sent) return;

  // Prefer native BarcodeDetector (Chrome, Safari 17+)
  if ('BarcodeDetector' in window) {
    const bd = new BarcodeDetector({ formats: ['qr_code'] });
    bd.detect(video).then(codes => {
      if (codes.length > 0) {
        onFound(codes[0].rawValue);
      } else {
        setStatus('', 'Scanning…', 'Point a QR code at the square');
        steadyCount = 0;
      }
      requestAnimationFrame(tick);
    }).catch(() => requestAnimationFrame(tick));
  } else {
    // Fallback: draw frame on canvas and use jsQR
    if (!window._jsqrLoaded) {
      window._jsqrLoaded = true;
      const s = document.createElement('script');
      s.src = 'https://cdn.jsdelivr.net/npm/jsqr@1.4.0/dist/jsQR.min.js';
      s.onload = tick;
      document.head.appendChild(s);
      return;
    }
    if (typeof jsQR === 'undefined') { requestAnimationFrame(tick); return; }

    const canvas = document.createElement('canvas');
    canvas.width  = video.videoWidth;
    canvas.height = video.videoHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(video, 0, 0);
    const img = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const code = jsQR(img.data, img.width, img.height);
    if (code) {
      onFound(code.data);
    } else {
      setStatus('', 'Scanning…', 'Point a QR code at the square');
      steadyCount = 0;
      requestAnimationFrame(tick);
    }
  }
}

function onFound(value) {
  if (sent) return;
  steadyCount++;
  if (steadyCount < 3) {
    setStatus('warn', 'QR detected — hold steady…', 'Keep it still for a moment');
    requestAnimationFrame(tick);
    return;
  }
  sent = true;
  setStatus('ok', 'Got it! Adding to vault…', '');
  fetch('/result', { method: 'POST', body: value })
    .then(() => {
      document.body.innerHTML = `
        <div id="done">
          <div id="check">✓</div>
          <p>Added to vault</p>
        </div>`;
      setTimeout(() => window.close(), 2000);
    })
    .catch(() => {
      setStatus('err', 'Could not send to tofa', '');
    });
}

function setStatus(cls, msg, h) {
  status.className = cls;
  status.textContent = msg;
  hint.textContent = h;
}

start();
</script>
</body>
</html>"#.to_string()
}

pub fn run(args: CamArgs, vault_path: PathBuf) -> CliResult {
    let server = MiniServer::bind()?;
    let url = server.url();

    eprintln!("Opening browser… if it does not open automatically visit:");
    eprintln!("  {url}");

    // Open browser
    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(&url).spawn().ok();
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&url)
        .spawn()
        .ok();

    // Block until the browser posts the QR URI
    let uri = server.wait_for_result()?;
    if uri.is_empty() {
        return Err("No QR code received.".into());
    }

    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    if uri.starts_with("otpauth-migration://") {
        let accounts = tofa_core::qr::parse_migration(&uri)?;
        let count = accounts.len();
        let today = tofa_core::today_iso();
        for otp in accounts {
            let name = args.name.clone().unwrap_or_else(|| otp.meta.derive_name());
            vault.add_entry(otp.into_vault_entry(name, today.clone()));
        }
        vault.save(&vault_path, &pass)?;
        println!("Imported {count} account(s).");
        return Ok(());
    }

    let otp = tofa_core::qr::parse_input(&uri)?;
    let name = args.name.unwrap_or_else(|| otp.meta.derive_name());
    let today = tofa_core::today_iso();
    let entry = otp.into_vault_entry(name.clone(), today);
    let code = generate_code_now(&entry).unwrap_or_else(|_| "------".into());
    let secs = seconds_remaining_now(&entry);
    vault.add_entry(entry);
    vault.save(&vault_path, &pass)?;
    println!("Added {name}");
    println!("Current code: {}  ({}s)", format_code(&code), secs);
    Ok(())
}
