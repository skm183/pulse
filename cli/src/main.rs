use clap::{Parser,Subcommand};
use core_logic::{Timer, Event};
use std::sync::mpsc;
use std::time::{SystemTime, Duration};
use std::process::Command;
use std::io::{self, Write};
use std::thread;

#[derive(Parser)]
#[command(name = "pulse", about = "pulse description")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long, default_value_t = 20)]
        eye: u64,
        #[arg(short, long, default_value_t=30)]
        water: u64
    }
}

fn format_remaining(end_time: SystemTime) -> String {
    match end_time.duration_since(SystemTime::now()) {
        Ok(duration) => format!("{:02}:{:02}", duration.as_secs()/60, duration.as_secs()%60),
        Err(_) => "--:--".to_string()
    }
}

fn notify(title: &str, body: &str, urgency: &str) {
    let _ = Command::new("notify-send")
        .args([title, body, "-u", urgency, "-a", "Pulse"])
        .spawn();
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Start { eye, water } => {
            println!("</> Pulse INITITATED");
            println!("    Eyes: {eye}m | Water: {water}m");
            println!("    Press Ctrl+C to stop,\n");

            let (tx, rx) = mpsc::channel();

            let timer = Timer::new(eye, water, tx);
            timer.start();

            let mut eye_end: Option<SystemTime> = None;
            let mut water_end: Option<SystemTime> = None;

            loop {
                while let Ok(event) = rx.try_recv() {
                    match event {
                        Event::TaskStart { name, end_time } => {
                            if name == "Eyes" {eye_end = Some(end_time); }
                            if name == "Water" {water_end = Some(end_time);}
                        },
                        Event::TaskEnd { name } => {
                            println!("\n {} ALERT", name.to_uppercase());

                            if name == "Eyes" {
                                eye_end = None;
                                notify("Eye Strain Alert", "Time for some eye exercises", "critical");
                            }
                            if name == "Water" {
                                water_end = None;
                                notify("Hydration Alert", "Drink some water", "normal");
                            }
                        }

                    }

                    
                    
                };
                let eye_str = match eye_end {
                    Some(t) => format_remaining(t),
                    None => "WAITING".to_string()
                };
                let water_str = match water_end {
                    Some(t) => format_remaining(t),
                    None => "WAITING".to_string()
                };
                print!("\r👁️ Eyes: {eye_str}   |   💧 Water: {water_str}   \x1b[K");
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}