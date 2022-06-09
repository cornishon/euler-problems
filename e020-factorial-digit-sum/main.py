import math

def factorial(n):
  return math.prod(range(1, n+1))
    

def digit_sum(n):
  total = 0
  while n != 0:
    total += n % 10
    n //= 10
  return total
  
N = 100
print(digit_sum(factorial(N)))