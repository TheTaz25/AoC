# https://adventofcode.com/2022/day/16
from copy import deepcopy

class Location:
  def __init__(self, name: str, valveFlow: int, connections: list[str]) -> None:
    self.name = name
    self.flow = valveFlow
    self.connections = connections

class Visitor:
  def __init__(self, start: str) -> None:
    self.current = start
    self.opened: list[Location] = []
    self.alreadyVisited = [start]
    self.cumulativeFlowRate = 0

  def tick(self):
    self.cumulativeFlowRate = sum([valve.flow for valve in self.opened])

  def addValve(self, valveLocation: Location):
    self.opened.append(valveLocation)

  def visit(self, next: str):
    self.current = next
class PipeStra:
  def __init__(self, start: Visitor, locations: list[Location], distance: int):
    self.visitors = [start]
    self.locations: dict[str, Location] = dict()
    for loc in locations:
      self.locations[loc.name] = loc
    self.eta = distance
    self.minute = 1
    self.valvesWithFlow = [loc for loc in locations if loc.flow > 0]

  def visit(self):
    while self.minute <= self.eta:
      print(f"Minute {self.minute}")
      for visitor in self.visitors:
        visitor.tick()
        currentLocation = self.locations.get(visitor.current)
        if (currentLocation.flow > 0 and currentLocation not in visitor.opened):
          visitor.addValve(currentLocation)
        else:
          if (currentLocation.connections[0] in visitor.alreadyVisited):
            continue
          visitor.visit(currentLocation.connections[0])
          for location in currentLocation.connections[1:]:
            newVisitor = deepcopy(visitor)
            newVisitor.visit(location)
            self.visitors.append(newVisitor)
      self.minute += 1


def strip(loc: str):
  if loc.endswith(','):
    return loc[0:-1]
  return loc

def prepareTask() -> tuple[str, int, list[str]]:
  data = open('./Y2022/D16/input')
  formattedValues: list[tuple[str, int, list[str]]] = []
  for valve in data.read().splitlines():
    splits = valve.split()
    formattedValues.append((
      splits[1],
      int(splits[4].split('=')[1][0:-1]),
      [strip(toValve) for toValve in splits[9:]]
    ))
  return formattedValues

def searchTheBestRoute(locations: list[Location]):
  starter = Visitor('AA')
  pipeline = PipeStra(starter, locations, 30)
  pipeline.visit()
  print([visitor.cumulativeFlowRate for visitor in pipeline.visitors])

if __name__ == "__main__":
  pipes = prepareTask()
  locations = [Location(name, flowRate, connects) for name, flowRate, connects in pipes]
  searchTheBestRoute(locations)
