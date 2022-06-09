N = 1000

def cycle_length(n):
  remainders = []
  for i in range(1, N):
    r = 10**i % n
    if r in remainders:
      break
    remainders.append(r)
  return len(remainders)

maxlen = 0
for i in range(N, 1 , -1):
  cl = cycle_length(i)
  maxlen = max(maxlen, cl)
  if maxlen >= i:
    break
  
print(maxlen + 1)