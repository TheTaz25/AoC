from operator import attrgetter
# https://adventofcode.com/2022/day/12
def prepareTask():
  data = open('./Y2022/D12/input2')
  return [[c for c in line] for line in data.read().splitlines()]

class Location:
  def __init__(self, x: int, y: int, h: int) -> None:
    self.x = x
    self.y = y
    self.distanceToStart = None
    self.height = h
    self.char = chr(h+97)
    self.visited = False

  def markVisited(self):
    self.visited = True

  def getChar(self):
    return self.char

  def __str__(self) -> str:
    return f"Location({self.x},{self.y}) - height: {chr(self.height + 97)} - distance: {self.distanceToStart}"
  def __repr__(self) -> str:
    return f"Location({self.x},{self.y}) - height: {chr(self.height + 97)} - distance: {self.distanceToStart}\n"

  def setLocation(self, x: int, y: int):
    self.x = x
    self.y = y

  def setHeight(self, height: int):
    self.height = height

  def setDistanceToStart(self, distance: int) -> None:
    self.distanceToStart = distance

  def getDistanceTo(self, loc) -> int:
    return abs(loc.x - self.x) + abs(loc.y - self.y)

  def findNeighboursOf(self, listOfAll: list):
    adjacents = set([(self.x - 1, self.y), (self.x + 1, self.y), (self.x, self.y - 1), (self.x, self.y + 1)])
    return [neighbour for neighbour in listOfAll if not adjacents.isdisjoint(set([(neighbour.x, neighbour.y)]))]

def getAdjacentTiles(fromLocation: Location, heightMap: list[Location]):
  return [loc for loc in heightMap if loc.getDistanceTo(fromLocation) == 1 and loc.distanceToStart == None]

def mapDistancesToPosition(heightMap: list[Location], current: Location):
  currentInstance = Location(current.x, current.y, current.height)
  currentInstance.setDistanceToStart(0)
  allCurrents = [currentInstance]
  distance = 1
  while len(allCurrents) != 0:
    current = allCurrents.pop()
    neighbours = getAdjacentTiles(current, heightMap)
    [neighbour.setDistanceToStart(distance) for neighbour in neighbours]
    distance += 1
    allCurrents += neighbours

class HeightMap:
  def __init__(self, start: Location, end: Location, heightMap: list[Location], rows: int, cols: int) -> None:
    self.location = Location(start.x, start.y, start.height)
    self.target = Location(end.x, end.y, end.height)
    self.map = heightMap.copy()
    self.currentHeight = start.height
    self.distanceTravelled = 0
    self.rows = rows
    self.cols = cols

  def __str__(self) -> str:
    ret = ""
    transformedSet = dict[tuple[int, int], Location]()
    for loc in self.map:
      transformedSet[(loc.x, loc.y)] = loc
    for y in range(self.rows):
      for x in range(self.cols):
        loc = transformedSet.get((x, y))
        ret += loc.getChar()
      ret += f'\n'
    return ret

  def getLocationsWithHeight(self, height: int):
    print(f"retreive locations for height {chr(height + 97)}")
    return [loc for loc in self.map if loc.height == height]

  def calculateBestRouteToNextHeight(self):
    print(f"Current Height: {chr(self.currentHeight + 97)}")
    locationsToNextHeight = []
    # Where are we now? -> SELF
    # On what height are we? -> SELF
    # Get list of current heights
    while len(locationsToNextHeight) == 0:
      currentHeightMap = self.getLocationsWithHeight(self.currentHeight)
      print(f"Found {len(currentHeightMap)} locations")
      if 'i' == chr(self.currentHeight + 97):
        print(currentHeightMap)
      # print(currentHeightMap)
      # Calculate all distances on this height-list
      mapDistancesToPosition(currentHeightMap, self.location)
      # print(currentHeightMap)
      # check which heights are adjacent to the next height
      reachableFields = [height for height in currentHeightMap if height.distanceToStart != None]
      # print(reachableFields)
      locationsToNextHeight = self.getAllLocationsToNextHeight(reachableFields)
      print(len(locationsToNextHeight))
      if len(locationsToNextHeight) == 0:
        self.currentHeight -= 1
    # Choose the cheapest height
    [height.markVisited() for height in reachableFields]
    # print(locationsToNextHeight)
    bestNextLocation = min(locationsToNextHeight, key=attrgetter('distanceToStart'))
    print(f"Travel to x:{bestNextLocation.x}//y:{bestNextLocation.y}")
    # Update location and currentHeight
    self.location.setLocation(bestNextLocation.x, bestNextLocation.y)
    self.currentHeight += 1
    self.distanceTravelled += bestNextLocation.distanceToStart

  def getAllLocationsToNextHeight(self, currentHeights: list[Location]):
    nextHeights = self.getLocationsWithHeight(self.currentHeight + 1)
    print(f"Found {len(nextHeights)} locations")
    if 'i' == chr(self.currentHeight + 97):
        print(nextHeights)
    hits: list[Location] = []
    for height in nextHeights:
      # print(f"checking {height}")
      hits += height.findNeighboursOf(currentHeights)
    return hits

  def travelToEnd(self):
    while self.currentHeight != 25:
      self.calculateBestRouteToNextHeight()
      print(self)
    # self.calculateBestRouteToEnd()

def resolveMap(heightMap: list[list[str]]) -> HeightMap:
  mapInstance = []
  start = None
  end = None
  ordA = ord('a')
  cols = 0
  rows = len(heightMap)
  for y, line in enumerate(heightMap):
    cols = len(line)
    for x, height in enumerate(line):
      cur = None
      if (height == 'S'):
        cur = Location(x, y, ord('a') - ordA)
        cur.setDistanceToStart(0)
        start = cur
      elif (height == 'E'):
        cur = Location(x, y, ord('z') - ordA)
        end = cur
      else:
        cur = Location(x, y, ord(height) - ordA)
      mapInstance.append(cur)
  return HeightMap(start, end, mapInstance, rows, cols)

def part1(heightMap: HeightMap):
  heightMap.travelToEnd()
  print(heightMap.distanceTravelled)

if __name__ == '__main__':
  heightMap = prepareTask()
  mapDict = resolveMap(heightMap)
  part1(mapDict)
  # print(mapDict)