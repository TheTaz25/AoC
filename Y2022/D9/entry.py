# https://adventofcode.com/2022/day/9
import math

class Knot:
  def __init__(self) -> None:
    self.x = 0
    self.y = 0

  def touchesKnot(self, headKnot) -> bool:
    distance = math.sqrt(((headKnot.x - self.x) ** 2) + ((headKnot.y - self.y) ** 2))
    return distance < 2
  
  def leadKnotIsInSameRowOrCol(self, leadKnot) -> bool:
    return self.x == leadKnot.x or self.y == leadKnot.y

  def getPosition(self) -> tuple[int, int]:
    return (self.x, self.y)

  def moveInDirection(self, direction):
    x, y = direction
    self.x += x
    self.y += y

  def follow(self, leadKnot):
    if (not self.touchesKnot(leadKnot)):
      if (self.x != leadKnot.x):
        self.x += 1 if leadKnot.x > self.x else -1
      if (self.y != leadKnot.y):
        self.y += 1 if leadKnot.y > self.y else -1

class Rope:
  def __init__(self, knots) -> None:
    self.knots = [Knot() for _ in range(knots)]
    self.moveHeadMatrix = { 'R': (1, 0), 'L': (-1, 0), 'U': (0, 1), 'D': (0, -1) }

  def moveHead(self, direction):
    self.knots[0].moveInDirection(self.moveHeadMatrix[direction])
    for index, knot in enumerate(self.knots):
      if index == 0:
        continue
      knot.follow(self.knots[index - 1])
  
  def getTailPosition(self):
    return self.knots[-1].getPosition()

def prepareTask():
  data = open('./Y2022/D9/input2')
  return data.read().splitlines()

def calculateTailVisitFields(movement: list[str], ropeLength):
  rope = Rope(ropeLength)
  fieldsVisited = set([rope.getTailPosition()])
  for step in movement:
    direction, amount = step.split(" ")
    for _ in range(0, int(amount)):
      rope.moveHead(direction)
      fieldsVisited.add(rope.getTailPosition())
  return len(fieldsVisited)

if __name__ == '__main__':
  movements = prepareTask()
  rope2VisitedFields = calculateTailVisitFields(movements, 2)
  rope10VisitedFields = calculateTailVisitFields(movements, 10)

  print(f"Rope with 2 knots visits {rope2VisitedFields} unique locations")
  print(f"Rope with 10 knots visits {rope10VisitedFields} unique locations")