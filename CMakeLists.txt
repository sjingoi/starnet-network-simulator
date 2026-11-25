cmake_minimum_required(VERSION 3.22)
project(sim LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(ns3 CONFIG REQUIRED COMPONENTS 
    core
    network
    internet
    point-to-point
    csma
    applications
    internet-apps
)

add_executable(sim src/main.cc)

target_link_libraries(sim PRIVATE
    ns3::core
    ns3::network
    ns3::internet
    ns3::point-to-point
    ns3::csma
    ns3::applications
    ns3::internet-apps
)