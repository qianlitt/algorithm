#!/usr/bin/env bash

CXX="clang++"
CXXFLAGS="-std=c++23 -Wall -Wno-deprecated -O2 -I./include"

# 目录
SRC_DIR="src"
BUILD_DIR="build"
BIN_DIR="${BUILD_DIR}/bin"
OBJ_DIR="${BUILD_DIR}/obj"
TEMPLATE_DIR="template"

# 子目录
SRC_408_DIR="${SRC_DIR}/408"
BIN_408_DIR="${BIN_DIR}/408"
SRC_LEETCODE_DIR="${SRC_DIR}/leetcode"
BIN_LEETCODE_DIR="${BIN_DIR}/leetcode"

handle_408_question() { # 408 题目字符串拆分
  local input=$1
  chapter=$(printf "%01d" "${input%/*}")
  question=$(printf "%02d" "${input#*/}")
}

test_408() {
  handle_408_question $1

  mkdir -p $BIN_408_DIR
  $CXX $CXXFLAGS $SRC_408_DIR/chapter$chapter/$question.cpp -o $BIN_408_DIR/$chapter-$question && $BIN_408_DIR/$chapter-$question
}

test_leetcode() {
  for file in $SRC_LEETCODE_DIR/*.cpp; do
    if [[ $file =~ ^.*/$1\..*\.cpp$ ]]; then
      # 匹配形如 1.two-sum.cpp 的文件
      mkdir -p $BIN_LEETCODE_DIR
      $CXX $CXXFLAGS $file -o $BIN_LEETCODE_DIR/$1 && $BIN_LEETCODE_DIR/$1
    fi
  done
}

creat_file() {
  echo "选择模板："
  echo "1) 408"
  echo "2) leetcode"
  read type

  read -p "题目：" question
  case $type in
    1)
      handle_408_question $question

      mkdir -p $SRC_408_DIR/chapter$chapter
      cp $TEMPLATE_DIR/408-template.cpp $SRC_408_DIR/chapter$chapter/$question.cpp
      ;;
    2)
      cp $TEMPLATE_DIR/$leetcode-template.cpp $SRC_LEETCODE_DIR/$question.cpp
      ;;
  esac
}

case $1 in
  408) test_408 $2;;
  leet) test_leetcode $2;;
  new) creat_file;;
  clean) rm -rf $BUILD_DIR;;
esac