use std::str::FromStr;

use anyhow::{anyhow, Result};
use log::{debug, info};

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn score(&self) -> usize {
        match self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        }
    }

    fn shape_for_outcome(&self, shape: &Shape) -> Shape {
        match (self, shape) {
            (GameResult::Win, Shape::Rock) => Shape::Paper,
            (GameResult::Win, Shape::Paper) => Shape::Scissors,
            (GameResult::Win, Shape::Scissors) => Shape::Rock,
            (GameResult::Lose, Shape::Rock) => Shape::Scissors,
            (GameResult::Lose, Shape::Paper) => Shape::Rock,
            (GameResult::Lose, Shape::Scissors) => Shape::Paper,
            (GameResult::Draw, _) => *shape,
        }
    }
}

impl FromStr for GameResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow!("Invalid ID: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn play(&self, other: &Shape) -> usize {
        let result = match (self, other) {
            (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Rock, Shape::Scissors) => GameResult::Win,

            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => GameResult::Lose,

            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => GameResult::Draw,
        };

        let score = result.score() + self.score();
        debug!("{:?} vs {:?}: {}", self, other, score);

        score
    }

    fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(anyhow!("Invalid ID: {}", s)),
        }
    }
}

fn follow_strategy_guide(input: &[String]) -> Result<usize> {
    let mut score = 0;

    for game in input {
        let mut parts = game.split_ascii_whitespace();
        let opponent_play = Shape::from_str(
            parts
                .next()
                .ok_or_else(|| anyhow!("Invalid game: {}", game))?,
        )?;

        let outcome = GameResult::from_str(
            parts
                .next()
                .ok_or_else(|| anyhow!("Invalid game: {}", game))?,
        )?;

        let self_play = outcome.shape_for_outcome(&opponent_play);

        score += self_play.play(&opponent_play);
    }

    Ok(score)
}

fn main() -> Result<()> {
    let input = utils::input()?;

    let score = follow_strategy_guide(&input)?;
    info!("Score: {}", score);

    Ok(())
}
