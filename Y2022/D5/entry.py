# https://adventofcode.com/2022/day/5

class SupplyStack:
  def __init__(self, columns: int) -> None:
    self.columnCount = columns
    self.columns = dict[int, list[str]]()
    for stack in range(1, columns + 1):
      self.columns[stack] = []
  
  def queueStack(self, input: list[str]) -> None:
    crateCounter = 1
    for crate in input:
      if (crate != ' '):
        self.columns[crateCounter].insert(0, crate)
      crateCounter += 1

  def insertOperations(self, commandList: list[str]) -> None:
    self.commandList = [command.split(' ')[1::2] for command in commandList]
    
  def executeCommandsCreateMover9000(self):
    for command in self.commandList:
      (amount, source, destination) = [int(c) for c in command]
      self.executeCommandCreateMover9000(amount, source, destination)

  def moveCrateCreateMover9000(self, _from: int, _to: int) -> None:
    crate = self.columns[_from].pop()
    self.columns[_to] += crate

  def executeCommandCreateMover9000(self, amount: int, source: int, destination: int) -> None:
    for _ in range(amount):
      self.moveCrateCreateMover9000(source, destination)

  def getCratesOnTopOfStack(self) -> str:
    return [column[-1] for column in list(self.columns.values())]

  def moveCrateCrateMover9001(self, amount: int, source: int, destination: int) -> None:
    crates = self.columns[source][amount * -1::]
    for _ in range(amount):
      self.columns[source].pop()
    self.columns[destination] += crates

  def executeCommandsCrateMover9001(self) -> None:
    for command in self.commandList:
      (amount, source, destination) = [int(c) for c in command]
      self.moveCrateCrateMover9001(amount, source, destination)


def prepareTask():
  data = open('./Y2022/D5/input')
  extracted = data.read().splitlines()
  columns = (len(extracted[0]) + 1) // 4

  stack = SupplyStack(columns)

  currentLine = extracted[0]
  lineCounter = 0
  while(']' in currentLine):
    stack.queueStack(list(currentLine[1::4]))
    lineCounter += 1
    currentLine = extracted[lineCounter]
  
  lineCounter += 2
  stack.insertOperations(extracted[lineCounter::])
  return stack

def getTopOfStackCrates(stack: SupplyStack):
  stack.executeCommandsCreateMover9000()
  return stack.getCratesOnTopOfStack()

def getTopOfCratesViaCrateMover9001(stack: SupplyStack):
  stack.executeCommandsCrateMover9001()
  return stack.getCratesOnTopOfStack()

if __name__ == '__main__':
  stack = prepareTask()
  stack2 = prepareTask()

  topCrates = getTopOfStackCrates(stack)
  print("The re-arranged top crates are", "".join(topCrates), "(CrateMover9000)")

  correctTopCreates = getTopOfCratesViaCrateMover9001(stack2)
  print("The re-arranged top crates are", "".join(correctTopCreates), "(CrateMover9001)")