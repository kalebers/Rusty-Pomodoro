use std::{thread, time::Duration, io::{self, Write}};

fn main() {
    let work_minutes = 25;
    let break_minutes = 5;
    let animation_chars = ['|', '/', '-', '\\'];

    println!("Pomodoro Timer - Work: {} min, Break: {} min", work_minutes, break_minutes);
    
    loop {
        run_timer(work_minutes, "Work", &animation_chars);
        run_timer(break_minutes, "Break", &animation_chars);
    }
}

fn run_timer(minutes: u64, phase: &str, animation_chars: &[char]) {
    let seconds = minutes * 60;
    let mut counter = 0;

    println!("\n{} phase started!", phase);

    for second in 0..seconds {
        let char_index = (second % animation_chars.len() as u64) as usize;
        print!("\r{} [{}] Time left: {:02}:{:02}", phase, animation_chars[char_index], (seconds - second) / 60, (seconds - second) % 60);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        counter += 1;
    }

    println!("\n{} phase completed!", phase);
}
