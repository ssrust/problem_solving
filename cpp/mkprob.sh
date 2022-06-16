#!/usr/bin/env bash

main()
{
  local pname="p$1"

  if [ -d "$pname" ]
    then echo "${pname} exists!"
    return
  fi

  mkdir "$pname"

  local template_cmake="
cmake_minimum_required(VERSION 3.22)
project($pname)

set(CMAKE_CXX_STANDARD 20)

add_executable($pname main.cpp)"

  local template_cpp="
#include <iostream>
#define FAST std::ios_base::sync_with_stdio(false), std::cin.tie(nullptr), std::cout.tie(nullptr)

int main() {
    FAST;

    return 0;
}"

  echo "$template_cmake" > "$pname/CMakeLists.txt"
  cp template.cpp "$pname/main.cpp"
}

if [ -z "$1" ]
then echo "mkprob.sh {problem_number}"
else main "$1"
fi