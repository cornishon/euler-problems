import math

def divisor_sum(n):
  res = {1}
  for k in range(2, int(math.sqrt(n)) + 1):
    d, m = divmod(n, k)
    if m == 0:
      res.add(k)
      res.add(d)
  return sum(res)
  
amicables = []

for n in range(2, 10000):
  if n in amicables:
    continue
  m = divisor_sum(n)
  if m > 10000:
    continue
  if n != m and divisor_sum(m) == n:
    amicables.extend((m, n))
  

print(amicables)
print(sum(amicables))