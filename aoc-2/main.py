with open("input.txt") as f:
  content = f.readlines()

content = [x.strip() for x in content] 

# part 1

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

# part 2

p2 = ''

for id1 in content:
  for id2 in content:
    list1 = list(id1)
    list2 = list(id2)
    wrong = 0
    i = 0
    while i < len(list1):
      if list1[i] != list2[i]:
        wrong += 1
        wrongindex = i
      i += 1
    if (wrong == 1):
      del list1[wrongindex]
      p2 = ''.join(list1)

print("part two:", p2)


