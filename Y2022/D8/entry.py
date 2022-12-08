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

def inRange(pos: int, higher: int):
  return pos >= 0 and pos < higher

def getSingleScenicScore(treeMap: list[str], x: int, y: int, h: int, func):
  score = 0
  nextX, nextY = func(x, y)
  while inRange(nextX, len(treeMap[0])) and inRange(nextY, len(treeMap)):
    score += 1
    pos = int(treeMap[nextX][nextY])
    if (pos >= h):
      return score
    nextX, nextY = func(nextX, nextY)
  return score

def getScenicScoresForTree(treeMap: list[str], x: int, y: int, h: int) -> list[int]:
  northernScore = getSingleScenicScore(treeMap, x, y, h, lambda x, y: [x - 1, y])
  easternScore = getSingleScenicScore(treeMap, x, y, h, lambda x, y: [x, y + 1])
  southernScore = getSingleScenicScore(treeMap, x, y, h, lambda x, y: [x + 1, y])
  westernScore = getSingleScenicScore(treeMap, x, y, h, lambda x, y: [x, y - 1])
  return [northernScore, easternScore, southernScore, westernScore]

def getHighestScenicScore(treeMap: list[str]):
  bestScenicScore = 0
  for xIndex, row in enumerate(treeMap):
    for yIndex, col in enumerate(row):
      northern, eastern, southern, western = getScenicScoresForTree(treeMap, xIndex, yIndex, int(col))
      calculatedScore = northern * eastern * southern * western
      if (calculatedScore > bestScenicScore):
        bestScenicScore = calculatedScore
  return bestScenicScore

if __name__ == '__main__':
  treeMap = prepareTask()
  formattedTreeMap = formatTreeMap(treeMap)

  visibleTreeCount = countVisibleTrees(formattedTreeMap, treeMap)
  print(f"{visibleTreeCount} trees are visible!")

  bestScenicScore = getHighestScenicScore(treeMap)
  print(f"The best scenic score is {bestScenicScore}")
