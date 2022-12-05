use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;
use log::{info, trace};

#[derive(Debug, Default)]
struct Rucksack {
    items: HashSet<char>,
    compartments: [HashSet<char>; 2],
}

impl Rucksack {
    fn _common_item_priority(&self) -> u32 {
        let intersection = self.compartments[0]
            .intersection(&self.compartments[1])
            .collect::<Vec<_>>();

        assert_eq!(1, intersection.len());

        let common_item = *intersection.first().cloned().unwrap();
        Self::item_priority(common_item)
    }

    fn item_priority(c: char) -> u32 {
        let p = c as u32;
        let p = if c.is_uppercase() {
            27 + p - ('A' as u32)
        } else {
            p - ('a' as u32 - 1)
        };

        trace!("{}: {}", c, p);
        p
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = Rucksack::default();

        let (first, second) = s.split_at(s.len() / 2);

        for c in first.chars() {
            r.items.insert(c);
            r.compartments[0].insert(c);
        }

        for c in second.chars() {
            r.items.insert(c);
            r.compartments[1].insert(c);
        }

        trace!("{:?}", r);
        Ok(r)
    }
}

fn get_rucksacks(input: &[String]) -> Vec<Rucksack> {
    input
        .iter()
        .map(|l| Rucksack::from_str(l).unwrap())
        .collect()
}

fn main() -> Result<()> {
    let rucksacks = get_rucksacks(&utils::input()?);
    assert_eq!(0, rucksacks.len() % 3,);

    let mut priority = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let mut intersection = rucksacks.get(i).unwrap().items.clone();
        for j in i + 1..i + 3 {
            intersection = intersection
                .intersection(&rucksacks.get(j).unwrap().items)
                .cloned()
                .collect::<HashSet<_>>();
        }

        assert_eq!(1, intersection.len());
        priority += Rucksack::item_priority(*intersection.iter().next().unwrap());
    }

    info!("Priority: {}", priority);

    Ok(())
}
