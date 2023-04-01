use clearscreen::clear;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, stdout, Write},
    thread, time,
};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    clear_screen();
    type_out("No chance you will guess the number...");
    type_out("Enter your (bad) guess: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Your PC is scuffed mate, I cant read the line...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                type_out("This is not a number...");
                type_out("Dumbass...");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => type_out("HAH, too small! Exactly like your reproductive organ LOL"),
            Ordering::Greater => type_out("Go lower man, don't flatter yourself"),
            Ordering::Equal => {
                type_out("You actually did it, I can't believe it...");
                sleep(1000);
                type_out("SEE YOU IN HELL. IM GONNA SELF DESTRUCT");
                break;
            }
        }
    }
}

fn sleep(ms: u64) {
    let duration = time::Duration::from_millis(ms);
    thread::sleep(duration)
}

fn type_out(text: &str) {
    for char in text.split("") {
        let rand_dur = rand::thread_rng().gen_range(10..=50);
        print!("{char}");

        match stdout().flush() {
            Err(_) => break,
            Ok(_) => sleep(rand_dur),
        };
    }

    println!("");
}

fn clear_screen() {
    clear().expect("Failed to clear the screen");
}
