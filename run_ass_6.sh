#!/usr/bin/env bash

set -e

cd rust_gen_results

cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ea_given \
    --location-file ../data/file-tsp.txt \
    --alg-type ea

cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ma_given \
    --location-file ../data/file-tsp.txt \
    --alg-type ma

cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ea_usa \
    --location-file ../data/tsp-usa-capitals.txt \
    --alg-type ea

cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ma_usa \
    --location-file ../data/tsp-usa-capitals.txt \
    --alg-type ma

cd ..

python sub_ass_6_plot_results.py \
    --in-file results/ma_given/data.json \
    --out-dir plots/ma_given \
    --location-file data/file-tsp.txt

python sub_ass_6_plot_results.py \
    --in-file results/ea_usa/data.json \
    --out-dir plots/ea_usa \
    --location-file data/tsp-usa-capitals.txt

python sub_ass_6_plot_results.py \
    --in-file results/ea_usa/data.json \
    --out-dir plots/ea_usa \
    --location-file data/tsp-usa-capitals.txt

python sub_ass_6_plot_results.py \
    --in-file results/ea_usa/data.json \
    --out-dir plots/ea_usa \
    --location-file data/tsp-usa-capitals.txt