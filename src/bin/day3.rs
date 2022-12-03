#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::fmt::Debug;
use std::io::BufRead;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let stdin = std::io::stdin()
        .lock()
        .lines();

    let lines: Vec<_> = stdin
        .map(|line| line.unwrap())
        .collect();


    let first = first(lines.iter());
    let second = second(lines.iter());

    println!("first: {first}");
    println!("second: {second}");

    Ok(())
}

fn first<I, S>(lines: I) -> u32
    where I: Iterator<Item=S>,
          S: AsRef<str>,
{
    lines
        .map(|line| find_duplicate(line.as_ref()))
        .map(to_priority)
        // why not just a sum()? because the trait only works for summing up to the same type
        // we want to sum up to u32 from u8, which won't work, go try it for yourself and tear
        // your hair out
        .fold(0_u32, |mut accum, curr| {
            accum += curr as u32;
            accum
        })
}


fn second<I, S>(lines: I) -> u32
    where I: Iterator<Item=S>,
          S: AsRef<str>,

{
    // sadly chunking for iterators is unstable
    lines
        .array_chunks::<3>()
        .map(|chunk| {
            let intersection = chunk
                .iter()
                .fold(HashSet::from_iter("abcdefghijklmnopqrstuvwyzABCDEFGHIJKLMNOPQRSTUVWYZ".chars()), |mut accum, curr| {
                    let curr: HashSet<char> = HashSet::from_iter(curr.as_ref().chars());
                    accum = &accum & &curr;
                    accum
                });

            assert!(intersection.len() == 1);
            intersection.into_iter().next().unwrap()
        })
        .map(to_priority)
        .fold(0_u32, |mut accum, curr| {
            accum += curr as u32;
            accum
        })
}

fn find_duplicate(line: &str) -> char {
    let (first, second) = line.split_at(line.len() / 2);

    let first: HashSet<_> = first.chars().collect();
    let second: HashSet<_> = second.chars().collect();

    let mut intersection = first.intersection(&second);

    *intersection.next().unwrap()
}


fn to_priority(c: char) -> u8 {
    let ascii = c as u8;

    if c.is_ascii_uppercase() {
        ascii - b'A' + 27
    } else {
        ascii - b'a' + 1
    }
}
