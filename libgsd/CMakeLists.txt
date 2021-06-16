cmake_minimum_required(VERSION 3.0)
project(LibGSD C)
add_library(gsd STATIC gsd.c)
install(TARGETS gsd DESTINATION .)