# The Yoga side of souple-bench. Yoga's sources are READ from a checkout (nothing is written there);
# build out of tree, under crates/bench/yoga-build/ (git-ignored):
#
#   cmake -S crates/bench/yoga -B crates/bench/yoga-build/msvc -G "Visual Studio 16 2019" -A x64
#   cmake --build crates/bench/yoga-build/msvc --config Release
#
# The flags mirror the Rust side's release profile (opt-level 3, lto, codegen-units = 1): Yoga's own
# release optimisation (/O2, or -O2 for clang) plus whole-program optimisation over Yoga and the
# harness (/GL + /LTCG, or -flto), which is what Yoga's own CMake asks for (IPO on yogacore).
cmake_minimum_required(VERSION 3.20)
project(souple_yoga_bench CXX)

set(YOGA_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../../../../lecodes-next/engines/ui/libs/yoga"
    CACHE PATH "A Yoga checkout (read-only): the sources, nlohmann/json, benchmark/TreeDeserialization.cpp")
if(NOT EXISTS "${YOGA_ROOT}/yoga/Yoga.h")
  message(FATAL_ERROR "No Yoga at YOGA_ROOT=${YOGA_ROOT}")
endif()

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_BUILD_TYPE Release)
endif()

file(GLOB_RECURSE YOGA_SOURCES CONFIGURE_DEPENDS "${YOGA_ROOT}/yoga/*.cpp")
add_executable(yoga-bench bench.cpp "${YOGA_ROOT}/benchmark/TreeDeserialization.cpp" ${YOGA_SOURCES})
target_include_directories(yoga-bench PRIVATE "${YOGA_ROOT}" "${YOGA_ROOT}/lib")
set_property(TARGET yoga-bench PROPERTY INTERPROCEDURAL_OPTIMIZATION_RELEASE ON)

if(MSVC)
  # /O2 is Yoga's release flag (cmake/project-defaults.cmake); /GR- as Yoga (no RTTI).
  target_compile_options(yoga-bench PRIVATE /EHsc /GR- $<$<CONFIG:Release>:/O2 /Ob2>)
  # The deep synthetic trees recurse thousands of levels: a 1 GB stack, as the Rust side's thread.
  target_link_options(yoga-bench PRIVATE /STACK:1073741824)
  set(FLAGS "/O2 /Ob2 /GL /LTCG /GR- /EHsc, x64")
else()
  target_compile_options(yoga-bench PRIVATE -fno-rtti $<$<CONFIG:Release>:-O2>)
  target_link_options(yoga-bench PRIVATE -Wl,-z,stacksize=1073741824)
  set(FLAGS "-O2 -flto -fno-rtti")
endif()
if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang" AND MSVC)
  set(FLAGS "clang-cl /O2 -flto=thin /GR- /EHsc, x64")
endif()
target_compile_definitions(yoga-bench PRIVATE BENCH_FLAGS="${FLAGS}" NDEBUG $<$<BOOL:${MSVC}>:_CRT_SECURE_NO_WARNINGS>)
