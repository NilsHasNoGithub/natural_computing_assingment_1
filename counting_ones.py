import numpy as np
import matplotlib.pyplot as plt

def initialize_bitstring(length):
    return np.random.randint(0, 2, length)

def mutate(bitstring, mutate_probability=0.1):
    x_m = np.copy(bitstring)
    mutations = np.random.uniform(0, 1, bitstring.shape[0]) < mutate_probability
    x_m = (x_m + mutations) % 2

    score1 = np.sum(bitstring)
    score2 = np.sum(x_m)

    # jsut return x_m for 4c
    #return x_m

    if score2 > score1:
        return x_m
    return bitstring

def mutation_iterations(bitstring, iterations, mutate_probability):
    best_scores = np.array([])
    bitstring = np.copy(bitstring)
    best_score = 0
    for i in range(iterations):
        bitstring = mutate(bitstring, mutate_probability=mutate_probability)
        score = np.sum(bitstring)
        if score > best_score:
            best_score = score
        best_scores = np.append(best_scores, best_score)

    return bitstring, best_scores

length = 100
mutation_rate = 1 / length
iterations = 1500
runs = 10

best_scores_all = []
reached_max = 0
for run in range(runs):
    bitstring = initialize_bitstring(length)
    bitstring, best_scores = mutation_iterations(bitstring, iterations=iterations, mutate_probability=mutation_rate)
    best_scores_all.append(best_scores)
    if best_scores[iterations-1] == length:
        reached_max += 1

print(reached_max, "/", runs, "runs reached score", length)

xAxis = np.arange(iterations)
for run in range(runs):
    yAxis = best_scores_all[run]
    plt.plot(xAxis, yAxis)
plt.title("Counting Ones")
plt.xlabel("Iteration")
plt.ylabel("Best fitness")
plt.show()
