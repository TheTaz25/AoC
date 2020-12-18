#!/bin/usr/python3

data = open('./data/3', 'r')
lines = data.read().splitlines()

def entry(parameter_list, right, down):
  treeCount = 0
  currentX = 0
  maxLen = len(parameter_list[0])
  for i in parameter_list[::down]:
    if (i[currentX] == '#'):
      treeCount = treeCount + 1
    currentX = (currentX + right) % maxLen
  return treeCount

if __name__ == "__main__":
  s1 = entry(lines, 1, 1)
  s2 = entry(lines, 3, 1)
  s3 = entry(lines, 5, 1)
  s4 = entry(lines, 7, 1)
  s5 = entry(lines, 1, 2)
  print(s1, s2, s3, s4, s5)
  print(s1*s2*s3*s4*s5)