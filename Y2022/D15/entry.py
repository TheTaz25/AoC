# https://adventofcode.com/2022/day/15
import re

def calculateRadius(center: tuple[int, int], outer: tuple[int, int]):
  return abs(center[0] - outer[0]) + abs(center[1] - outer[1])
class Sensor:
  def __init__(self, sensorX: int, sensorY: int, beaconX: int, beaconY: int) -> None:
    self.position = {}
    self.position['x'] = sensorX
    self.position['y'] = sensorY
    self.radius = calculateRadius((sensorX, sensorY), (beaconX, beaconY))

  def isCoveringRow(self, row: int) -> bool:
    return abs(self.position['y'] - row) <= self.radius

  def getCoverageOnRow(self, row: int) -> list[tuple[int, int]]:
    coveredSpots = [(self.position['x'], row)]
    radialWidth = abs(abs(self.position['y'] - row) - self.radius)
    for radialWidth in range(1, radialWidth + 1):
      coveredSpots += [(self.position['x'] + radialWidth, row), (self.position['x'] - radialWidth ,row)]
    return coveredSpots

  def getCoverageBoundariesOnRow(self, row: int) -> tuple[int, int]:
    radialWidth = abs(abs(self.position['y'] - row) - self.radius)
    return (self.position['x'] - radialWidth, self.position['x'] + radialWidth)
  
  def locationIsCoveredBySensor(self, location: tuple[int, int]):
    distance = calculateRadius((self.position['x'], self.position['y']), location)
    return distance <= self.radius

  def __repr__(self) -> str:
    return f"Sensor at x={self.position['x']}, y={self.position['y']}: radius is {self.radius}\n"

def prepareTask() -> list[list[int]]:
  data = open('./Y2022/D15/input2')
  r = re.compile("^Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)$")
  matches = [r.match(line) for line in data.read().splitlines()]
  return [[int(t) for t in match.groups()] for match in matches]

def initializeSensors(sensoryData: list[list[int]]):
  return [Sensor(sX, sY, bX, bY) for sX, sY, bX, bY in sensoryData]

def getBeaconLocations(sensoryData: list[list[int]]):
  return [(bX, bY) for _, _, bX, bY in sensoryData]

def getCoverageOnRow(sensors: list[Sensor], row: int):
  beaconCoverageOnLine: set[tuple[int, int]] = set()
  for sensor in sensors:
    if (sensor.isCoveringRow(row)):
      [beaconCoverageOnLine.add(cover) for cover in sensor.getCoverageOnRow(row)]
  return beaconCoverageOnLine

def calculateBoundariesForRow(sensors: list[Sensor], row: int):
  boundaries: list[tuple[int, int]] = []
  for sensor in sensors:
    if sensor.isCoveringRow(row):
      f = sensor.getCoverageBoundariesOnRow(row)
      boundaries.append(f)
  return boundaries

def findUncoveredSpotInRow(boundaries: list[tuple[int, int]]):
  uncoveredSpot = 0
  for boundMin, boundMax in boundaries:
      if boundMin <= uncoveredSpot + 1:
        if boundMax > uncoveredSpot:
          uncoveredSpot = boundMax
      else:
        return uncoveredSpot + 1
  return None

def searchTheHole(sensors: list[Sensor], maxRange: int):
  for row in range(0, maxRange + 1):
    boundaries = calculateBoundariesForRow(sensors, row)
    boundaries.sort(key= lambda x: x[0])

    potentialSpotNotCovered = findUncoveredSpotInRow(boundaries)
    if potentialSpotNotCovered != None:
      return (potentialSpotNotCovered, row)

if __name__ == "__main__":
  # Prepare data
  sensoryData = prepareTask()
  beacons = getBeaconLocations(sensoryData)
  sensors = initializeSensors(sensoryData)

  # Part 1
  ROW = 2000000
  coveredArea = getCoverageOnRow(sensors, ROW)
  print(f"In row {ROW}, {len(coveredArea.difference(beacons))} positions cannot contain a beacon")

  # Part 2
  MAX_DIMENSIONS = 4000000
  uncoveredPosition = searchTheHole(sensors, MAX_DIMENSIONS)
  tuningFrequency = uncoveredPosition[0] * MAX_DIMENSIONS + uncoveredPosition[1]
  print(f"The tuning frequency of the beacon in question is {tuningFrequency}")