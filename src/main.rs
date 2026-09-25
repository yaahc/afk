use clap::Parser;
use enigo::*;
use std::{thread::sleep, time::Duration};
use user_idle::UserIdle;

fn main() -> Result<(), String> {
    let opt = Opt::parse();

    let mut enigo = Enigo::new(&Settings::default()).map_err(|error| error.to_string())?;

    loop {
        move_mouse(&mut enigo, &opt);
        sleep(Duration::from_secs(1));
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "afk",
    about = "A command-line program to make your mouse wander. ;)"
)]
struct Opt {
    /// Max delay time in seconds
    #[arg(long = "delay", default_value_t = 28)]
    delay: u64,
}

fn move_mouse(enigo: &mut Enigo, opt: &Opt){
    let idle_time = UserIdle::get_time()
        .expect("failed to get user idle time")
        .as_seconds();

    if idle_time > opt.delay {
        enigo
            .key(Key::Unicode('w'), Direction::Press)
            .expect("failed to send key");

        sleep(Duration::from_millis(500));

        enigo
            .key(Key::Unicode('w'), Direction::Release)
            .expect("failed to release key");
    }
}

