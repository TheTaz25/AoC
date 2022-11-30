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

def getDiagnosticDataSummary(diagnosticData: list[list[str]]):
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

def getOxygenGeneratorRating(diagnosticData: list[list[str]]):
  candidates = diagnosticData
  # while len(candidates) != 1:
  #   pass
  return candidates[0]

def getCO2ScrubberRating(diagnosticData: list[list[str]]):
  candidates = diagnosticData
  currentPosition = 0
  while len(candidates) != 1:
    candidateSummary = getDiagnosticDataSummary(candidates)
    print("candidateSummary", candidateSummary[currentPosition])
    print("len of candidates", len(candidates))
    x = flipBits([1 if positionBitCounter >= len(candidates) // 2 else 0 for positionBitCounter in candidateSummary])
    print("x", x[currentPosition])
    candidates = [candidate for candidate in candidates if int(candidate[currentPosition]) == x[currentPosition]]
    currentPosition = currentPosition + 1
  print('candidate', candidates[0])
  return candidates[0]

def getLifeSupportReports(diagnosticData: list[list[str]]):
  oxygenGeneratorRating = getOxygenGeneratorRating(diagnosticData)
  cO2ScrubberRating = getCO2ScrubberRating(diagnosticData)

  print(oxygenGeneratorRating, cO2ScrubberRating)

  return [oxygenGeneratorRating, cO2ScrubberRating]

def calculateLifeSupportRating(diagnosticData: list[list[str]]):
  [oxygenGeneratorRating, cO2ScrubberRating] = getLifeSupportReports(diagnosticData)

  print("Current lifesupport-rating is:", oxygenGeneratorRating * cO2ScrubberRating)

if __name__ == '__main__':
  diagnosticReport = prepareTask()
  calculateSubmarinePowerConsumption(diagnosticReport)
  calculateLifeSupportRating(diagnosticReport)