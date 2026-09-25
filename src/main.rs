use enigo::*;
use std::{thread::sleep, time::Duration};
use structopt::StructOpt;
use user_idle::UserIdle;

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    let mut enigo = Enigo::new(&Settings::default()).map_err(|error| error.to_string())?;

    loop {
        move_mouse(&mut enigo, &opt);
        sleep(Duration::from_secs(1));
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "afk",
    about = "A command-line program to make your mouse wander. ;)"
)]
struct Opt {
    /// Max delay time in seconds
    #[structopt(long = "delay", default_value = "28")]
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

