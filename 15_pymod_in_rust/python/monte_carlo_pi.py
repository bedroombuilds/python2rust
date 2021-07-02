"""
Silly method to compute PI using monte carlo approach
"""
import math
import random
import time
import concurrent.futures
import psutil
import multiprocessing


def monte_carlo_pi(iterations):
    num_cpus = psutil.cpu_count(logical=True)  # real and virtual

    now = time.time()
    #total_inside = thread_monte_carlo_pi(num_cpus, iterations)
    total_inside = proc_monte_carlo_pi(num_cpus, iterations)

    stop = time.time()
    total_iterations = num_cpus * iterations
    elapsed = stop - now
    pi = total_inside / total_iterations * 4
    calculations_string = round(total_iterations / elapsed)
    return (pi, f'{calculations_string:,}')


def monte_compute(iterations):
    inside = 0
    random.seed()
    for _ in range(iterations):
        a = random.random()
        b = random.random()
        c = math.pow(a, 2.0) + math.pow(b, 2.0)
        if c <= 1.0:
            inside += 1
    return inside


def proc_monte_carlo_pi(num_cpus, iterations):
    with multiprocessing.Pool(num_cpus) as p:
        results = p.map(monte_compute, [iterations] * num_cpus)
        total_inside = sum(results)
    return total_inside


def thread_monte_carlo_pi(num_cpus, iterations):
    with concurrent.futures.ThreadPoolExecutor() as executor:
        futures = [executor.submit(monte_compute, iterations) for _ in range(num_cpus)]
        total_inside = sum([f.result() for f in futures])
    return total_inside


if __name__ == "__main__":
    start = time.time()
    pi, calcs_p_sec = monte_carlo_pi(1_000_000)
    stop = time.time()
    print(f'{stop - start:.2f}secs runtime')
    print(pi)
    print(f"{calcs_p_sec} calculations per second")
