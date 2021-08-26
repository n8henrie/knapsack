#!/usr/bin/env python3

from collections import namedtuple
from itertools import combinations

import knapsack


def solve_it(input_data, language="rust"):
    if language == "python":
        return solve_it_python(input_data)
    return solve_it_rust(input_data)


def solve_it_rust(input_data):
    return knapsack.solve(input_data)


Item = namedtuple("Item", ["index", "value", "weight"])


def solve_it_python(input_data):
    print("running in python", file=sys.stderr)
    # parse the input
    lines = input_data.split("\n")

    firstLine = lines[0].split()
    item_count = int(firstLine[0])
    capacity = int(firstLine[1])

    items = []

    for i in range(1, item_count + 1):
        line = lines[i]
        parts = line.split()
        items.append(Item(i - 1, int(parts[0]), int(parts[1])))

    # a trivial algorithm for filling the knapsack
    # it takes items in-order until the knapsack is full
    value = 0
    taken = [0] * len(items)

    all_combinations = (
        comb
        for n in range(1, len(items) + 1)
        for comb in combinations(items, n)
    )
    small_enough = (
        comb
        for comb in all_combinations
        if sum(item.weight for item in comb) <= capacity
    )
    winner = max(small_enough, key=lambda items: sum(i.value for i in items))
    value = sum(i.value for i in winner)

    for idx, item in enumerate(items):
        if item in winner:
            taken[idx] = 1

    # prepare the solution in the specified output format
    output_data = str(value) + " " + str(1) + "\n"
    output_data += " ".join(map(str, taken))
    return output_data


if __name__ == "__main__":
    import sys

    if len(sys.argv) > 1:
        file_location = sys.argv[1].strip()
        with open(file_location, "r") as input_data_file:
            input_data = input_data_file.read()

        if len(sys.argv) > 2:
            language = sys.argv[2].lower().strip()
            print(solve_it(input_data, language=language))
        else:
            print(solve_it(input_data))
    else:
        print(
            "This test requires an input file.  Please select one from the data directory. (i.e. python solver.py ./data/ks_4_0)"
        )
