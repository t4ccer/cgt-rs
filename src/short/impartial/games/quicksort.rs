use std::fmt::Display;

use crate::numeric::nimber::Nimber;

#[derive(Debug, PartialEq, Eq)]
pub struct Quicksort(pub Vec<u32>);

impl Display for Quicksort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in &self.0 {
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

impl Quicksort {
    fn pick(&self, pivot: usize) -> Self {
        let mut res = Quicksort(Vec::with_capacity(self.0.len()));
        for elem in &self.0 {
            if *elem < self.0[pivot] {
                res.0.push(*elem);
            }
        }
        res.0.push(self.0[pivot]);
        for elem in &self.0 {
            if *elem > self.0[pivot] {
                res.0.push(*elem);
            }
        }
        res
    }

    pub fn moves(&self) -> Vec<Self> {
        let mut res = vec![];
        for pivot in 0..self.0.len() {
            let new = self.pick(pivot);
            if !res.contains(&new) && &new != self {
                res.push(new);
            }
        }
        res
    }

    pub fn game(&self) -> Nimber {
        let moves = self.moves();
        let mut game_moves = Vec::with_capacity(moves.len());
        for m in moves {
            game_moves.push(m.game());
        }

        Nimber::mex(game_moves)
    }
}