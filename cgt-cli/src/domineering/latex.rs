use super::common::DomineeringResult;
use anyhow::{anyhow, Context, Result};
use cgt::{grid::FiniteGrid, numeric::rational::Rational, short::partizan::games::domineering};
use clap::Parser;
use std::{
    fs::File,
    io::{stdin, stdout, BufReader, BufWriter, Read, Write},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct DomineeringEntry {
    temperature: Rational,
    grid: domineering::Domineering,
}

impl DomineeringEntry {
    fn new(result: &DomineeringResult) -> Result<Self> {
        Ok(DomineeringEntry {
            temperature: Rational::from_str(&result.temperature)
                .ok()
                .context("Invalid temperature")?,
            grid: domineering::Domineering::from_str(&result.grid)
                .ok()
                .context("Invalid grid")?,
        })
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Input newline-separated JSON file, usually obtained by running `search` command. Use '-' for stdin
    #[arg(long)]
    in_file: String,

    /// Output LaTeX file with generated table. Use '-' for stdout
    #[arg(long, default_value = "-")]
    out_file: String,

    /// Number of columns in LaTeX file
    #[arg(long, default_value_t = 3)]
    columns: usize,

    /// Position scale, ie. scaling factor of tile size. 1 => 1cm.
    #[arg(long, default_value_t = 0.4)]
    position_scale: f32,

    /// Include positions that are rotations of already included positions
    #[arg(long, default_value_t = false)]
    include_rotations: bool,
}

pub fn run(args: Args) -> Result<()> {
    let input: BufReader<Box<dyn Read>> = if args.in_file == "-" {
        BufReader::new(Box::new(stdin()))
    } else {
        BufReader::new(Box::new(
            File::open(&args.in_file)
                .context(format!("Could not open input file '{}'", args.in_file))?,
        ))
    };

    let mut output: BufWriter<Box<dyn Write>> = if args.out_file == "-" {
        BufWriter::new(Box::new(stdout()))
    } else {
        BufWriter::new(Box::new(File::create(&args.out_file).context(format!(
            "Could not create/open output file '{}'",
            args.out_file
        ))?))
    };

    let input = serde_json::de::Deserializer::from_reader(input)
        .into_iter::<DomineeringResult>()
        .map(|line| {
            line.context("Could not parse JSON '{line}'")
                .and_then(|r| DomineeringEntry::new(&r))
        })
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // remove rotations
    let mut input = if args.include_rotations {
        input
    } else {
        let mut input_without_rotations =
            input.iter().cloned().map(|e| Some(e)).collect::<Vec<_>>();
        for (idx, entry) in input.iter().enumerate() {
            let grid = *entry.grid.grid();
            let rot_90deg = grid.rotate();
            let rot_180deg = rot_90deg.rotate();
            let rot_270deg = rot_180deg.rotate();
            let vertical_flip = grid.vertical_flip();
            let horizontal_flip = grid.horizontal_flip();
            let equivalent_grids = [
                rot_90deg,
                rot_180deg,
                rot_270deg,
                vertical_flip,
                horizontal_flip,
            ];
            for idx in (idx + 1)..input_without_rotations.len() {
                if let Some(next_entry) = &input_without_rotations[idx] {
                    if equivalent_grids.contains(&next_entry.grid.grid()) {
                        input_without_rotations[idx] = None;
                    }
                }
            }
        }
        input_without_rotations
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    };

    input.sort_by(|lhs, rhs| rhs.temperature.cmp(&lhs.temperature)); // descending sort

    let max_grid_width = input
        .iter()
        .map(|entry| entry.grid.grid().width())
        .max()
        .context("Input file was empty")?;

    let mut input = input.into_iter().peekable();

    let pos_width = format!("{}cm", args.position_scale * (max_grid_width as f32));

    // define table
    if args.columns <= 0 {
        Err(anyhow!("Must have at least 1 column"))?;
    }
    writeln!(output, "{{")?;
    writeln!(output, "%% Auto generated by `cgt-cli`")?;
    writeln!(output, "%% Make sure to include preamble from README.md")?;
    write!(output, "\\begin{{longtabu}}{{m{{{pos_width}}} m{{1cm}}")?;
    for _ in 1..args.columns {
        write!(output, "|m{{{pos_width}}} m{{1cm}}")?;
    }
    write!(output, "}} \n\\hline ")?;

    // header
    for idx in 0..args.columns {
        if idx != 0 {
            write!(output, "& ")?;
        }
        write!(output, "Position & Temp. ")?;
    }
    writeln!(output, "\\\\ \\hline \\endhead")?;

    // entries
    while input.peek().is_some() {
        for idx in 0..args.columns {
            match input.next() {
                Some(entry) => {
                    if idx != 0 {
                        write!(output, "{}", "& ")?;
                    }
                    write!(
                        output,
                        "{} & ${}$ ",
                        entry.grid.to_latex_with_scale(args.position_scale),
                        entry.temperature
                    )?;
                }
                None => {}
            }
        }
        writeln!(output, "{}", r"\\")?;
    }

    writeln!(output, "{}", r"\end{longtabu}")?;
    writeln!(output, "}}")?;
    Ok(())
}
