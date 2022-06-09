from itertools import permutations

perm = list(permutations(range(10)))[999_999]

ans = int("".join([str(x) for x in perm]))

print(ans)