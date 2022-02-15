# This file contains information about how to reproduce our results

## Preliminaries

### Install rust
See: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Install anaconda
See: [https://docs.anaconda.com/anaconda/install/index.html](https://docs.anaconda.com/anaconda/install/index.html)

### (Recommended) install mamba for faster virtual environment installations
See: [https://github.com/mamba-org/mamba](https://github.com/mamba-org/mamba)

### Install and activate the virtual environment:
```bash
<mamba/conda> env create -f conda_environment.yml && \
conda activate natural_computing
```

## Exercise 4
We only figured out tuesday evening that we had to submit our code in a github repository. Sadly, the team-mate which has the code for this assignment on his PC is not able to provide the code before the deadline. We'll add the code as soon as it's available.

## Exercise 6

Generate all the results used in the exercises. Either use the script: 
```bash
bash run_ass6.sh
```
Or perform the steps below.
### Navigate to the rust project
```bash
cd rust_gen_results
```
### Generate the results for the given dataset using the EA algorithm
```bash
cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ea_given \
    --location-file ../data/file-tsp.txt \
    --alg-type ea
```
### Generate the results for the given dataset using the MA algorithm
```bash
cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ma_given \
    --location-file ../data/file-tsp.txt \
    --alg-type ma
```

### Generate the results for the USA capitals dataset using the EA algorithm
```bash
cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ea_usa \
    --location-file ../data/tsp-usa-capitals.txt \
    --alg-type ea
```

### Generate the results for the USA capitals dataset using the MA algorithm
```bash
cargo run --release -- \
    --num-runs 10 \
    --num-epochs 1500 \
    --population-size 100 \
    --result-dir ../results/ma_usa \
    --location-file ../data/tsp-usa-capitals.txt \
    --alg-type ma
```

### Change directory back to the root repository
```bash
cd ..
```

### Plot the results for the given dataset using the EA algorithm
```bash
python sub_ass_6_plot_results.py \
    --in-file results/ea_given/data.json \
    --out-dir results/ea_given \
    --location-file data/file-tsp.txt
```

### Plot the results for the given dataset using the MA algorithm
```bash
python sub_ass_6_plot_results.py \
    --in-file results/ma_given/data.json \
    --out-dir results/ma_given \
    --location-file data/file-tsp.txt
```

### Plot the results for the USA capitals dataset using the EA algorithm
```bash
python sub_ass_6_plot_results.py \
    --in-file results/ea_usa/data.json \
    --out-dir results/ea_usa \
    --location-file data/tsp-usa-capitals.txt
```

### Plot the results for the USA capitals dataset using the MA algorithm
```bash
python sub_ass_6_plot_results.py \
    --in-file results/ea_usa/data.json \
    --out-dir results/ea_usa \
    --location-file data/tsp-usa-capitals.txt
```

### Inspect the results
The plots are in the `results` directory.

## Exercise 8
Open the notebook `Exercise_8.ipynb` and run all the cells.



