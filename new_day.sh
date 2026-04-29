#!/bin/bash

create_new_file() {
  local src_path
  while IFS= read -r -d '' file; do
    nb_files=$(ls -1q "$file" | wc -l)
    if [[ ("$file" == src/year_* && "$nb_files" -lt 20) || ("$file" == src/story_* && "$nb_files" -lt 3) ]]; then
      src_path="$file"
      break
    fi
  done < <(find src/* -type d -print0 | tac -s $'\0')

  if [[ -z "$src_path" ]]; then
    echo "No missing files in current directories"
    return 1
  fi
  local target_name="${src_path##src/}"

  local day
  day=$(ls "$src_path" | grep -Eo "[0-9]+" | sort -nr | head -1)
  day=${day##0} # Remove leading zero to avoid octal error
  ((day++))

  day=$(printf "%02d" $day)

  mkdir -p "${src_path/src/input}"/day"$day"

  sed -i "/load_src\!($target_name/ s/\(day[[:digit:]]\{2\}\))/\1, day$day)/" src/lib.rs

  cat <<EOF >"$src_path"/day"$day".rs
use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!(),
    }
}
EOF
}

create_new_file
