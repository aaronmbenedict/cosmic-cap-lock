// src/keyboard.rs
// Caps Lock detection — no extra crate dependencies.
//
// Priority:
//   1. /sys/class/leds/*capslock*/brightness  (works Wayland + X11)
//   2. `xset q` stdout parsing                (X11 fallback)
//   3. false                                  (safe default)

pub fn query_caps_lock() -> bool {
    sysfs_led().or_else(|| xset_query()).unwrap_or(false)
}

fn sysfs_led() -> Option<bool> {
    let entries = std::fs::read_dir("/sys/class/leds").ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name.contains("capslock") || name.contains("caps_lock") {
            let val: u32 = std::fs::read_to_string(entry.path().join("brightness"))
                .ok()?
                .trim()
                .parse()
                .ok()?;
            return Some(val > 0);
        }
    }
    None
}

fn xset_query() -> Option<bool> {
    std::env::var("DISPLAY").ok()?; // only attempt on X11
    let out = std::process::Command::new("xset").arg("q").output().ok()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout.lines() {
        if line.contains("Caps Lock") {
            return Some(line.contains("Caps Lock:   on"));
        }
    }
    None
}
