use std::{process::Command, thread, time::Duration};

fn main()
{
    let mut child: std::process::Child = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
        .arg("--remote-debugging-port=9222")
        .spawn()
        .expect("Failed to start Chrome");
    thread::sleep(Duration::from_secs(10));
    child.kill();
}
