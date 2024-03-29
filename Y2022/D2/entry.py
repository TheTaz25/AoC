# https://adventofcode.com/2022/day/2

def prepareTask():
  data = open('./Y2022/D2/input', 'r')
  return [list(datum)[0::2] for datum in data.read().splitlines()]

def getFormValue(type: str):
  if (type == 'Y'):
    return 2
  if (type == 'X'):
    return 1
  return 3

def won(type: str):
  base = 6
  return getFormValue(type) + base

def draw(type: str):
  base = 3
  return getFormValue(type) + base

def loose(type: str):
  return getFormValue(type)

def getScore(opponent: str, player: str):
  lookup = {
    'AX': draw, 'AY': won, 'AZ': loose,
    'BX': loose, 'BY': draw, 'BZ': won,
    'CX': won, 'CY': loose, 'CZ': draw,
  }
  return lookup[opponent+player](player)

def getTotalScore(guide: list[list[str]]):
  calculatedScores = [getScore(opponent, player) for opponent, player in guide]
  return sum(calculatedScores)

def getReaction(opponent: str, outcome: str):
  lookup = {
    'AX': 'Z', 'AY': 'X', 'AZ': 'Y',
    'BX': 'X', 'BY': 'Y', 'BZ': 'Z',
    'CX': 'Y', 'CY': 'Z', 'CZ': 'X',
  }
  return lookup[opponent + outcome]


def getCounterMeasures(guide: list[list[str]]):
  realPlayStrategy = [[opponent, getReaction(opponent, outcome)] for opponent, outcome in guide]
  return realPlayStrategy

if __name__ == "__main__":
  rpsGuide = prepareTask()

  totalScore = getTotalScore(rpsGuide)
  print("The total imaginary score is", totalScore)

  counterMeasures = getCounterMeasures(rpsGuide)
  totalScore = getTotalScore(counterMeasures)
  print("The real total score is", totalScore)