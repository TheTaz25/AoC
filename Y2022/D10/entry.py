# https://adventofcode.com/2022/day/10

def prepareTask():
  data = open('./Y2022/D10/input3')
  return data.read().splitlines()

class CentralProcessingUnit:
  def __init__(self, prog: list[str]) -> None:
    self.program = prog
    self.cycle = 1
    self.x = 1
    self.cycleState = "pre"
    self.operation = None
    self.line = -1
    self.endOfExecution = False

  def preOperation(self):
    self.cycleState = 'dur'

  def durOperation(self):
    self.cycleState = 'post'
    self.operation.execute(self)

  def postOperation(self):
    self.cycleState = 'pre'
    self.cycle += 1
    if (self.operation.tick()):
      self.getNextLine()

  def readInNextOperation(self, instructions: str):
    if instructions[0] == 'noop':
      self.operation = Noop()
    elif instructions[0] == 'addx':
      self.operation = AddX(int(instructions[1]))

  def getNextInstructionSet(self) -> list[str]:
    return self.program[self.line].split(' ')

  def getNextLine(self):
    self.line += 1
    if (len(self.program) <= self.line):
      self.endOfExecution = True
      return
    self.readInNextOperation(self.getNextInstructionSet())

  def tick(self):
    if self.cycleState == 'pre':
      self.preOperation()
    elif self.cycleState == 'dur':
      self.durOperation()
    elif self.cycleState == 'post':
      self.postOperation()

  def executeProgramAndGetSignalProbes(self, prober):
    probes = []
    self.getNextLine()
    while not self.endOfExecution:
      print(f"{self.cycleState} | {self.cycle}")
      if prober(self.cycleState, self.cycle):
        probes.append((self.cycle, self.x))
      self.tick()
    return probes

class Task:
  def execute(self, cpu: CentralProcessingUnit):
    pass
  def tick() -> bool:
    pass
class AddX(Task):
  def __init__(self, value: int) -> None:
    self.valueToAdd = value
    self.progress = 1

  def execute(self, cpu: CentralProcessingUnit):
    if (self.progress == 2):
      cpu.x += self.valueToAdd

  def tick(self):
    if (self.progress != 2):
      self.progress += 1
      return False
    return True

class Noop(Task):
  def __init__(self) -> None:
    pass

  def execute(self, _):
    pass

  def tick(self):
    return True

def bootStrap():
  summedUpProbes = 0
  prog = prepareTask()
  cpu = CentralProcessingUnit(prog)
  collected = cpu.executeProgramAndGetSignalProbes(lambda cycle, state: cycle == 'dur' and state % 20 == 0)[::2]
  for cycle, probe in collected:
    summedUpProbes += cycle * probe
  print("The sum of all probes is", summedUpProbes)

if __name__ == "__main__":
  bootStrap()