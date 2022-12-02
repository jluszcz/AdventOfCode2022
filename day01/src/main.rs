use anyhow::{anyhow, Result};
use log::{debug, info, trace, LevelFilter};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Debug)]
struct Elf {
    id: u32,
    calories: u32,
}

impl Elf {
    fn new(id: u32) -> Self {
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
        Some(self.calories.cmp(&other.calories))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

fn load_elves() -> Result<BinaryHeap<Elf>> {
    let mut elves = BinaryHeap::new();
    let mut curr_elf = None;

    let mut id = 1;
    for line in utils::input()? {
        trace!("{}", line);

        if curr_elf.is_none() {
            curr_elf = Some(Elf::new(id));
            id += 1;
        } else if line.is_empty() {
            let elf = curr_elf.take().unwrap();
            debug!("Adding {:?} to heap", elf);
            elves.push(elf);
        }

        if let Some(elf) = &mut curr_elf {
            elf.calories += u32::from_str(&line)?;
        }
    }

    if let Some(elf) = curr_elf.take() {
        debug!("Adding {:?} to heap", elf);
        elves.push(elf);
    }

    Ok(elves)
}

fn main() -> Result<()> {
    utils::init_logger(LevelFilter::Info)?;

    let mut elves = load_elves()?;
    debug!("Elves: {:?}", elves);

    let top_k = 3;
    let mut top_k_calories = 0;
    for _ in 0..top_k {
        let elf = elves.pop().ok_or_else(|| anyhow!("No Elf found"))?;
        debug!("{:?}", elf);
        top_k_calories += elf.calories;
    }

    info!("Calories: {}", top_k_calories);

    Ok(())
}
