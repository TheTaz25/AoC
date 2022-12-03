# https://adventofcode.com/2022/day/3
import re

def prepareTask():
  data = open('./Y2022/D3/input', 'r')
  return [[datum[0:len(datum)//2], datum[len(datum)//2::]] for datum in data.read().splitlines()]

def convertPriorities(rawPriorities: list[str]):
  a = ord('a') - 1
  return [ord(prio) - 38 if prio <= 'Z' else ord(prio) - a for prio in rawPriorities]

def getPrioritiesFromRucksacks(rucksacks: list[list[str]]):
  rawPriorities = [re.compile(r'['+re.escape(compartment1)+r']').search(compartment2).group() for [compartment1, compartment2] in rucksacks]
  return convertPriorities(rawPriorities)

if __name__ == '__main__':
  rucksacks = prepareTask()
  priorities = getPrioritiesFromRucksacks(rucksacks)
  print(sum(priorities))