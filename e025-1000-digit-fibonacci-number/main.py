from itertools import takewhile

def fibs():
  a, b = 0, 1
  while True:
    yield b
    a, b = b, a + b

fib_list = list(takewhile(
  lambda f: f < 10**999, fibs()))

ans = len(fib_list) + 1
print(ans)