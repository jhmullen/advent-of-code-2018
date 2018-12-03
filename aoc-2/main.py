with open("input.txt") as f:
  content = f.readlines()

content = [x.strip() for x in content] 

twos = 0
threes = 0

for id in content: 
  counts = {}
  for c in id:
    if c in counts.keys():
      counts[c] += 1
    else:
      counts[c] = 1
  if 2 in counts.values():
    twos += 1
  if 3 in counts.values():
    threes += 1

p1 = twos * threes

print("part one:", p1)
