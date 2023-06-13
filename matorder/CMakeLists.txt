cmake_minimum_required(VERSION 3.10)

find_package(ArrayFire)
include_directories(${ArrayFire_INCLUDE_DIRS})

add_executable(order src/order.cpp)
option(DEVICE "AF backend to use" "CPU")

if(${DEVICE} STREQUAL "CPU")
    target_link_libraries(order ArrayFire::afcpu pthread)
elseif(${DEVICE} STREQUAL "CUDA")
    target_link_libraries(order ArrayFire::afcuda pthread)
elseif(${DEVICE} STREQUAL "OPENCL")
    target_link_libraries(order ArrayFire::afopencl pthread)
else()
    target_link_libraries(order ArrayFire::af pthread)
endif()