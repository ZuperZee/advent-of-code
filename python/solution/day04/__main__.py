import time

import part1
import part2

with open("../input/day04", "r") as f:
    input = f.read()

start = time.time()
result = part1.parse(input)
end = time.time()
elapsed = end - start
print(result)
print(f"Part 1 result: {result}")
print(f"Executed in {elapsed}s")

start = time.time()
result = part2.parse(input)
end = time.time()
elapsed = end - start
print(result)
print(f"Part 2 result: {result}")
print(f"Executed in {elapsed}s")
