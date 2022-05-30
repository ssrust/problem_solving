cmake_minimum_required(VERSION 3.22)
project(cpp_problem_solving)

set(CMAKE_CXX_STANDARD 20)

FILE(GLOB entries ${CMAKE_CURRENT_LIST_DIR}/p*)

FOREACH(entry ${entries})
    IF(IS_DIRECTORY ${entry})
        add_subdirectory(${entry})
    ENDIF()
ENDFOREACH()
