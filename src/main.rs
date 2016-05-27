use chrono::Utc;
use std::{process, thread, time};

fn main() {
    thread::spawn(|| loop {
        let _ = process::Command::new("raspistill")
            .args(&[
                "-t",
                "1000",
                "-ss",
                "10000",
                "--awb",
                "greyworld",
                "-o",
                &format!("/data/{}.jpg", Utc::now().to_rfc3339()),
            ])
            .status();
        thread::sleep(time::Duration::from_secs(60));
    });

    loop {
        thread::sleep(time::Duration::from_secs(60));
    }
}
