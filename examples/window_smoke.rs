//! Live proof of the scap-vc fix: capture ONE frame from a cross-process
//! window (the operation that always failed in upstream scap 0.0.8).
//! Tries candidate windows until one delivers a frame (hidden/utility
//! windows never render, so each candidate gets a short timeout).
use std::sync::mpsc;
use std::time::Duration;

use scap_vc::capturer::{Capturer, Options};
use scap_vc::frame::{Frame, FrameType};
use scap_vc::Target;

fn try_window(window: scap_vc::Target) -> Option<(u32, u32, usize)> {
    let options = Options {
        fps: 10,
        target: Some(window),
        output_type: FrameType::BGRAFrame,
        show_cursor: false,
        ..Default::default()
    };
    let mut capturer = Capturer::build(options).ok()?;
    capturer.start_capture().ok()?;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        if let Ok(Frame::BGRA(f)) = capturer.get_next_frame() {
            let _ = tx.send((f.width as u32, f.height as u32, f.data.len()));
        }
        let _ = capturer.stop_capture();
    });
    rx.recv_timeout(Duration::from_secs(3)).ok()
}

fn main() {
    let targets = scap_vc::get_all_targets();
    let mut candidates: Vec<_> = targets
        .iter()
        .filter_map(|t| match t {
            Target::Window(w) if !w.title.is_empty() => Some(w),
            _ => None,
        })
        .collect();
    // Prefer windows that are likely visible & rendering (the YouTube tab).
    candidates.sort_by_key(|w| !(w.title.contains("Chrome") || w.title.contains("YouTube")));
    eprintln!("{} candidate windows", candidates.len());

    for w in candidates.into_iter().take(15) {
        eprintln!("trying id={} title={:?}", w.id, w.title);
        if let Some((width, height, bytes)) = try_window(Target::Window(w.clone())) {
            eprintln!(
                "FRAME OK: {}x{} ({} bytes) from {:?}",
                width, height, bytes, w.title
            );
            return;
        }
        eprintln!("  no frame (likely hidden); next");
    }
    eprintln!("FAILED: no candidate window delivered a frame");
    std::process::exit(1);
}
