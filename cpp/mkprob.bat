@echo off

IF not [%1] == [] (
GOTO Main
)

:NoArg
echo "mkprob.bat {problem_number}"
EXIT /B -1

:Main
setlocal EnableDelayedExpansion

set pname=p%1
if exist %pname% (
    echo %pname% exists
    EXIT /B 0
)
mkdir %pname%

set LF=^


set template_cmake=cmake_minimum_required(VERSION 3.22)!LF!^
project(%pname%)!LF!^
!LF!^
set(CMAKE_CXX_STANDARD 20)!LF!^
!LF!^
add_executable(%pname% main.cpp)

echo !template_cmake! > %pname%\CMakeLists.txt
copy template.cpp %pname%\main.cpp
endlocal
EXIT /B 0
