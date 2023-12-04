use inline_colorization::*;
use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("{color_green}Meow");
    sleep(Duration::from_millis(800)).await;
    println!("{color_blue}Meow");
    sleep(Duration::from_millis(800)).await;
    println!("{color_magenta}I'm a cow");
    sleep(Duration::from_millis(800)).await;
    println!("{color_green}Meow");
    sleep(Duration::from_millis(300)).await;
    println!("{color_blue}Meow");
    sleep(Duration::from_millis(300)).await;
    println!("{color_magenta}I'm a cow");
    sleep(Duration::from_millis(600)).await;
    if let Err(error) = open::that("https://www.youtube.com/watch?v=CZlfbep2LdU") {
        println!("{color_red}Can't open URL: {}", error.to_string());
    }
    println!("{color_white}");
}
