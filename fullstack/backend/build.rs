fn main() {
    // std::process::Command::new("trunk")
    //     .args(["serve", "--release"])
    //     .current_dir("../frontend")
    //     .output()
    //     .unwrap();
    std::fs::rename("../frontend/dist", "./static").unwrap();
}
