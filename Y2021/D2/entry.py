# https://adventofcode.com/2021/day/2

def prepareTask():
  data = open('./Y2021/D2/input', 'r')
  return [x.split(' ') for x in data.read().splitlines()]

def executeTask1(instructions):
  depth = 0
  horizontal = 0
  for [instruction, value] in instructions:
    numeric = int(value)
    if instruction == 'forward':
      horizontal = horizontal + numeric
    elif instruction == 'down':
      depth = depth + numeric
    elif instruction == 'up':
      depth = depth - numeric
    else:
      raise Exception('Instruction is not covered', instruction)

  print("Depth is", depth, ". Distance traveled is", horizontal)
  print("Multiplied value is", depth * horizontal)

def executeTask2(instructions):
  depth = 0
  horizontal = 0
  aim = 0
  for [instruction, value] in instructions:
    numeric = int(value)
    if instruction == 'forward':
      horizontal = horizontal + numeric
      depth = depth + (aim * numeric)
    elif instruction == 'down':
      aim = aim + numeric
    elif instruction == 'up':
      aim = aim - numeric
    else:
      raise Exception('Instruction is not covered', instruction)

  print("Depth is", depth, ". Distance traveled is", horizontal)
  print("Multiplied value is", depth * horizontal)


if __name__ == '__main__':
  dataList = prepareTask()
  executeTask1(dataList)
  executeTask2(dataList)