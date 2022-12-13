# https://adventofcode.com/2022/day/13
from functools import cmp_to_key

def prepareTask():
  data = open('./Y2022/D13/input2')
  return data.read()

def evalPackets(left, right):
  l = eval(left) if type(left) == str else left
  r = eval(right) if type(right) == str else right
  if (isinstance(l, int) and isinstance(r, int)):
    return l - r
  elif (isinstance(l, int) and isinstance(r, list)):
    return evalPackets([l], r)
  elif (isinstance(l, list) and isinstance(r, int)):
    return evalPackets(l, [r])
  elif (isinstance(l, list) and isinstance(r, list)):
    for leftEntry, rightEntry in zip(l, r):
      interResult = evalPackets(leftEntry, rightEntry)
      if interResult:
        return interResult
    return len(l) - len(r)

def calculatePacketScore(packetPairs: list[str]):
  result = 0
  for index, pair in enumerate(packetPairs):
    left, right = pair.split('\n')
    if evalPackets(left, right) < 0:
      result += index + 1
  return result

def getDistressSignalDecoderKey(packetPairs: list[str]):
  dividerPackets = ["[[6]]", "[[2]]"]
  dividerPackets += packetPairs
  x = sorted(dividerPackets, key=cmp_to_key(evalPackets))
  divPacketIndex2 = x.index('[[2]]') + 1
  divPacketIndex6 = x.index('[[6]]') + 1
  return divPacketIndex2 * divPacketIndex6

if __name__ == '__main__':
  packetPairs = prepareTask()
  packetScore = calculatePacketScore(packetPairs.split('\n\n'))
  decoderKey = getDistressSignalDecoderKey([packet for packet in packetPairs.splitlines() if packet != ''])
  print(packetScore)
  print(decoderKey)