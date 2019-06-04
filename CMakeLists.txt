cmake_minimum_required(VERSION 3.5)
project(rj-debug)

if(NOT DEFINED ENV{JAVA_HOME})
    message(FATAL_ERROR "not define environment variable:JAVA_HOME")
endif()

if(UNIX)
    include_directories(
            $ENV{JAVA_HOME}/include
            $ENV{JAVA_HOME}/include/linux
    )
endif()

add_library(rjdebug SHARED src/debug.c)