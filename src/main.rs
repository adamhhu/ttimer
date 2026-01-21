use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use regex::Regex;
use colored::*;

fn main() {
    let logo = r#"
    ---------------------------------------------------------------------------------
    @    ooooooooooo  ooooooooooo  ooooo  oooo     oooo  ooooooooooo  oooooooooo    @
    @    88  888  88  88  888  88   888    8888o   888    888    88    888    888   @
    @        888          888       888    88 888o8 88    888ooo8      888oooo88    @
    @        888          888       888    88  888  88    888    oo    888  88o     @
    @       o888o        o888o     o888o  o88o  8  o88o  o888ooo8888  o888o  88o8   @
    ---------------------------------------------------------------------------------       
    "#;

    println!("{}", logo.blue());
    println!("{}", "Try typing 50s, 10m or 2h. Type 'exit' to quit".green());

loop {
    print!("\n> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");

    let input = input.trim();

    if input == "exit" {
        println!("Goodbye!");
        break;
    }

    if input.is_empty() {
        continue;
    }
    
    match parse_time(input) {
      Some(seconds) => {
        println!("Starting timer: {}", input);
        run_timer(seconds);
        }
        
      None => {
        println!("Wrong format! Try again (Ex. 10s, 5m or 1h.)");
      }
    }
  }
}

fn parse_time(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+)([smh]?)").unwrap();
    let caps = re.captures(input)?;

    let value: u64 = caps[1].parse().ok()?;
    let unit = caps.get(2).map_or("s", |m| m.as_str());

    match unit {
        "s" => Some(value),
        "m" => Some(value * 60),
        "h" => Some(value * 3600),
        _ => None,
    }
}

fn run_timer(total_seconds: u64) {
    let bar_width = 20;

    for i in (1..=total_seconds).rev() {
        let h = i / 3600;
        let m = (i % 3600) / 60;
        let s = i % 60;
        
        let elapsed_percent = (total_seconds - i) as f64 / total_seconds as f64;
        let num_blocks = (elapsed_percent * bar_width as f64) as usize;

        let mut bar_string = String::new();

        for j in 0..num_blocks {
            let block_percent = j as f64 / bar_width as f64;

            let block = if block_percent < 0.3 {
                "█".blue()
            } else if block_percent < 0.6 {
                "█".green()
            } else if block_percent < 0.9 {
                "█".yellow()
            } else {
                "█".red()
            };
            bar_string.push_str(&block.to_string());
        }

        let spaces = "░".repeat(bar_width - num_blocks).truecolor(60, 60, 60);
        let percent_display = format!("{:>3.0}%", elapsed_percent * 100.0);

        print!("\r\x1B[K");
        println!("Time left: {:02}:{:02}:{:02}", h, m, s);

        print!("\r\x1B[K");
        print!("[{}{}] {}", bar_string, spaces, percent_display.bold());

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));

        if i > 1 {
            print!("\r\x1B[1A");
        }
    }
    print!("\r\x1B[K");
    let full_bar = "█".repeat(bar_width).red();
    println!("Time left: 00:00:00");
    println!("[{}] 100%", full_bar);
    println!("\n{}", "TIME IS UP!".red().bold());
}
