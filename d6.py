#!/bin/usr/python3

data = open('./data/6', 'r')
formData = data.read().split('\n\n');

def entry(dataList):
  sum = 0
  allSum = 0
  for q in dataList:
    sum += len(set([x for x in q.replace('\n', '')]))
    groupSize = len(q.splitlines())
    qDict = dict()
    for letter in [x for x in q.replace('\n', '')]:
      if (letter not in qDict):
        qDict[letter] = 1
      else:
        qDict[letter] += 1
    allSum += len([x for x in qDict if qDict[x] == groupSize])
  return sum, allSum


if __name__ == "__main__":
  print(entry(formData))