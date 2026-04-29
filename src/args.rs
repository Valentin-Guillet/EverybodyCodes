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

    #[arg(short, long, conflicts_with = "year", num_args = 0..=1, default_missing_value = "0")]
    story: Option<u32>,

    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long)]
    part: Option<u8>,
}

pub struct RunArgs {
    pub year: Option<u32>,
    pub story: Option<u32>,
    pub day: u8,
    pub part: u8,
    pub input_file: PathBuf,
}

impl RunArgs {
    pub fn parse() -> Result<Self, Box<dyn Error>> {
        let args = RunOptionArgs::parse();

        let (year, story) = match (args.year, args.story) {
            (None, None) => (Some(get_max_src_directory("src", "year_")?), None),
            (None, Some(0)) => (None, Some(get_max_src_directory("src", "story_")?)),
            (year, story) => (year, story),
        };
        let mut src_path = if year.is_some() {
            format!("src/year_{}", year.unwrap())
        } else {
            format!("src/story_{}", story.unwrap())
        };
        let input_path = format!("input/{}", &src_path[4..]);

        let day = match args.day {
            Some(day) => day,
            None => get_max_day_file(&src_path)?,
        };

        src_path.push_str(&format!("/day{day:02}.rs"));
        if !Path::new(&src_path).exists() {
            let src_msg = if year.is_some() {
                format!("year {}", year.unwrap())
            } else {
                format!("story {}", story.unwrap())
            };
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no source file for {src_msg} day {day:02}"),
            )
            .into());
        }

        let part = match args.part {
            Some(part) if (1..=3).contains(&part) => part,
            Some(part) => Err(format!("invalid part {part}"))?,
            None => get_default_part(&input_path, day)?,
        };

        let input_file = args
            .input_file
            .unwrap_or_else(|| get_default_input(&input_path, day, part));

        if !Path::new(&input_file).exists() {
            let input_msg = if year.is_some() {
                format!("year {}", year.unwrap())
            } else {
                format!("story {}", story.unwrap())
            };
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no input file for {input_msg} day {day:02} part {part}"),
            )
            .into());
        }

        Ok(Self {
            story,
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

fn get_max_src_directory(path: &str, prefix: &str) -> Result<u32, Box<dyn Error>> {
    read_max_entry(
        path,
        |e| e.file_type().is_ok_and(|e| e.is_dir()),
        |name| name.strip_prefix(prefix)?.parse().ok(),
    )
}

fn get_max_day_file(path: &str) -> Result<u8, Box<dyn Error>> {
    read_max_entry(
        path,
        |e| e.file_type().is_ok_and(|e| e.is_file()),
        |name| name.strip_prefix("day")?.strip_suffix(".rs")?.parse().ok(),
    )
}

fn get_default_part(input_path: &str, day: u8) -> Result<u8, Box<dyn Error>> {
    read_max_entry(
        format!("{}/day{day:02}", input_path).as_str(),
        |e| e.file_type().is_ok_and(|e| e.is_file()),
        |name| {
            name.strip_prefix("part")?
                .strip_suffix(".txt")?
                .parse()
                .ok()
        },
    )
}

fn get_default_input(input_path: &str, day: u8, part: u8) -> PathBuf {
    format!("{}/day{day:02}/part{part}.txt", input_path).into()
}
