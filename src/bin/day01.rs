use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, bail, Context, Result};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

fn parse_u32(s: &str) -> Result<u32> {
    s.parse()
        .with_context(|| format!("Failed to parse u32: {}", s))
}

fn read_input(file: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    let file = File::open(file).with_context(|| format!("Failed to open input file: {}", file))?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().flatten().enumerate() {
        let mut nums = line.split_whitespace();

        let num1 = nums
            .next()
            .ok_or(anyhow!("No number for list 1 on line {}", i + 1))
            .and_then(|s| parse_u32(s))?;
        list1.push(num1);

        let num2 = nums
            .next()
            .ok_or(anyhow!("No number for list 2 on line {}", i + 1))
            .and_then(|s| parse_u32(s))?;
        list2.push(num2);
    }

    Ok((list1, list2))
}

fn distance(list1: &mut [u32], list2: &mut [u32]) -> Result<u32> {
    if list1.len() != list2.len() {
        bail!("Lists with different lengths")
    }

    rayon::join(|| list1.par_sort_unstable(), || list2.par_sort_unstable());

    let distance = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    Ok(distance)
}

fn similarity(list1: &[u32], list2: &[u32]) -> u32 {
    let mut list2_counts: HashMap<u32, u32> = HashMap::new();

    for ele in list2.iter().cloned() {
        list2_counts.entry(ele).and_modify(|x| *x += 1).or_insert(1);
    }

    list1
        .par_iter()
        .cloned()
        .map(|x| x * list2_counts.get(&x).unwrap_or(&0))
        .sum()
}

fn main() -> Result<()> {
    let input_file = "input/day01.txt";

    let (list1, list2) = read_input(input_file)
        .with_context(|| format!("Failed to read input from file: {}", input_file))?;

    let distance = distance(&mut list1.clone(), &mut list2.clone())
        .context("Failed to calculate distance of lists")?;

    println!("distance: {}", distance);

    let similarity = similarity(&list1, &list2);
    println!("similarity: {}", similarity);

    Ok(())
}
