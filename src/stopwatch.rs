use std::thread;
use std::time::Duration;

pub fn clock() {
    let mut hours: i8 = 0;
    let mut minutes: i8 = 0;
    let mut seconds: i8 = 0;
    let mut centiseconds: i8 = 0;

    let one_centisecond = Duration::from_millis(10);

    loop {
        display_time(hours, minutes, seconds, centiseconds);
        centiseconds += 1;
        if centiseconds > 99 {
            centiseconds = 0;
            seconds += 1;
        }
        if seconds > 59 {
            seconds = 0;
            minutes += 1;
        }
        if minutes > 59 {
            seconds = 0;
            minutes = 0;
            hours += 1;
        }
        thread::sleep(one_centisecond);
    }
}

fn display_time(hours: i8, minutes: i8, seconds: i8, centiseconds: i8) {
    std::process::Command::new("clear").status().unwrap();
    println!(
        "{:02}:{:02}:{:02}.{:02}",
        hours, minutes, seconds, centiseconds
    );
}
