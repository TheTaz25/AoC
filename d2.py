#!/usr/bin/python3

data = open('./data/2', 'r')
lines = data.read().splitlines()

def getPwData(paramters):
  rule, pw = paramters.split(':')
  numbers, letter = rule.split(' ')
  start, stop = [int(x) for x in numbers.split('-')]
  return pw, letter, start, stop

def old_entry(parameter_list):
  validPw = 0
  for line in parameter_list:
    pw, letter, start, stop = getPwData(line)
    letterCount = len([l for l in pw if l == letter])
    if (start <= letterCount <= stop):
      validPw = 1 + validPw
  return validPw

def new_entry(parameter_list):
  validPw = 0
  for line in parameter_list:
    pw, letter, start, stop = getPwData(line)
    first = pw[start]
    second = pw[stop]
    if (first != second and (first == letter or second == letter)):
      validPw = validPw + 1
  return validPw

if __name__ == "__main__":
  print(old_entry(lines), " passwords are valid according to the old rule")
  print(new_entry(lines), " passwords are valid according to the new rule")
  
