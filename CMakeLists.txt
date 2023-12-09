cmake_minimum_required(VERSION 3.0)
include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4.4
)
FetchContent_MakeAvailable(Corrosion)

project(armsim)
option(BUILD_SHARED_LIBS "Create kernel as shared/dynamic library." ON)
option(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS "Export all non-static symbols on Win32" ON)

corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml)

add_library(ask ask.c)
target_link_libraries(ask PUBLIC rust-armsim)
