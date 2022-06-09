import itertools

month_lenghts = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

month_lenghts_leap = month_lenghts
month_lenghts_leap[1] = 29

def is_leap(n):
  return ((n % 4 == 0 and n % 100 != 0)
    or (n % 400 == 0))
    
for yr in range(1900, 2001):
  if is_leap(yr):
    