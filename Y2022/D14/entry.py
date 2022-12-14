# https://adventofcode.com/2022/day/14
def prepareTask():
  data = open('./Y2022/D14/input')
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
    self.rockFormation: dict[tuple[int, int], str] = dict()
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
          self.rockFormation[(x, y)] = '#'
          x = to(x, previous[0])
        self.rockFormation[(x, y)] = '#'
        while y != previous[1]:
          self.rockFormation[(x, y)] = '#'
          y = to(y, previous[1])
        self.rockFormation[(x, y)] = '#'
      previous = [int(pos) for pos in point.split(',')]

  def readCaveScan(self, caveScan: list[str]):
    for rockFormation in caveScan:
      lines = rockFormation.split(' -> ')
      self.initRockLine(lines)
      

  def printCave(self):
    print(self.rockFormation)
    for line in range(0, self.deepest + 2):
      out = ""
      for col in range(self.outerEdges[0] - 2, self.outerEdges[1] + 3):
        if self.sandSpawn == (col, line):
          out += '+'
        else:
          try:
            out += self.rockFormation[(col, line)]
          except:
            out += '.'
      print(out)

def main(caveScan: list[str]):
  caveSystem = CaveSystem()
  caveSystem.readCaveScan(caveScan)
  caveSystem.printCave()

if __name__ == "__main__":
  caveScan = prepareTask()
  main(caveScan)