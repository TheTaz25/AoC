#!/bin/bash
echo "Enter the year which should be initialized"
read year

echo "Which programming language would you like to use? (specify the file ending like 'py' for python files)"
read lang

echo "Creating structure!"
mkdir "Y$year"

AOC_HOMEPAGE="https://adventofcode.com"

PY_COMMENT="#"
JS_COMMENT="//"

function getCommentForLang {
  case $lang in
    py)
      echo $PY_COMMENT
    ;;
    js | ts)
      echo $JS_COMMENT
    ;;
    *)
      return 1
    ;;
  esac
}

COMMENT=$(getCommentForLang)

cd "Y$year"
for t in {1..25}
do
  mkdir "D$t"
  cd "D$t"
  touch input
  echo "$COMMENT $AOC_HOMEPAGE/$year/day/$t" > "entry.$lang"
  cd ..
done