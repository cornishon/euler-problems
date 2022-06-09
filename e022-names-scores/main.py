with open('p022_names.txt', 'r') as f:
  names = [name.strip('"') for name in f.readline().split(',')]

names = sorted(names)

def letter_value(word):
  return sum(ord(c) - 64 for c in word)

print(names[937])
print(letter_value('COLIN'))

total_score = 0
for i, name in enumerate(names):
  total_score += (i+1) * letter_value(name)
  
print(total_score)