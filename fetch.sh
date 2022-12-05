#!/bin/bash


day=$(date +%-d)
year=$(date +%Y)
dir=day_$(($day + 1))
echo "Creating" $dir

cargo new $dir --lib
cp template.rs $dir/src/lib.rs
touch $dir/src/input.txt
touch $dir/src/sample.txt

