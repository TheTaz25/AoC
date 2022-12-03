# https://adventofcode.com/2022/day/3
import re

def prepareTask():
  data = open('./Y2022/D3/input', 'r')
  return [datum for datum in data.read().splitlines()]

def convertPriorities(rawPriorities: list[str]):
  a = ord('a') - 1
  return [ord(prio) - 38 if prio <= 'Z' else ord(prio) - a for prio in rawPriorities]

def getPrioritiesFromRucksacks(rucksacks: list[list[str]]):
  rawPriorities = [re.compile(r'['+re.escape(compartment1)+r']').search(compartment2).group() for [compartment1, compartment2] in rucksacks]
  return convertPriorities(rawPriorities)

def getCommonLetterInTriplet(triplet: list[str]):
  one, two, three = [set(list(section)) for section in triplet]
  return list(one.intersection(two).intersection(three))[0]

def getElfGroupPriorities(rucksacks: list[str]):
  triplets = [rucksacks[offset:offset+3] for offset in range(0, len(rucksacks), 3)]
  return convertPriorities([getCommonLetterInTriplet(triplet) for triplet in triplets])

if __name__ == '__main__':
  rucksacks = prepareTask()

  priorities = getPrioritiesFromRucksacks([[sack[0:len(sack)//2], sack[len(sack)//2::]] for sack in rucksacks])
  print("The sum of all priorities is:", sum(priorities))

  elfGroupPriorities = getElfGroupPriorities(rucksacks)
  print("The sum of all group priorities is:", sum(elfGroupPriorities))
