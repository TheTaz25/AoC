# https://adventofcode.com/2022/day/4

def prepareTask():
  data = open('./Y2022/D4/input')
  return [pair.split(',') for pair in data.read().splitlines()]

def getRange(rawRange: str):
  start, end = rawRange.split('-')
  return [int(start), int(end)]

def convertRanges(assignment1, assignment2):
  range1Start, range1End = getRange(assignment1)
  range2Start, range2End = getRange(assignment2)
  range1 = set(range(range1Start, range1End + 1)) if range1Start != range1End else set([range1Start])
  range2 = set(range(range2Start, range2End + 1)) if range2Start != range2End else set([range2Start])
  return [range1, range2]

def calculateFullOverlaps(assignments: list[list[set[int]]]):
  overlaps = 0
  for range1, range2 in assignments:
    if (range1.issubset(range2) or range2.issubset(range1)):
      overlaps = overlaps + 1

  return overlaps

def calculatePartialOverlaps(assignments: list[list[set[int]]]):
  overlaps = 0
  for range1, range2 in assignments:
    if (not range1.isdisjoint(range2)):
      overlaps += 1
  return overlaps

if __name__ == '__main__':
  assignments = prepareTask()
  rangedAssigments = [convertRanges(assignment1, assignment2) for assignment1, assignment2 in assignments]

  fullOverlaps = calculateFullOverlaps(rangedAssigments)
  print(fullOverlaps, "assignments are fully overlapping")

  partialOverlaps = calculatePartialOverlaps(rangedAssigments)
  print(partialOverlaps, "assignments do partial overlap")