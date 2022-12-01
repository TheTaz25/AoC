# https://adventofcode.com/2022/day/1

def prepareTask():
  data = open('./Y2022/D1/input', 'r')
  return "".join([x + ',' if x != '' else ';' for x in data.read().splitlines()]).split(';')

def calculateCaloriesPerElf(calorieList: list[str]):
  rawCalorieSummariesPerElf = [elf.split(',') for elf in calorieList]
  return [sum([int(datum) if datum != '' else 0 for datum in rawData]) for rawData in rawCalorieSummariesPerElf]

def getThreeTopCalorieElves(caloriesPerElf: list[int]):
  caloriesPerElf.sort()
  return caloriesPerElf[-3:]

if __name__ == '__main__':
  calorieList = prepareTask()

  calorieCountsPerElf = calculateCaloriesPerElf(calorieList)
  print("The highest calorie-count is", max(calorieCountsPerElf))

  topThreeElves = getThreeTopCalorieElves(calorieCountsPerElf)
  print("The top three elves have together", sum(topThreeElves), "calories")