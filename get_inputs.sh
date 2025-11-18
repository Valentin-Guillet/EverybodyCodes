#!/bin/bash

for file in $HOME/Downloads/everybody_codes*; do
    filename="$(basename "$file")"
    year="${filename:17:4}"
    day="${filename:23:2}"
    part="${filename:27:1}"
    mv "$file" "input/year_$year/day$day/part$part.txt"
done
