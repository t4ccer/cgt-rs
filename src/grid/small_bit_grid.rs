//! Grid with up to 64 tiles holding a single bit of information.

use crate::grid::{FiniteGrid, Grid};
use std::{fmt::Display, str::FromStr};

/// Internal representation of a grid
type GridBits = u64;

/// A grid with up to 64 tiles holding a single bit of information.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SmallBitGrid {
    width: u8,
    height: u8,
    grid: GridBits,
}

impl Grid for SmallBitGrid {
    type Item = bool;

    fn get(&self, x: u8, y: u8) -> Self::Item {
        let n = self.width as GridBits * y as GridBits + x as GridBits;
        (self.grid >> n) & 1 == 1
    }

    fn set(&mut self, x: u8, y: u8, value: Self::Item) {
        let val = value as GridBits;
        let n = self.width as GridBits * y as GridBits + x as GridBits;
        self.grid = (self.grid & !(1 << n)) | (val << n);
    }
}

impl FiniteGrid for SmallBitGrid {
    fn width(&self) -> u8 {
        self.width
    }

    fn height(&self) -> u8 {
        self.height
    }
}

impl Display for SmallBitGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let chr = if self.get(x, y) { '#' } else { '.' };
                write!(f, "{}", chr)?;
            }
            if y != self.height - 1 {
                write!(f, "|")?;
            }
        }
        Ok(())
    }
}

impl SmallBitGrid {
    /// Check if dimensions are small enough to fit in the fixed-size bit representation.
    const fn check_dimensions(width: u8, height: u8) -> Option<()> {
        if (width as usize * height as usize) > 8 * std::mem::size_of::<GridBits>() {
            return None;
        }
        Some(())
    }

    /// Creates empty grid with given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use cgt::grid::small_bit_grid::SmallBitGrid;
    ///
    /// assert_eq!(&format!("{}", SmallBitGrid::empty(2, 3).unwrap()), "..|..|..");
    /// ```
    ///
    /// # Errors
    /// - Grid has more than 64 tiles
    pub fn empty(width: u8, height: u8) -> Option<Self> {
        Self::check_dimensions(width, height)?;

        Some(Self {
            width,
            height,
            grid: 0,
        })
    }

    /// Creates empty grid of zero size
    #[must_use]
    pub const fn zero_size() -> Self {
        Self {
            width: 0,
            height: 0,
            grid: 0,
        }
    }

    /// Creates filled grid with given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use cgt::grid::small_bit_grid::SmallBitGrid;
    ///
    /// assert_eq!(&format!("{}", SmallBitGrid::filled(3, 2).unwrap()), "###|###");
    /// ```
    ///
    /// # Errors
    /// - Grid has more than 64 tiles
    pub fn filled(width: u8, height: u8) -> Option<Self> {
        Self::check_dimensions(width, height)?;

        Some(Self {
            width,
            height,
            grid: GridBits::MAX,
        })
    }

    /// Create a grid that correspondes to given size and "internal id".
    ///
    /// # Arguments
    ///
    /// `grid_id` - A number that represents empty and taken grid tiles. Starting from left and the
    /// lowest bit, if bit is 1 then tile is filled, otherwise the tile is empty.
    /// Bits outside grid size are ignored
    ///
    /// # Examples
    ///
    /// ```
    /// use cgt::grid::small_bit_grid::SmallBitGrid;
    ///
    /// assert_eq!(&format!("{}", SmallBitGrid::from_number(3, 2, 0b101110).unwrap()), ".##|#.#");
    /// ```
    ///
    /// # Errors
    /// - Grid has more than 64 tiles
    pub fn from_number(width: u8, height: u8, grid_id: GridBits) -> Option<Self> {
        Self::check_dimensions(width, height)?;
        Some(Self {
            width,
            height,
            grid: grid_id,
        })
    }

    /// Creates a grid from given array of bools.
    ///
    /// # Arguments
    ///
    /// * `grid` - Lineralized grid of size `width * height`, empty if if value is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cgt::grid::small_bit_grid::SmallBitGrid;
    ///
    /// SmallBitGrid::from_arr(2, 3, &[true, true, false, false, false, true]).unwrap();
    /// ```
    ///
    /// # Errors
    /// - Grid has more than 64 tiles
    pub fn from_arr(width: u8, height: u8, grid: &[bool]) -> Option<Self> {
        Self::from_number(width, height, arr_to_bits(grid))
    }

    // NOTE: I'm not sure if that's a good place, or is it too domineering-specific
    /// Parses a grid from `.#` notation.
    ///
    /// # Arguments
    ///
    /// * `input` - `.#` notation with `|` as rows separator
    ///
    /// # Examples
    ///
    /// ```
    /// use cgt::grid::small_bit_grid::SmallBitGrid;
    /// use std::str::FromStr;
    ///
    /// SmallBitGrid::from_str("..#|.#.|##.").unwrap();
    /// ```
    ///
    /// # Errors
    /// - Grid has more than 64 tiles
    /// - Input is in invalid format
    pub fn parse(input: &str) -> Option<Self> {
        // number of chars till first '|' or eof is the width
        // number of '|' + 1 is the height
        let width = input.split('|').next()?.len() as u8;
        let height = input.chars().filter(|c| *c == '|').count() as u8 + 1;

        let mut grid = Self::empty(width, height)?;
        let mut x = 0;
        let mut y = 0;

        for chr in input.chars() {
            if chr == '|' {
                if x == width {
                    x = 0;
                    y += 1;
                    continue;
                }
                // Not a rectangle
                return None;
            }
            grid.set(
                x,
                y,
                match chr {
                    '.' => false,
                    '#' => true,
                    _ => return None,
                },
            );
            x += 1;
        }

        if x != width {
            // Not a rectangle in the last row
            return None;
        }
        Some(grid)
    }

    /// Rotate grid 90° clockwise
    #[must_use]
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::missing_panics_doc))]
    pub fn rotate(&self) -> Self {
        let mut result = Self::empty(self.height(), self.width()).unwrap();
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.set(result.width() - y - 1, x, self.get(x, y));
            }
        }
        result
    }

    /// Flip grid vertically
    #[must_use]
    pub fn vertical_flip(&self) -> Self {
        let mut result = *self;
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.set(result.width() - x - 1, y, self.get(x, y));
            }
        }
        result
    }

    /// Flip grid horizontally
    #[must_use]
    pub fn horizontal_flip(&self) -> Self {
        let mut result = *self;
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.set(x, result.height() - y - 1, self.get(x, y));
            }
        }
        result
    }
}

impl FromStr for SmallBitGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or(())
    }
}

#[test]
fn set_works() {
    let mut grid = SmallBitGrid::parse(".#.|##.").unwrap();
    grid.set(2, 1, true);
    grid.set(0, 0, true);
    grid.set(1, 0, false);
    assert_eq!(&format!("{}", grid), "#..|###",);
}

/// Convert bits in a number to an array but in reverse order.
pub fn bits_to_arr(num: GridBits) -> [bool; 64] {
    let mut grid = [false; 64];

    #[allow(clippy::needless_range_loop)]
    for grid_idx in 0..64 {
        grid[grid_idx] = ((num >> grid_idx) & 1) == 1;
    }
    grid
}

#[test]
fn bits_to_arr_works() {
    assert_eq!(
        bits_to_arr(0b1011001),
        [
            true, false, false, true, true, false, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false
        ]
    );
}

/// Reverse of [`bits_to_arr`]
///
/// # Panics
/// - `grid` has more than 64 elements
pub fn arr_to_bits(grid: &[bool]) -> GridBits {
    assert!(
        grid.len() <= 8 * std::mem::size_of::<GridBits>(),
        "grid cannot have more than 64 elements"
    );
    let mut res: GridBits = 0;
    for i in (0..grid.len()).rev() {
        res <<= 1;
        res |= grid[i] as GridBits;
    }
    res
}

#[test]
fn bits_to_arr_to_bits_roundtrip() {
    let inp = 3874328;
    assert_eq!(inp, arr_to_bits(&bits_to_arr(inp)),);
}

#[test]
fn parse_grid() {
    let width = 3;
    let height = 3;
    assert_eq!(
        SmallBitGrid::parse("..#|.#.|##.").unwrap(),
        SmallBitGrid::from_arr(
            width,
            height,
            &[false, false, true, false, true, false, true, true, false]
        )
        .unwrap()
    );
}

#[test]
fn rotation_works() {
    let position = SmallBitGrid::from_str(
        "##..|\
	 ....|\
	 #..#",
    )
    .unwrap()
    .rotate();

    assert_eq!(
        &format!("{position}"),
        "#.#|\
	 ..#|\
	 ...|\
	 #.."
    );

    let position = position.rotate();
    assert_eq!(
        &format!("{position}"),
        "#..#|\
	 ....|\
	 ..##"
    );
}

#[test]
fn flip_works() {
    let position = SmallBitGrid::parse(
        "##..|\
	 ....|\
	 #..#",
    )
    .unwrap();

    assert_eq!(
        &format!("{}", position.vertical_flip()),
        "..##|\
	 ....|\
	 #..#",
    );

    assert_eq!(
        &format!("{}", position.horizontal_flip()),
        "#..#|\
	 ....|\
	 ##..",
    );
}
