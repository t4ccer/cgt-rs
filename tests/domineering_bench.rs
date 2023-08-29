use cgt::short::partizan::{
    games::domineering::Domineering, partizan_game::PartizanGame,
    transposition_table::TranspositionTable,
};
use std::hint::black_box;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn bench_domineering() {
    let profiler = dhat::Profiler::builder().build();

    let width = black_box(5);
    let height = black_box(4);

    let tt = TranspositionTable::new();
    for i in 0..(width * height) {
        let domineering = Domineering::from_number(width as u8, height as u8, i).unwrap();
        let _ = domineering.canonical_form(&tt);
    }

    let stats = dhat::HeapStats::get();
    eprintln!("{:#?}", stats);
    drop(profiler);
}