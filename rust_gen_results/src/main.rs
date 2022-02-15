/// `Cargo.toml`:
/// ```toml
/// [package]
/// name = "rust_gen_results"
/// version = "0.1.0"
/// edition = "2021"

/// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

/// [dependencies]
/// indicatif = "0.16.2"
/// itertools = "0.10.3"
/// rand = "0.8.5"
/// rayon = "1.5.1"
/// serde = { version = "1.0.136", features = ["derive"] }
/// serde_json = "1.0.79"
/// structopt = "0.3.26"
/// ```
use std::{path::PathBuf, str::FromStr};

use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct Results {
    fitness_hist: Vec<Vec<Vec<f32>>>,
    population_hist: Vec<Vec<Vec<Vec<i32>>>>,
}

#[derive(Debug, Clone, Copy)]
enum AlgType {
    Ea,
    Ma,
}

impl FromStr for AlgType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ea" => Ok(Self::Ea),
            "ma" => Ok(Self::Ma),
            _ => Err("Invalid alg"),
        }
    }
}

fn init_population(pop_size: usize, candidate_len: i32) -> Vec<Vec<i32>> {
    let mut base: Vec<_> = (0..candidate_len).collect();
    let mut rng = rand::thread_rng();
    (0..pop_size)
        .map(|_| {
            base.shuffle(&mut rng);
            base.clone()
        })
        .collect()
}

fn fill_missing(offspring: &mut [i32], missing_ordered: &[i32]) {
    let mut miss_idx = 0;
    for i in 0..offspring.len() {
        if offspring[i] == -1 {
            offspring[i] = missing_ordered[miss_idx];
            miss_idx += 1;
        }
    }
}

fn mutate(individual: &mut [i32]) {
    let l = individual.len();

    let mut rng = rand::thread_rng();

    let idx0 = rng.gen_range(0..l);
    let mut idx1 = rng.gen_range(0..l);

    while idx0 == idx1 {
        idx1 = rng.gen_range(0..l);
    }

    individual.swap(idx0, idx1);
}

fn fitness(candidate: &[i32], locations: &[(f32, f32)]) -> f32 {
    let mut total_distance = 0.0f32;

    for (i, j) in candidate.iter().copied().tuple_windows() {
        let i = i as usize;
        let j = j as usize;
        total_distance += ((locations[i].0 - locations[j].0).powi(2)
            + (locations[i].1 - locations[j].1).powi(2))
        .sqrt();
    }
    1.0 / total_distance
}

fn apply2opt(mut individual: Vec<i32>, locations: &[(f32, f32)]) -> Vec<i32> {
    let mut best_fitness = fitness(&individual, locations);

    loop {
        let mut improved = false;
        'outer: for i in 0..individual.len() {
            for j in i + 1..individual.len() {
                let mut new_individual = individual.clone();
                new_individual.swap(i, j);
                let new_fitness = fitness(&new_individual, locations);

                if new_fitness > best_fitness {
                    individual = new_individual;
                    best_fitness = new_fitness;
                    improved = true;
                    break 'outer;
                }
            }
        }

        if !improved {
            break;
        }
    }

    individual
}

fn generate_offspring(
    p1: &[i32],
    p2: &[i32],
    locations: &[(f32, f32)],
    p_mutation: f32,
    alg: AlgType,
) -> (Vec<i32>, Vec<i32>) {
    let length = p1.len();
    let mut rng = rand::thread_rng();

    let mut cut_start = rng.gen_range(0..length);
    let mut cut_end = rng.gen_range(0..length);

    if cut_end > cut_start {
        std::mem::swap(&mut cut_start, &mut cut_end);
    }

    let mut offspring1 = vec![-1; length];
    let mut offspring2 = offspring1.clone();

    for i in cut_start..cut_end {
        offspring1[i] = p1[i];
        offspring2[i] = p2[i];
    }

    let mut missing1: Vec<_> = (0..length)
        .map(|c| c as i32)
        .filter(|c| !offspring1.contains(c))
        .collect();
    let mut missing2: Vec<_> = (0..length)
        .map(|c| c as i32)
        .filter(|c| !offspring2.contains(c))
        .collect();

    missing1.sort_by_key(|&c| {
        p2.iter()
            .copied()
            .position(|c2| c == c2).unwrap()
    });
    missing2.sort_by_key(|&c| {
        p1.iter()
            .copied()
            .position(|c2| c == c2).unwrap()
    });

    fill_missing(&mut offspring1, &missing1);
    fill_missing(&mut offspring2, &missing2);

    if rng.gen::<f32>() < p_mutation {
        mutate(&mut offspring1);
    }
    if rng.gen::<f32>() < p_mutation {
        mutate(&mut offspring2);
    }

    match alg {
        AlgType::Ea => (offspring1, offspring2),
        AlgType::Ma => (
            apply2opt(offspring1, locations),
            apply2opt(offspring2, locations),
        ),
    }
}

fn run(
    locations: &[(f32, f32)],
    pop_size: usize,
    epochs: usize,
    p_mutation: f32,
    alg: AlgType,
) -> (Vec<Vec<f32>>, Vec<Vec<Vec<i32>>>) {
    let mut rng = rand::thread_rng();
    let mut population = init_population(pop_size, locations.len() as i32);

    let mut pop_hist = vec![population.clone()];
    let mut fitnesses = Vec::with_capacity(epochs);

    for _ in (0..epochs).progress_count(epochs as u64) {
        let fitness_values: Vec<_> = population.iter().map(|c| fitness(&c, locations)).collect();
        fitnesses.push(fitness_values.clone());

        let total_fitness: f32 = fitness_values.iter().sum();
        let prop_fitnesses: Vec<_> = fitness_values.iter().map(|&f| f / total_fitness).collect();

        let mut mating_pool = Vec::with_capacity(pop_size);

        for _ in 0..pop_size {
            let p1_idx = rng.gen_range(0..pop_size);
            let p2_idx = loop {
                let i = rng.gen_range(0..pop_size);
                if i != p1_idx {
                    break i;
                }
            };

            if prop_fitnesses[p1_idx] > prop_fitnesses[p2_idx] {
                mating_pool.push(&population[p1_idx]);
            } else {
                mating_pool.push(&population[p2_idx]);
            }

        }

        let mut new_population = Vec::new();


        for mut chunk in &mating_pool.iter().chunks(2) {

            let p1 = *chunk.next().unwrap();
            let p2 = *chunk.next().unwrap();

            let (offspring1, offspring2) = generate_offspring(p1, p2, locations, p_mutation, alg);

            new_population.push(offspring1);
            new_population.push(offspring2);
        }

        assert_eq!(population.len(), new_population.len());
        population = new_population;
        pop_hist.push(population.clone());
    }

    return (fitnesses, pop_hist);
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    location_file: PathBuf,

    #[structopt(long, default_value = "1")]
    num_runs: usize,

    #[structopt(long, default_value = "1500")]
    num_epochs: usize,

    #[structopt(long, default_value = "results")]
    result_dir: PathBuf,

    #[structopt(long, default_value = "ea")]
    alg_type: AlgType,

    #[structopt(long, default_value = "100")]
    population_size: usize,

    #[structopt(long, default_value = "0.1")]
    p_mutate: f32,
}

fn find_min_f(iter: impl IntoIterator<Item = f32>) -> f32 {
    iter.into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn find_max_f(iter: impl IntoIterator<Item = f32>) -> f32 {
    iter.into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn main() {
    let opt = Opt::from_args();

    std::fs::create_dir_all(&opt.result_dir).unwrap();

    let locations_str = std::fs::read_to_string(&opt.location_file).unwrap();

    let locations: Vec<(f32, f32)> = locations_str
        .lines()
        .map(|l| {
            let l = l.trim();
            let mut split = l.split_whitespace();

            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let min_x = find_min_f(locations.iter().copied().map(|(x, _)| x));
    let max_x = find_max_f(locations.iter().copied().map(|(x, _)| x));

    let min_y = find_min_f(locations.iter().copied().map(|(_, y)| y));
    let max_y = find_max_f(locations.iter().copied().map(|(_, y)| y));

    let max = max_x.max(max_y);
    let min = min_x.min(min_y);
    let delta = max - min;

    // normalize for 
    let locations: Vec<_> = locations
        .into_iter()
        .map(|(x, y)| ((x - min) / delta, (y - min) / delta))
        .collect();

    let run_results: Vec<_> = (0..opt.num_runs)
        .into_par_iter()
        .map(|_| {
            run(
                &locations,
                opt.population_size,
                opt.num_epochs,
                opt.p_mutate,
                opt.alg_type,
            )
        })
        .collect();

    let mut fitness_hist = Vec::new();
    let mut population_hist = Vec::new();

    for (f, p) in run_results {
        fitness_hist.push(f);
        population_hist.push(p);
    }

    let results = Results {
        fitness_hist,
        population_hist,
    };

    let save_file_name = "data.json";

    let save_file = opt.result_dir.join(save_file_name);

    let json = serde_json::to_string(&results).unwrap();

    std::fs::write(save_file, json).unwrap();
}
