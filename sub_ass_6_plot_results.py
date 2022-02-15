import click
import numpy as np
import matplotlib.pyplot as plt
from os.path import join as pjoin
from dataclasses import dataclass
from functools import cached_property
from typing import List
import os
import pickle
import orjson

@dataclass
class Results:
    fitness_hist: List[List[List[float]]] # for each run, for each epoch, all fitnesses
    population_hist: List[List[np.ndarray]] # for each run, for each epoch, all populations

    @cached_property
    def fitness_hist_mean(self) -> List[List[float]]:
        return [[np.mean(fitnesses) for fitnesses in run] for run in self.fitness_hist]

    @cached_property
    def fitness_hist_max(self) -> List[List[float]]:
        return [[np.max(fitnesses) for fitnesses in run] for run in self.fitness_hist]


def plot_route(route, locations):
    plt.plot(locations[route, 0], locations[route, 1], '-o')

@click.command()
@click.option("--in-file", type=click.Path(exists=True))
@click.option("--out-dir", type=str)
@click.option("--location-file", type=click.Path(exists=True))
@click.option("--population-size", type=int, default=100)
@click.option("--p-mutation", type=float, default=0.1)
def main(in_file: str ,out_dir: str, location_file: str, population_size: int, p_mutation: float):
    os.makedirs(out_dir, exist_ok=True)

    with open(in_file, 'rb') as f:
        if in_file.endswith('.pkl'):
            data: Results = pickle.load(f)
        elif in_file.endswith('.json'):
            data: dict = orjson.loads(f.read())
            data = Results(
                fitness_hist=data['fitness_hist'],
                population_hist=data['population_hist']
            )

    locations = np.loadtxt(location_file)


    fig, subplots = plt.subplots(1, 2, figsize=(12, 6))
    fig.suptitle(f'Fitness per epoch, {population_size=}, {p_mutation=}')

    plt_mean, plt_max = subplots.flatten()

    plt_mean.set_title('Mean')
    plt_mean.set_xlabel('Epoch')

    plt_max.set_title('Max')
    plt_max.set_xlabel('Epoch')

    for means, maxs in zip(data.fitness_hist_mean, data.fitness_hist_max):
        plt_mean.plot(means)
        plt_max.plot(maxs)

    fig.tight_layout()
    fig.savefig(pjoin(out_dir, 'fitness.png'))

    plt.clf()

    for i, (run_fitnesses, candidates) in enumerate(zip(data.fitness_hist, data.population_hist)):    
        last_epoch_fitnesses, last_epoch_candidates = run_fitnesses[-1], candidates[-1]
        most_fit_idx = np.argmax(last_epoch_fitnesses)

        route = last_epoch_candidates[most_fit_idx]
        plot_route(route, locations)
        plt.title("Best route")
        plt.tight_layout()
        plt.savefig(pjoin(out_dir, f'best_route_{i}.png'))
        plt.clf()

if __name__ == '__main__':
    main()