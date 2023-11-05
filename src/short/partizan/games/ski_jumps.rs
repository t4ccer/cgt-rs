//! Ski Jumps game

use crate::{
    drawing::svg::{self, Svg},
    grid::{vec_grid::VecGrid, CharTile, FiniteGrid, Grid},
    short::partizan::{canonical_form::CanonicalForm, partizan_game::PartizanGame},
};
use std::{fmt::Display, str::FromStr};

/// Skier type
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Skier {
    /// Skier that can jump over skiers below
    Jumper,
    /// Skier that was jumped over tunrs into slipper and cannot jump anymore
    Slipper,
}

/// Ski Jumps game grid tile
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    /// Empty tile, without skiers
    Empty,

    /// Left player's skier
    Left(Skier),

    /// Right player's skier
    Right(Skier),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl CharTile for Tile {
    fn tile_to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Left(Skier::Jumper) => 'L',
            Tile::Left(Skier::Slipper) => 'l',
            Tile::Right(Skier::Jumper) => 'R',
            Tile::Right(Skier::Slipper) => 'r',
        }
    }

    fn char_to_tile(input: char) -> Option<Self> {
        match input {
            '.' => Some(Tile::Empty),
            'L' => Some(Tile::Left(Skier::Jumper)),
            'l' => Some(Tile::Left(Skier::Slipper)),
            'R' => Some(Tile::Right(Skier::Jumper)),
            'r' => Some(Tile::Right(Skier::Slipper)),
            _ => None,
        }
    }
}

// NOTE: Consider caching positions of left and right skiers to avoid quadratic loops
/// Ski Jumps game
#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkiJumps {
    grid: VecGrid<Tile>,
}

impl Display for SkiJumps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.display(f, '|')
    }
}

impl FromStr for SkiJumps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(VecGrid::parse(s).ok_or(())?))
    }
}

impl SkiJumps {
    /// Create new Ski Jumps game from a grid
    #[inline]
    pub fn new(grid: VecGrid<Tile>) -> Self {
        SkiJumps { grid }
    }

    /// Check if jumping move is possible
    pub fn jump_available(&self) -> bool {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                // Check if in a row below current row, there is a tile that can be jumped over
                let current = self.grid.get(x, y);
                for dx in 0..self.grid.width() {
                    if y + 1 < self.grid.height() {
                        match (current, self.grid.get(dx, y + 1)) {
                            (Tile::Left(Skier::Jumper), Tile::Right(_)) => {
                                return true;
                            }
                            (Tile::Right(Skier::Jumper), Tile::Left(_)) => {
                                return true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        false
    }

    /// Draw position as SVG image
    pub fn to_svg(&self) -> String {
        // Chosen arbitrarily
        let tile_size = 48;
        let grid_width = 4;

        let offset = grid_width / 2;
        let svg_width = self.grid.width() as u32 * tile_size + grid_width;
        let svg_height = self.grid.height() as u32 * tile_size + grid_width;

        let mut buf = String::new();
        Svg::new(&mut buf, svg_width, svg_width, |buf| {
            for y in 0..self.grid.height() {
                for x in 0..self.grid.width() {
                    match self.grid.get(x, y) {
                        Tile::Empty => {}
                        tile => {
                            let text = svg::Text {
                                x: (x as u32 * tile_size + offset + tile_size / 2) as i32,
                                y: (y as u32 * tile_size + offset + (0.6 * tile_size as f32) as u32)
                                    as i32,
                                text: tile.tile_to_char().to_string(),
                                text_anchor: svg::TextAnchor::Middle,
                                ..svg::Text::default()
                            };
                            Svg::text(buf, &text)?;
                        }
                    }
                }
            }

            Svg::g(buf, "black", |buf| {
                for y in 0..(self.grid.height() + 1) {
                    Svg::line(
                        buf,
                        0,
                        (y as u32 * tile_size + offset) as i32,
                        svg_width as i32,
                        (y as u32 * tile_size + offset) as i32,
                        grid_width,
                    )?;
                }

                for x in 0..(self.grid.width() + 1) {
                    Svg::line(
                        buf,
                        (x as u32 * tile_size + offset) as i32,
                        0,
                        (x as u32 * tile_size + offset) as i32,
                        svg_height as i32,
                        grid_width,
                    )?;
                }

                Ok(())
            })
        })
        .unwrap();

        buf
    }
}

impl PartizanGame for SkiJumps {
    fn left_moves(&self) -> Vec<Self> {
        let mut moves = vec![];

        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                match self.grid.get(x, y) {
                    Tile::Empty | Tile::Right(_) => {}
                    tile_to_move @ Tile::Left(skier) => {
                        // Check sliding moves
                        for dx in (x + 1)..=self.grid.width() {
                            if dx == self.grid.width() {
                                let mut new_grid = self.grid.clone();
                                new_grid.set(x, y, Tile::Empty);
                                moves.push(Self::new(new_grid));
                            } else if self.grid.get(dx, y) == Tile::Empty {
                                let mut new_grid = self.grid.clone();
                                new_grid.set(x, y, Tile::Empty);
                                new_grid.set(dx, y, tile_to_move);
                                moves.push(Self::new(new_grid));
                            } else {
                                // Blocked, cannot go any further
                                break;
                            }
                        }

                        // Check jump
                        if skier == Skier::Jumper && y + 1 < self.grid.height() {
                            match self.grid.get(x, y + 1) {
                                Tile::Empty | Tile::Left(_) => {}
                                Tile::Right(_) => {
                                    let mut new_grid = self.grid.clone();
                                    new_grid.set(x, y, Tile::Empty);
                                    new_grid.set(x, y + 1, Tile::Right(Skier::Slipper));
                                    if y + 2 < self.grid.height() {
                                        new_grid.set(x, y + 2, Tile::Left(Skier::Jumper));
                                    }
                                    moves.push(Self::new(new_grid));
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    fn right_moves(&self) -> Vec<Self> {
        let mut moves = vec![];

        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                match self.grid.get(x, y) {
                    Tile::Empty | Tile::Left(_) => {}
                    tile_to_move @ Tile::Right(skier) => {
                        // Check sliding moves
                        for dx in (0..x + 1).rev() {
                            // We're iterating with 1 off to avoid using negative numbers but still
                            // catch going off grid, so the `dx - 1` hack.

                            if dx == 0 {
                                let mut new_grid = self.grid.clone();
                                new_grid.set(x, y, Tile::Empty);
                                moves.push(Self::new(new_grid));
                            } else if self.grid.get(dx - 1, y) == Tile::Empty {
                                let mut new_grid = self.grid.clone();
                                new_grid.set(x, y, Tile::Empty);
                                new_grid.set(dx - 1, y, tile_to_move);
                                moves.push(Self::new(new_grid));
                            } else {
                                // Blocked, cannot go any further
                                break;
                            }
                        }

                        // Check jump
                        if skier == Skier::Jumper && y + 1 < self.grid.height() {
                            match self.grid.get(x, y + 1) {
                                Tile::Empty | Tile::Right(_) => {}
                                Tile::Left(_) => {
                                    let mut new_grid = self.grid.clone();
                                    new_grid.set(x, y, Tile::Empty);
                                    new_grid.set(x, y + 1, Tile::Left(Skier::Slipper));
                                    if y + 2 < self.grid.height() {
                                        new_grid.set(x, y + 2, Tile::Right(Skier::Jumper));
                                    }
                                    moves.push(Self::new(new_grid));
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    fn reductions(&self) -> Option<CanonicalForm> {
        // If neither player can jump, the optimal move is to move any of the pieces by one tile
        // so the game value is the difference of sum of distances to the board edge
        if !self.jump_available() {
            let mut value = 0i64;
            for y in 0..self.grid.height() {
                for x in 0..self.grid.width() {
                    match self.grid.get(x, y) {
                        Tile::Empty => {}
                        Tile::Left(_) => value += self.grid.width() as i64 - x as i64,
                        Tile::Right(_) => value -= (x + 1) as i64,
                    }
                }
            }
            return Some(CanonicalForm::new_integer(value));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::short::partizan::transposition_table::TranspositionTable;

    macro_rules! test_canonical_form {
        ($input:expr, $output:expr) => {{
            let tt = TranspositionTable::new();
            let pos = SkiJumps::from_str($input).expect("Could not parse the game");
            let cf = pos.canonical_form(&tt);
            assert_eq!(cf.to_string(), $output)
        }};
    }

    #[test]
    fn winning_ways_examples() {
        // I couldn't find other implementations so we're comparing against positions in winning ways
        test_canonical_form!("...L....|..R.....|........", "2");
        test_canonical_form!("........|...l....|.......R|........|......L.", "-1");
        test_canonical_form!(".L...|.R...|.....", "5/2");
        test_canonical_form!("...R.|...L.|.....", "-5/2");
        test_canonical_form!("L....|....R|.....", "1/2");
    }
}