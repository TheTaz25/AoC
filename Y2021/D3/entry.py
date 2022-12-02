# https://adventofcode.com/2021/day/3

def prepareTask():
  raw = open('./Y2021/D3/input', 'r')
  return [line for line in raw.read().splitlines()]

def getBitCountOnPosition(dataList: list[str], position: str):
  ones = 0
  zeroes = 0
  for entry in dataList:
    if int(entry, 2) & int(position, 2):
      ones = ones + 1
    else:
      zeroes = zeroes + 1
  return [ones, zeroes]

def getPowerConsumption(report: list[str]):
  position = bin(1 << len(report[0]) - 1)
  binaryReport = [bin(int(datum, 2)) for datum in report]
  while position:
    position >> 1
  print(position)
  print(binaryReport)
  print(getBitCountOnPosition(binaryReport, position))
  pass

if __name__ == '__main__':
  diagnosticReport = prepareTask()
  print(diagnosticReport)

  getPowerConsumption(diagnosticReport)