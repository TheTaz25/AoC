# https://adventofcode.com/2021/day/3

def prepareTask():
  data = open('./Y2021/D3/input', 'r')
  return [list(x) for x in data.read().splitlines()]

def binaryConverter(binaryData):
  exp = 0
  result = 0

  for bit in binaryData:
    result = result + ((2 ** exp) * bit)
    exp = exp + 1
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
  print(gammaRate * epsilonRate)

if __name__ == '__main__':
  diagnosticReport = prepareTask()
  calculateSubmarinePowerConsumption(diagnosticReport)