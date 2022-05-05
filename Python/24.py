from itertools import permutations
def solve(n, digits):
    return list(permutations(digits))[n-1]


print(solve(1000000, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))

