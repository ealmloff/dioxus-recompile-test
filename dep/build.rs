use std::io::Write;

// When this project is built, write the timestamp to the build_log file
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("build_log");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(dest_path)
        .unwrap();
    writeln!(file, "timestamp: {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()).unwrap();
}
