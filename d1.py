#!/usr/bin/python3
import sys

data = open('./data/1', 'r')
numeric_list = [int(x) for x in data.read().splitlines()]
magic_number = 2020

def entry(numbers, maxdepth, sum = 0, depth = 1, multiplicator = 1):
  """
  Evaluates, if for a given list of numbers is a subset that adds up to *magic_number* and
  and prints the multiplication of the subsets values  
  param "numbers": the set of numbers to search  
  param "maxdepth": How large the subset needs to be  
  optional param "sum": Holds the current sum of the calculation iteration  
  optional param "depth": used internally for keeping track of the depth  
  optional param "multiplicator": used internally for keeping track of the multiplication  
  """
  matches = [x for x in numbers if magic_number - sum >= x]
  for match in matches:
    if (sum + match < magic_number):
      entry([x for x in matches if x != match], maxdepth, sum + match, depth + 1, multiplicator*match)
    elif (sum + match == magic_number and depth == maxdepth):
      print( multiplicator*match )
      exit()

if __name__ == "__main__":
  entry(numeric_list, int(sys.argv[1]))