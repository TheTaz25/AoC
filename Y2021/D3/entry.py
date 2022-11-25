# https://adventofcode.com/2021/day/3

def prepareTask():
  raw = open('./Y2021/D3/input', 'r')
  return [list(line) for line in raw.read().splitlines()]

def binaryConverter(binaryData):
  exponent = 0
  result = 0

  for bit in binaryData:
    result = result + ((2 ** exponent) * bit)
    exponent = exponent + 1
  return result
  
def flipBits(binaryData):
  return [1 if bit == 0 else 0 for bit in binaryData]

def getDiagnosticDataSummary(diagnosticData: list[list[int]]):
  mostCommonBitsInDiagnosticData = [0] * len(diagnosticData[0])
  for datum in diagnosticData:
    mostCommonBitsInDiagnosticData = [bitTrack + int(dataBit) for bitTrack, dataBit in zip(mostCommonBitsInDiagnosticData, datum)]
  return mostCommonBitsInDiagnosticData

def calculateSubmarinePowerConsumption(diagnosticData: list[list[str]]):
  halfLengthOfDiagnosticData = len(diagnosticData) // 2

  mostCommonBitsInDiagnosticData = getDiagnosticDataSummary(diagnosticData)

  condensedDiagnosticData = [1 if positionBitCounter > halfLengthOfDiagnosticData else 0 for positionBitCounter in mostCommonBitsInDiagnosticData]
  # We want to start with the least significant bit when converting from binary data to decimal data
  condensedDiagnosticData.reverse()

  gammaRate = binaryConverter(condensedDiagnosticData)
  epsilonRate = binaryConverter(flipBits(condensedDiagnosticData))
  print("Power consumption is:", gammaRate * epsilonRate)

if __name__ == '__main__':
  diagnosticReport = prepareTask()
  calculateSubmarinePowerConsumption(diagnosticReport)