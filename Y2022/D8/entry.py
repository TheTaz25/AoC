# https://adventofcode.com/2022/day/8

def prepareTask():
  data = open('./Y2022/D8/input')
  return data.read().splitlines()

def formatTreeMap(treeMap: list[str]) -> dict[int, list[int]]:
  treeList: dict[int, list[int]] = {}
  for i in range(0, 10):
    treeList[i] = []
  for x in range(0, len(treeMap)):
    for y in range(0, len(treeMap[0])):
      treeList[int(treeMap[x][y])].append([x, y])
  return treeList

def checkIfTreeIsVisible(treeDict: dict[int, list[int]], xPos: int, yPos: int, height: int):
  northFree = True
  eastFree = True
  southFree = True
  westFree = True
  for checkHeight in range(height, 10):
    trees = treeDict[checkHeight]
    if northFree:
      northFree = len([x for x, y in trees if x < xPos and y == yPos]) == 0
    if eastFree:
      eastFree = len([y for x ,y in trees if y > yPos and x == xPos]) == 0
    if southFree:
      southFree = len([x for x, y in trees if x > xPos and y == yPos]) == 0
    if westFree:
      westFree = len([y for x, y in trees if y < yPos and x == xPos]) == 0
    if (not northFree and not eastFree and not southFree and not westFree):
      return False
  return True

def countVisibleTrees(treeDict: dict[int, list[int]], treeMap: list[str]):
  visibleTreeCount = 0
  for xIndex, row in enumerate(treeMap):
    for yIndex, col in enumerate(row):
      if (checkIfTreeIsVisible(treeDict, xIndex, yIndex, int(col))):
        visibleTreeCount += 1
  return visibleTreeCount

if __name__ == '__main__':
  treeMap = prepareTask()
  formattedTreeMap = formatTreeMap(treeMap)

  visibleTreeCount = countVisibleTrees(formattedTreeMap, treeMap)
  print(f"{visibleTreeCount} trees are visible!")
