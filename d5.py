#!/bin/usr/python3

import math

data = open('./data/5', 'r')
dataList = data.read().splitlines()

def getSeat(row, e):
  s = 0
  for l in row:
    if (l == 'F' or l == 'L'):
      e = e - math.ceil((e - s) / 2)
    elif (l == 'B' or l == 'R'):
      s = s + math.ceil((e - s) / 2)
  return s

def entry(seatList):
  highestSeat = 0
  occupiedSeats = list()
  for seat in seatList:
    seatRow = getSeat(seat[:-3], 127)
    seatCol = getSeat(seat[-3:], 7)
    id = (seatRow * 8) + seatCol
    occupiedSeats.append(id)
    if (id > highestSeat):
      highestSeat = id
  print(highestSeat)
  occupiedSeats.sort()
  for x in range(len(occupiedSeats)):
    if (occupiedSeats[x + 1] - occupiedSeats[x] > 1):
      return occupiedSeats[x] + 1

if __name__ == "__main__":
  print("You are sitting on: ", entry(dataList))