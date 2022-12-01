use std::{env};
use aoc_2022::io as our_io;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filename = env::args()
        .nth(1)
        .expect("No filename given");
    let input = our_io::read_lines(filename)?
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut energies: Vec<_> = input
        // every every line denotes a new group
        .split(|line| line.is_empty())
        // sum up the energy of each group
        .map(|group| {
            group
                .iter()
                .fold(0, |mut accumulator, curr| {
                    accumulator += curr.parse::<u32>().unwrap();
                    accumulator
                })
        })
        .collect();

    // print the max energy
    println!("max energy {}", energies.iter().max().unwrap());

    // print the sum of the top 3 energies by descending order
    energies.sort_unstable();
    energies.reverse();
    println!("sum of max 3 energies {}", energies.as_slice()[0..3].iter().sum::<u32>());

    Ok(())
}
