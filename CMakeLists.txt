cmake_minimum_required(VERSION 3.22)
project(sim LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.6.1   # pin a release
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH orbit_sim/Cargo.toml)

find_package(ns3 CONFIG REQUIRED COMPONENTS 
    core
    network
    internet
    point-to-point
    csma
    applications
    internet-apps
)

add_executable(sim src/main.cc src/ipv4-sat-routing.cc)

target_include_directories(sim PRIVATE orbit_sim/include)

target_link_libraries(sim PRIVATE
    orbit_sim
    ns3::core
    ns3::network
    ns3::internet
    ns3::point-to-point
    ns3::csma
    ns3::applications
    ns3::internet-apps
)