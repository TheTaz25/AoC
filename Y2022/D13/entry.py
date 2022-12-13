# https://adventofcode.com/2022/day/13

def prepareTask():
  data = open('./Y2022/D13/input2')
  return data.read().split('\n\n')

def evalPackets(left: list or int, right: list or int):
  if (isinstance(left, int) and isinstance(right, int)):
    return left - right
  elif (isinstance(left, int) and isinstance(right, list)):
    return evalPackets([left], right)
  elif (isinstance(left, list) and isinstance(right, int)):
    return evalPackets(left, [right])
  elif (isinstance(left, list) and isinstance(right, list)):
    for leftEntry, rightEntry in zip(left, right):
      interResult = evalPackets(leftEntry, rightEntry)
      if interResult:
        return interResult
    return len(left) - len(right)

def calculatePacketScore(packetPairs: list[str]):
  result = 0
  for index, pair in enumerate(packetPairs):
    left, right = pair.split('\n')
    if evalPackets(eval(left), eval(right)) < 0:
      result += index + 1
  return result

if __name__ == '__main__':
  packetPairs = prepareTask()
  packetScore = calculatePacketScore(packetPairs)
  print(packetScore)