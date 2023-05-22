#![feature(extend_one)]
// mod game;
mod canonical_short_game;
mod dyadic_rational_number;
mod grid;

// use game::*;
use canonical_short_game::*;
use grid::*;

use crate::dyadic_rational_number::DyadicRationalNumber;
// use grid::*;

fn main() {
    let mut gs = GameStorage::new();
    let grid = Grid::empty(4, 3);
    let game = grid.to_game(&mut gs);
    let mut buf = String::new();
    gs.display_game(game, &mut buf).unwrap();
    println!("{}", buf);

    // println!("{:?}", gs.zero_id);
    // println!("{:?}", gs.star_id);

    // gs.construct_rational(DyadicRationalNumber::rational(1, 2).unwrap());

    // ./target/release/cg  0.54s user 0.04s system 99% cpu 0.575 total
    // println!("{}", Grid::empty(5, 5).to_game());
    // println!("{}", Grid::empty(5, 4).reduce());

    // let grid = Grid::empty(3, 3);
    // println!("{}", grid.to_game());

    // for m in grid.right_moves() {
    // println!("{}\n", m.move_top_left());
    // }

    // println!("{}", Grid::to_game(&Grid::empty(5, 4)));

    // let grid = Grid::empty(4, 5);
    // println!("\"{grid}\"\n  Canonical Form: {}", grid.to_game());

    // let grid = Grid::parse(3, 3, "..#|.#.|##.").unwrap();
    // println!("{}\n", grid);
    // for g in grid.decompositons() {
    //     println!("{g}\n");
    // }

    // Takes 30s with -r
    // println!("{}", Grid::empty(5, 5).to_game());
}
