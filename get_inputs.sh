#!/bin/bash

get_inputs() {
    local file filename regex dest_name year day part
    for file in "$HOME"/Downloads/everybody_codes*; do
        filename="$(basename "$file")"
        regex="everybody_codes_e([0-9]+)_q([0-9]+)_p([0-9]).txt"

        [[ "$filename" =~ $regex ]] || continue

        year="${BASH_REMATCH[1]}"
        day="${BASH_REMATCH[2]}"
        part="${BASH_REMATCH[3]}"
        [[ "$year" -lt 2020 ]] && dest_name=story || dest_name=year

        mkdir -p "input/${dest_name}_$year/day$day"
        mv "$file" "input/${dest_name}_$year/day$day/part$part.txt"
    done
}

get_inputs
