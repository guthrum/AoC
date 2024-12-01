import sys
from collections import defaultdict
from typing import List, Tuple


def parse(loc: str) -> Tuple[List[int], List[int]]:
    numbers1 = []
    numbers2 = []
    for line in open(loc).read().splitlines():
        (f, s) = line.split("   ")
        numbers1.append(int(f))
        numbers2.append(int(s))
    return numbers1, numbers2

def part1(numbers1: List[int], numbers2: List[int]) -> int:
    return sum([abs(a - b) for (a, b) in zip(sorted(numbers1), sorted(numbers2))])

def part2(numbers1: List[int], numbers2: List[int]) -> int:
    counts = defaultdict(int)
    for n2 in numbers2:
        counts[n2] += 1
    return sum([n * counts[n] for n in numbers1])


if __name__ == '__main__':
    if len(sys.argv) != 2:
        exit(f'USAGE: {sys.argv[0]} <input file>')
    path = sys.argv[1]
    (numbers1, numbers2) = parse(path)
    print(f'Part 1: {part1(numbers1, numbers2)}')
    print(f'Part 2: {part2(numbers1, numbers2)}')