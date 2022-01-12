use rand::Rng;
use std::{thread, time};
fn main() {
    println!("Rolling the dice...");
    let mut rng = rand::thread_rng();
    let dice_roll = rng.gen_range(1..6);
    thread::sleep(time::Duration::from_secs(2));
    if dice_roll == 1 {
        println!("__________\
                  \n|         |\
                  \n|    O    |\
                  \n|         |\
                  \n|_________|");
    } else if dice_roll == 2 {
        println!("__________\
                  \n|         |\
                  \n| O       |\
                  \n|         |\
                  \n|       O |\
                  \n|_________|");
    } else if dice_roll == 3 {
        println!("__________\
                  \n|         |\
                  \n| O       |\
                  \n|    O    |\
                  \n|      O  |\
                  \n|_________|");
    } else if  dice_roll == 4{
        println!("__________\
                  \n|         |\
                  \n| O   O   |\
                  \n|         |\
                  \n|   O   O |\
                  \n|_________|");
    } else if dice_roll == 5 {
        println!("__________\
                  \n|         |\
                  \n| O   O   |\
                  \n|    O    |\
                  \n|   O   O |\
                  \n|_________|");
    } else if dice_roll == 6 {
        println!("__________\
                  \n|         |\
                  \n| O   O   |\
                  \n|   O   O |\
                  \n|   O   O |\
                  \n|_________|");
    }
    println!("You rolled a {}", dice_roll);
}
