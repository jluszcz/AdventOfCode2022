use anyhow::Result;
use log::{debug, info, trace};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Debug)]
struct Elf {
    id: usize,
    calories: usize,
}

impl Elf {
    fn new(id: usize) -> Self {
        Elf { id, calories: 0 }
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Elf {}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.calories.cmp(&self.calories))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        other.calories.cmp(&self.calories)
    }
}

fn try_add_elf(elf: &mut Option<Elf>, top_elves: &mut BinaryHeap<Elf>, k: usize) {
    if let Some(elf) = elf.take() {
        if top_elves.len() == k {
            let top_elf = top_elves.peek().unwrap();
            if top_elf > &elf {
                debug!("Replacing {:?} with {:?}", top_elf, elf);
                top_elves.pop();
                top_elves.push(elf);
            }
        } else {
            top_elves.push(elf);
        }
    }
}

fn top_k_elves(k: usize, input: &[String]) -> Result<BinaryHeap<Elf>> {
    let mut elves = BinaryHeap::new();
    let mut curr_elf = None;

    let mut id = 1;
    for line in input {
        trace!("{}", line);

        if curr_elf.is_none() {
            curr_elf = Some(Elf::new(id));
            id += 1;
        } else if line.is_empty() {
            try_add_elf(&mut curr_elf, &mut elves, k);
        }

        if let Some(elf) = &mut curr_elf {
            elf.calories += usize::from_str(&line)?;
        }
    }

    try_add_elf(&mut curr_elf, &mut elves, k);

    Ok(elves)
}

fn main() -> Result<()> {
    let input = utils::input()?;

    let k = 3;

    let elves = top_k_elves(k, &input)?;
    debug!("Top {} Elves: {:?}", k, elves);

    let calories: usize = elves.into_iter().map(|e| e.calories).sum();

    info!("Calories: {}", calories);

    Ok(())
}
