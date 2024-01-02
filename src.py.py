import time
import random


def estimate_pi(iterations):
    inside_circle = 0
    for _ in range(iterations):
        x, y = random.random(), random.random()
        if x*x + y*y <= 1.0:
            inside_circle += 1
    return 4 * inside_circle / iterations


pi_estimates = []
execution_times = []
iterations = 10000
max_executions = 10

for _ in range(max_executions):
    start_time = time.time()
    pi_estimate = estimate_pi(iterations)
    end_time = time.time()

    execution_time = (end_time - start_time) * 1000  # milliseconds
    pi_estimates.append(pi_estimate)
    execution_times.append(execution_time)

    print(f"推定値: {pi_estimate}, 時間: {execution_time}ms")

average_pi = sum(pi_estimates) / len(pi_estimates)
average_time = sum(execution_times) / len(execution_times)
print(f"\n平均円周率の推定値: {average_pi}")
print(f"平均実行時間: {average_time}ms")
