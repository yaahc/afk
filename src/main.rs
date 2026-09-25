use enigo::*;
use rand::prelude::*;
use std::{thread::sleep, time::Duration};
use structopt::StructOpt;

fn main() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|error| error.to_string())?;

    let opt = Opt::from_args();

    let mut rng = rand::thread_rng();

    loop {
        if let Err(message) = move_mouse(&mut enigo, &opt, &mut rng) {
            return Err(format!("{}. Exiting gracefully.", message));
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "afk",
    about = "A command-line program to make your mouse wander. ;)"
)]
struct Opt {
    /// Max delay time in seconds
    #[structopt(long = "max-delay", default_value = "30")]
    max_delay: u64,

    /// Min delay time in seconds
    #[structopt(long = "min-delay", default_value = "5")]
    min_delay: u64,
}

fn move_mouse(enigo: &mut Enigo, opt: &Opt, rng: &mut ThreadRng) -> Result<(), &'static str> {
    if opt.min_delay > opt.max_delay {
        return Err("min-delay is greater than max-delay");
    }

    enigo
        .key(Key::Unicode('w'), Direction::Press)
        .map_err(|_| "failed to send key")?;

    let sleep_time = rng.gen_range(opt.min_delay..=opt.max_delay);

    println!("Will move again in {} seconds.", sleep_time);

    sleep(Duration::from_secs(sleep_time));

    Ok(())
}

