use colored::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            // ANSI escape codes were successfully enabled, or this is a non-Windows platform.
            // Use your terminal color library of choice here.
            println!("{}", "Meow".green());
            sleep(Duration::from_millis(800)).await;
            println!("{}", "Meow".blue());
            sleep(Duration::from_millis(800)).await;
            println!("{}", "I'm a cow".magenta());
            sleep(Duration::from_millis(800)).await;
            println!("{}", "Meow".green());
            sleep(Duration::from_millis(300)).await;
            println!("{}", "Meow".blue());
            sleep(Duration::from_millis(300)).await;
            println!("{}", "I'm a cow".magenta());
            sleep(Duration::from_millis(600)).await;
            if let Err(error) = open::that("https://www.youtube.com/watch?v=CZlfbep2LdU") {
                println!(
                    "{}: {}",
                    "Can't open URL".red(),
                    error.to_string().bright_red()
                );
            }
        }
        Err(_) => {
            // The operation was unsuccessful, typically because it's running on an older
            // version of Windows. The program may choose to disable ANSI color code output in
            // this case.
            println!("Meow");
            sleep(Duration::from_millis(800)).await;
            println!("Meow");
            sleep(Duration::from_millis(800)).await;
            println!("I'm a cow");
            sleep(Duration::from_millis(800)).await;
            println!("Meow");
            sleep(Duration::from_millis(300)).await;
            println!("Meow");
            sleep(Duration::from_millis(300)).await;
            println!("I'm a cow");
            sleep(Duration::from_millis(600)).await;
            if let Err(error) = open::that("https://www.youtube.com/watch?v=CZlfbep2LdU") {
                println!("Can't open URL: {}", error.to_string());
            }
        }
    }
}
