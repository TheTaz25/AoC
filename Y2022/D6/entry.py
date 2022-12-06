# https://adventofcode.com/2022/day/6

def prepareTask():
  data = open('./Y2022/D6/input')
  return data.readline()

def getStartPositionInStream(packetData: str, offset = 4):
  currentPosition = 0
  window = set(packetData[0:offset])
  while len(window) != offset:
    currentPosition += 1
    window = set(packetData[currentPosition:currentPosition + offset])
  return currentPosition + offset

if __name__ == '__main__':
  data = prepareTask()

  startOfPacketMarker = getStartPositionInStream(data)
  print("The packet transition starts after position", startOfPacketMarker)

  startOfFirstMessageMarker = getStartPositionInStream(data, 14)
  print("The first message starts after position", startOfFirstMessageMarker)