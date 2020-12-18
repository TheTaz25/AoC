#!/usr/bin/python3
import re

data = open('./data/4', 'r')
passports = data.read().split('\n\n')

class Passport:
  def __init__(self, rawPassport: str):
    self.data = dict()
    sanitized = rawPassport.replace('\n', ' ')
    splitData = sanitized.split(' ')
    for dataPoint in splitData:
      prop, val = dataPoint.split(':')
      self.data[prop] = val

  def validateFieldsPresent(self):
    minKeys = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
    for key in minKeys:
      if (key not in self.data):
        return False
    return True

  def __validateByr(self):
    try:
      year = int(self.data['byr'])
      return 1920 <= year <= 2002
    except:
      return False

  def __validateIyr(self):
    try:
      year = int(self.data['iyr'])
      return 2010 <= year <= 2020
    except:
      return False

  def __validateEyr(self):
    try:
      year = int(self.data['eyr'])
      return 2020 <= year <= 2030
    except:
      return False

  def __validateHgt(self):
    cmRe = re.compile("^\d{3}cm$")
    inRe = re.compile("^\d{2}in$")
    try:
      if (cmRe.fullmatch(self.data['hgt'])):
        hgt = int(self.data['hgt'][0:-2])
        return 150 <= hgt <= 193
      elif (inRe.fullmatch(self.data['hgt'])):
        hgt = int(self.data['hgt'][0:-2])
        return 59 <= hgt <= 76
      else:
        return False
    except:
      return False

  def __validateHcl(self):
    hclRe = re.compile('^#[\d,a,b,c,d,e,f]{6}$')
    hcl = self.data['hcl']
    return hclRe.fullmatch(hcl)

  def __validateEcl(self):
    validColors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
    ecl = self.data['ecl']
    return ecl in validColors

  def __validatePid(self):
    pidRe = re.compile('^\d{9}$')
    pid = self.data['pid']
    return pidRe.fullmatch(pid)

  def validateIntegrity(self):
    if (not self.__validateByr()
    or not self.__validateIyr()
    or not self.__validateEyr()
    or not self.__validateHgt()
    or not self.__validateHcl()
    or not self.__validateEcl()
    or not self.__validatePid()):
      return False
    return True

def entry(rawPassportList):
  validCount = 0
  for passport in rawPassportList:
    passInstance = Passport(passport)
    if (passInstance.validateFieldsPresent()):
      if (passInstance.validateIntegrity()):
        validCount += 1
  return validCount

if __name__ == "__main__":
  print(entry(passports))