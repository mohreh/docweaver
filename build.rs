use std::process::Command;

fn main() {
    Command::new("tailwindcss")
        .args([
            "-i",
            "./templates/assets/main.scss",
            "-o",
            "./assets/styles/main.css",
            "-m",
        ])
        .status()
        .unwrap();
}
