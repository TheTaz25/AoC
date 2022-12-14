# https://adventofcode.com/2022/day/14
def prepareTask():
  data = open('./Y2022/D14/input2')
  return data.read().splitlines()

def to(src: int, trgt: int):
  if (src > trgt):
    return src - 1
  elif (src < trgt):
    return src + 1
  return src
class CaveSystem:
  def __init__(self) -> None:
    self.sandSpawn = (500, 0)
    self.rockFormation: list[tuple[int, int]] = list()
    self.sandLocations: list[tuple[int, int]] = list()
    self.deepest: int = 0
    self.outerEdges: tuple[int, int] = (500, 500)

  def applyCaveEdges(self, x: int, y: int):
    if y > self.deepest:
      self.deepest = y
    if x > self.outerEdges[1]:
      self.outerEdges = (self.outerEdges[0], x)
    if x < self.outerEdges[0]:
      self.outerEdges = (x, self.outerEdges[1])

  def initRockLine(self, lines: list[str]):
    previous = None
    for point in lines:
      x, y = [int(pos) for pos in point.split(',')]
      self.applyCaveEdges(x, y)
      if previous != None:
        while x != previous[0]:
          self.rockFormation.append((x, y))
          x = to(x, previous[0])
        self.rockFormation.append((x, y))
        while y != previous[1]:
          self.rockFormation.append((x, y))
          y = to(y, previous[1])
        self.rockFormation.append((x, y))
      previous = [int(pos) for pos in point.split(',')]

  def nextSandLocation(self, current: tuple[int, int]):
    down, downLeft, downRight = [(direction[0] + current[0], direction[1] + current[1]) for direction in [(0, 1), (-1, 1), (1, 1)]]
    if down not in self.rockFormation and down not in self.sandLocations:
      return down
    elif downLeft not in self.rockFormation and downLeft not in self.sandLocations:
      return downLeft
    elif downRight not in self.rockFormation and downRight not in self.sandLocations:
      return downRight
    else:
      return None

  def readCaveScan(self, caveScan: list[str]):
    for rockFormation in caveScan:
      lines = rockFormation.split(' -> ')
      self.initRockLine(lines)

  def letTheSandFlow(self):
    current = self.sandSpawn
    while current[1] < self.deepest:
      nextPosition = self.nextSandLocation(current)
      if nextPosition == None:
        self.sandLocations.append(current)
        current = self.sandSpawn
        continue
      current = nextPosition

  def printCave(self):
    for line in range(0, self.deepest + 2):
      out = ""
      for col in range(self.outerEdges[0] - 2, self.outerEdges[1] + 3):
        pos = (col, line)
        if self.sandSpawn == pos:
          out += '+'
        elif pos in self.rockFormation:
          out += '#'
        elif pos in self.sandLocations:
          out += 'o'
        else:
          out += '.'
      print(out)

def main(caveScan: list[str]):
  caveSystem = CaveSystem()
  caveSystem.readCaveScan(caveScan)
  caveSystem.printCave()
  caveSystem.letTheSandFlow()
  caveSystem.printCave()
  unitsOfSand = len(caveSystem.sandLocations)
  print(f"There are {unitsOfSand} Units of sand!")

if __name__ == "__main__":
  caveScan = prepareTask()
  main(caveScan)