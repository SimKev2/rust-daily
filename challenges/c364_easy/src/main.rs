extern crate rand;

use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    // let dice_lines = read_file("inputs/example_input.txt");
    let dice_lines = read_file("inputs/challenge_input.txt");

    for roll in dice_lines {
        let dice_vec: Vec<&str> = roll.split('d').collect();
        let num_dice: u8 = dice_vec[0].parse::<u8>().expect("could not convert to u8");
        let num_sides: u8 = dice_vec[1].parse::<u8>().expect("could not convert to u8");

        let mut roll_total: u32 = 0;
        let mut rolls: Vec<u8> = Vec::new();

        for _ in 0..num_dice {
            let roll = dice_roll(num_sides);
            roll_total += roll as u32;
            rolls.push(roll);
        }

        println!("{:?}: {:?}", roll_total, rolls);
    }
}

fn read_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("file does not exist.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("could not read line"))
        .collect()
}

fn dice_roll(sides: u8) -> u8 {
    if sides == 0 {
        return 0;
    }

    let mut rng = thread_rng();
    rng.gen_range(0, sides)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let num: u8 = dice_roll(0);
        assert_eq!(0, num);
        let num: u8 = dice_roll(1);
        assert_eq!(0, num);
        let num: u8 = dice_roll(2);
        assert!(num <= 2);
    }
}
