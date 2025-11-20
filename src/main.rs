use std::process::Command;
use std::thread;
use std::time::Duration;

use enigo::{Enigo, Key, Keyboard, Settings,
    Direction::{Click, Press, Release}
};

fn main() {
  const CHANNEL_NAME: &str = "metaverse3";
  const MESSAGE: &str = "$collect";

  loop {
    launch_discord();
    thread::sleep(Duration::from_millis(1000));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Ctrl + K
    let _ = enigo.key(Key::Control, Press);
    let _ = enigo.key(Key::Unicode('k'), Click);
    let _ = enigo.key(Key::Control, Release);
    thread::sleep(Duration::from_millis(500));

    // Type channel name and enter
    let _ = enigo.text(CHANNEL_NAME);
    let _ = enigo.key(Key::Return, Press);
    thread::sleep(Duration::from_millis(500));

    // Type message and enter
    let _ = enigo.text(MESSAGE);
    // enigo.key(Key::Return, Press);
    thread::sleep(Duration::from_millis(500));

    // Wait 2 hours
    thread::sleep(Duration::from_secs(7200));
  }
}

fn launch_discord() {
  Command::new("open")
    .arg("/Applications/Discord.app")
    .spawn()
    .expect("Failed to launch Discord");
}

