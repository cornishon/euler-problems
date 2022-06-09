import math
from itertools import product

def divisor_sum(n):
  res = {1}
  for k in range(2, int(math.sqrt(n)) + 1):
    d, m = divmod(n, k)
    if m == 0:
      res.add(k)
      res.add(d)
  return sum(res)
  
abundants = set()

N = 28123
for n in range(12, N):
  if divisor_sum(n) > n:
    abundants.add(n)
  
numbers = set(range(1, N))

for (a, b) in product(abundants, repeat=2):
  #ba = abundants.difference(set(range(a)))
  numbers.discard(a + b)
  #print(len(numbers))

answer = sum(numbers)

print(len(numbers))
print(answer)
