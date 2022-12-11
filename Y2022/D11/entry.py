# https://adventofcode.com/2022/day/11
class Test:
  def __init__(self, test: str, ifTrue: str, ifFalse: str) -> None:
    self.divisor = int(test.split()[-1])
    self.passToMonkeyIfTrue = int(ifTrue.split()[-1])
    self.passToMonkeyIfFalse = int(ifFalse.split()[-1])

  def executeTest(self, itemValue: int):
    if (itemValue % self.divisor == 0):
      return self.passToMonkeyIfTrue
    return self.passToMonkeyIfFalse
class Operation:
  def __init__(self, operationString: str) -> None:
    (op1, action, op2) = operationString.split('=')[1].strip().split()
    self.operator1 = None if op1 == 'old' else int(op1)
    self.operator2 = None if op2 == 'old' else int(op2)
    self.operator = getOperationLambda(action)
    self.actionChar = action

  def executeOperation(self, oldValue) -> int:
    op1 = self.operator1 or oldValue
    op2 = self.operator2 or oldValue
    return self.operator(op1, op2)
class Item:
  def __init__(self, worryLevel: int) -> None:
    self.worryLevel = worryLevel
  
  def applyOperation(self, operation: Operation) -> None:
    self.worryLevel = operation.executeOperation(self.worryLevel)

  def getBored(self):
    self.worryLevel = self.worryLevel // 3

  def applyLcm(self, lcm: int):
    self.worryLevel %= lcm
class Monkey:
  def __init__(self, rawDefinition: str) -> None:
    definitionPieces = rawDefinition.splitlines()
    (monkeyId, items, operation, test) = parseMonkeyDefinition(definitionPieces)
    self.items = items
    self.itemOperation = operation
    self.test = test
    self.id = monkeyId
    self.insepctedItems = 0

  def inspectAllItems(self, business):
    for item in self.items:
      self.insepctedItems += 1
      item.applyOperation(self.itemOperation)
      if business.boredom:
        item.getBored()
      else:
        item.applyLcm(business.lcm)
      throwItemToMonkey = self.test.executeTest(item.worryLevel)
      business.throwItemToMonkey(throwItemToMonkey, item)
    self.items.clear()

  def receiveThrownItem(self, item: Item):
    self.items.append(item)

  def printInspectionCount(self):
    print(f"Monkey {self.id} inspected items {self.insepctedItems} times.")
class MonkeyBusiness:
  def __init__(self, boredom: bool) -> None:
    self.monkeys: list[Monkey] = []
    self.boredom = boredom

  def setup(self, rawMonkeyData: str):
    monkeys = rawMonkeyData.split('\n\n')
    lcm = 1
    for monkeyData in monkeys:
      monkey = Monkey(monkeyData)
      lcm *= monkey.test.divisor
      self.monkeys.append(monkey)
    self.lcm = lcm

  def doRounds(self, amount: int):
    for _ in range(amount):
      self.doRound()
  
  def doRound(self):
    for monkey in self.monkeys:
      monkey.inspectAllItems(self)

  def throwItemToMonkey(self, toMonkey: int, item: Item):
    theMonkey = [monkey for monkey in self.monkeys if monkey.id == toMonkey][0]
    theMonkey.receiveThrownItem(item)

  def printMonkeyInspections(self):
    for monkey in self.monkeys:
      monkey.printInspectionCount()

  def getLevel(self):
    top2 = [monkey.insepctedItems for monkey in self.monkeys]
    top2.sort(reverse=True)
    return top2[0] * top2[1]

def getOperationLambda(char: str):
  if char == '*':
    return lambda x, y: x * y
  return lambda x, y: x + y

def parseMonkeyId(data: str):
  return int(data[:-1].split(' ')[1])

def parseMonkeyItems(data: str):
  return [Item(int(i.strip())) for i in data.split(':')[1].split(',')]

def parseMonkeyOperation(data: str):
  return Operation(data)

def parseMonkeyTest(test: str, trueStatement: str, falseStatement: str):
  return Test(test, trueStatement, falseStatement)

def parseMonkeyDefinition(pieces: list[str]):
  (idLine, itemLine, operationLine, testLine, trueLine, falseLine) = pieces
  return (
    parseMonkeyId(idLine),
    parseMonkeyItems(itemLine),
    parseMonkeyOperation(operationLine),
    parseMonkeyTest(testLine, trueLine, falseLine)
  )

def prepareTask():
  data = open('./Y2022/D11/input2', 'r')
  return data.read()

def calculate20RoundsWithBoredom(monkeyData: str):
  monkeyBusiness = MonkeyBusiness(True)
  monkeyBusiness.setup(monkeyData)
  monkeyBusiness.doRounds(20)
  # monkeyBusiness.printMonkeyInspections()
  monkeyBusinessLevel = monkeyBusiness.getLevel()
  print(f"After 20 rounds, the monkey business level is {monkeyBusinessLevel}")

def calculate10kRoundsWithoutBoredom(monkeyData: str):
  monkeyBusiness = MonkeyBusiness(False)
  monkeyBusiness.setup(monkeyData)
  monkeyBusiness.doRounds(10000)
  # monkeyBusiness.printMonkeyInspections()
  monkeyBusinessLevel = monkeyBusiness.getLevel()
  print(f"After 10000 rounds, the monkey business level is {monkeyBusinessLevel}")

def main():
  monkeyData = prepareTask()
  calculate20RoundsWithBoredom(monkeyData)
  calculate10kRoundsWithoutBoredom(monkeyData)  

if __name__ == '__main__':
  main()