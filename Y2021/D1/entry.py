# https://adventofcode.com/2021/day/1
import sys

def prepareTask():
  data = open('./Y2021/D1/input', 'r')
  return [int(x) for x in data.read().splitlines()]

def executeTask1(dataList):
  increases = 0
  previous = dataList[0]
  for entry in dataList:
    if previous < entry:
      increases = increases + 1
    previous = entry

  print("The value increases", increases, "times")

def executeTask2(dataList):
  windows = lambda L, n: [L[i:i+n] for i in range(len(L) - n + 1)]
  executeTask1([sum(x) for x in windows(dataList, 3)])

if __name__ == '__main__':
  dataList = prepareTask()
  executeTask1(dataList)
  executeTask2(dataList)