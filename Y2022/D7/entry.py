# https://adventofcode.com/2022/day/7

class Blob:
  def __init__(self, type: str, name: str, location: list[str]):
    self.type = type
    self.name = name
    self.location = location.copy()
    if (type == 'dir'):
      self.location.append(name)
    self.size = 0

  def setSize(self, newSize: int):
    self.size = newSize

  def isFile(self) -> bool:
    return self.type == 'file'

  def isDir(self) -> bool:
    return self.type == 'dir'

  def addSize(self, other: int):
    self.size += other

  def isInDirectoryPath(self, path: list[str]):
    dirPath = "/".join(path)
    filePath = "/".join(self.location) or "/"
    if (dirPath in filePath):
      return True
    return False

  def __repr__(self):
    location = "/".join(self.location) or "/"
    return f"Blob({self.type}, {self.size}): {self.name or 'root'} in {location}\n"

class FileSystem:
  def __init__(self) -> None:
    self.diskSpace = 70000000
    self.pwd = ['']
    self.contents: list[Blob] = [Blob("dir", "", self.pwd.copy())]
  
  def cd(self, param: str):
    if param == '/':
      self.pwd = ['']
    elif param == '..':
      self.pwd.pop()
    else:
      self.pwd.append(param)

  def addFile(self, fileName: str, fileSize: int):
    newFile = Blob('file', fileName, self.pwd.copy())
    newFile.setSize(fileSize)
    self.contents.append(newFile)

  def addFolder(self, folderName: str):
    self.contents.append(Blob('dir', folderName, self.pwd.copy()))

  def calculateFolderSizes(self):
    for file in [file for file in self.contents if file.isFile()]:
      for dir in [dir for dir in self.contents if dir.isDir()]:
        if file.isInDirectoryPath(dir.location):
          dir.addSize(file.size)

  def getFolders(self):
    return [folder for folder in self.contents if folder.isDir()]

  def getUnusedSpace(self):
    return self.diskSpace - self.contents[0].size

def prepareTask():
  data = open('./Y2022/D7/input')
  return data.read().splitlines()

def executeFileSystemCommands(commands: list[str]) -> FileSystem:
  fs = FileSystem()
  for command in commands:
    params = command.split(' ')
    if params[0] == '$' and params[1] == 'cd':
      fs.cd(params[2])
    elif params[0] != '$' and params[0] == 'dir':
      fs.addFolder(params[1])
    elif params[0] != '$' and params[0] != 'dir':
      fs.addFile(params[1], int(params[0]))
  return fs

def getSumOfAllFoldersBelow100k(fs:FileSystem):
  fs.calculateFolderSizes()

  smallFolders = fs.getFolders()
  return sum([folder.size for folder in smallFolders if folder.size <= 100000])

def prepareFSForUpdate(fs: FileSystem):
  currentUnusedSpace = fs.getUnusedSpace()
  updateSize = 30000000
  minimumSpaceRequired = updateSize - currentUnusedSpace

  print(f"Currently unused space is {currentUnusedSpace} bytes, but update requires {updateSize} bytes")
  print(f"Detecting best folder to delete in order to free up space of minimum {minimumSpaceRequired}")

  eligableFolders = [folder for folder in fs.getFolders() if folder.size >= minimumSpaceRequired]
  eligableFolders.sort(key = lambda a : a.size)
  bestFolderToDelete = eligableFolders[0]

  return bestFolderToDelete

if __name__ == '__main__':
  cmdOutput = prepareTask()
  fs = executeFileSystemCommands(cmdOutput)
  smallFoldersCombinedSize = getSumOfAllFoldersBelow100k(fs)

  print(f"Small folders account for {smallFoldersCombinedSize}")
  bestFolderToDelete = prepareFSForUpdate(fs)
  print(f"The best folder has a size of {bestFolderToDelete.size}")
  print(bestFolderToDelete)

