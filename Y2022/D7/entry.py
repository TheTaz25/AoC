# https://adventofcode.com/2022/day/7


class Directory:
  def __init__(self, name: str):
    self.name = name
    self.size = 0
    self.contents: list[object] = []
  
  def __iadd__(self, other: object):
    self.size += other.getSize()
    self.contents.append(other)
    return self

  def getSize(self):
    return self.size

  def getSubDirectory(self, subDirectoryName):
    [subDirectory] = [obj for obj in self.contents if obj.name == subDirectoryName]
    if subDirectory != None:
      return subDirectory
    else:
      raise Exception(f"No subdirectory {subDirectoryName} found!")

  def getContents(self, tabs = 0):
    allContents = ("  " * tabs) + " ".join(["-", self.name, "(dir)", "\n"])
    allContents += "".join([content.getContents(tabs + 1) for content in self.contents])
    return allContents

class File:
  def __init__(self, name: str, size: int):
    self.name = name
    self.size = size

  def getSize(self):
    return self.size

  def getContents(self, tabs: int):
    return ("  " * tabs) + " ".join(["-", self.name, f"(file, size={str(self.size)})", "\n" ])

class FileSystem:
  def __init__(self):
    self.struct = Directory('/')
    self.current = ['/']

  def executeCommand(self, command: str):
    cmd = command.split(' ')
    if cmd[1] == 'cd':
      self.changeDirectory(cmd[2])
  
  def changeDirectory(self, into: str):
    if into == '/':
      self.current = []
    elif into == '..' and len(self.current) == 0:
      return
    elif into == '..':
      self.current.pop()
    else:
      self.current.append(into)
  
  def ls(self, entry: str):
    (dirOrFileSize, fileName) = entry.split(' ')
    if (dirOrFileSize == 'dir'):
      self.addDirToCurrentPath(fileName)
    else:
      self.addFileToCurrentPath(int(dirOrFileSize), fileName)
  
  def addDirToCurrentPath(self, dirName: str):
    currentDirectory = self.struct
    for path in self.current:
      currentDirectory = currentDirectory.getSubDirectory(path)
    currentDirectory += Directory(dirName)

  def addFileToCurrentPath(self, fileSize: int, fileName: str):
    currentDirectory = self.struct
    for path in self.current:
      currentDirectory = currentDirectory.getSubDirectory(path)
    currentDirectory += File(fileName, fileSize)

  def __str__(self) -> str:
    return self.struct.getContents()

def prepareTask():
  data = open('./Y2022/D7/input')
  return data.read().splitlines()

def printDirectory(commands: list[str]):
  fs = FileSystem()
  for command in commands:
    if (command.startswith('$')):
      fs.executeCommand(command)
    else:
      fs.ls(command)
  print(fs)

if __name__ == '__main__':
  cmdOutput = prepareTask()
  printDirectory(cmdOutput)