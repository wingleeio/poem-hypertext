use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");

    Command::new("npx")
        .args([
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "src/style.css",
            "-o",
            "public/style.css",
            "--minify",
        ])
        .status()
        .unwrap();
}
