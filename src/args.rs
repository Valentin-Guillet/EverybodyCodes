use clap::Parser;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(version, about)]
struct RunOptionArgs {
    #[arg(short, long = "input")]
    input_file: Option<PathBuf>,

    #[arg(short, long)]
    year: Option<u32>,

    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long)]
    part: Option<u8>,
}

pub struct RunArgs {
    pub year: u32,
    pub day: u8,
    pub part: u8,
    pub input_file: PathBuf,
}

impl RunArgs {
    pub fn parse() -> Result<Self, Box<dyn Error>> {
        let args = RunOptionArgs::parse();

        let year = match args.year {
            Some(year) => year,
            None => get_max_year_directory("src")?,
        };
        let day = match args.day {
            Some(day) => day,
            None => get_max_day_file(&format!("src/year_{year}"))?,
        };

        let source_file = format!("src/year_{year}/day{day:02}.rs");
        if !Path::new(&source_file).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no source file for year {year} day {day:02}"),
            )
            .into());
        }

        let part = match args.part {
            Some(part) if (1..=3).contains(&part) => part,
            Some(part) => Err(format!("invalid part {part}"))?,
            None => get_default_part(year, day)?,
        };

        let input_file = args
            .input_file
            .unwrap_or_else(|| get_default_input(year, day, part));

        if !Path::new(&input_file).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no input file for year {year} day {day:02} part {part}"),
            )
            .into());
        }

        Ok(Self {
            year,
            day,
            part,
            input_file,
        })
    }
}

fn read_max_entry<T, P, F>(path: &str, predicate: P, parse_fn: F) -> Result<T, Box<dyn Error>>
where
    T: Ord,
    P: Fn(&fs::DirEntry) -> bool,
    F: Fn(&str) -> Option<T>,
{
    fs::read_dir(path)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, format!("no directory {path}")))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let file_name_os = e.file_name();
                let file_name = file_name_os.to_str()?;
                predicate(&e).then(|| parse_fn(file_name)).flatten()
            })
        })
        .max()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("no valid entry in {path}")).into()
        })
}

fn get_max_year_directory(path: &str) -> Result<u32, Box<dyn Error>> {
    read_max_entry(
        path,
        |e| e.file_type().is_ok_and(|e| e.is_dir()),
        |name| name.strip_prefix("year_")?.parse().ok(),
    )
}

fn get_max_day_file(path: &str) -> Result<u8, Box<dyn Error>> {
    read_max_entry(
        path,
        |e| e.file_type().is_ok_and(|e| e.is_file()),
        |name| name.strip_prefix("day")?.strip_suffix(".rs")?.parse().ok(),
    )
}

fn get_default_part(year: u32, day: u8) -> Result<u8, Box<dyn Error>> {
    read_max_entry(
        format!("input/year_{year}/day{day:02}").as_str(),
        |e| e.file_type().is_ok_and(|e| e.is_file()),
        |name| name.strip_prefix("part")?.strip_suffix(".txt")?.parse().ok(),
    )
}

fn get_default_input(year: u32, day: u8, part: u8) -> PathBuf {
    format!("input/year_{year}/day{day:02}/part{part}.txt").into()
}
