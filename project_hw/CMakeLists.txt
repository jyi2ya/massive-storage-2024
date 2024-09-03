cmake_minimum_required(VERSION 2.8)

set(CMAKE_C_STANDARD 99)

set(LIB_DIR ${PROJECT_SOURCE_DIR}/lib)
set(ALG_DIR ${PROJECT_SOURCE_DIR}/algorithm)

# 设置头文件目录变量
set(HEADER_DIRS
    ${LIB_DIR}
    ${ALG_DIR}
)

# 添加源文件变量
set(SOURCES
    ${PROJECT_SOURCE_DIR}/main.c
    ${ALG_DIR}/algorithm.c
    rust/liballoc.rlib.c
    rust/libcfg_if-0_1_10_H3.rlib.c
    rust/libcompiler_builtins-0_1_45_H92.rlib.c
    rust/libcore.rlib.c
    rust/libhashbrown-0_11_0_H328.rlib.c
    rust/liblibc-0_2_95_H19.rlib.c
    rust/libpanic_abort.rlib.c
    rust/librustc_demangle-0_1_19_H3.rlib.c
    rust/librustc_std_workspace_alloc-1_99_0.rlib.c
    rust/librustc_std_workspace_core-1_99_0.rlib.c
    rust/libstd.rlib.c
    rust/libstd_detect-0_1_5_Ha7.rlib.c
    rust/libunwind.rlib.c
)

# 包含头文件目录
include_directories(${HEADER_DIRS})

# 生成可执行文件
add_executable(project_hw  ${SOURCES})
target_link_libraries(project_hw PRIVATE ${PROJECT_SOURCE_DIR}/lib/libseek_model.so libutil.so librt.so libpthread.so libm.so libdl.so libc.so libgcc_s.so libatomic.so)

# 添加编译选项和宏定义
target_compile_options(project_hw PRIVATE -g -O3 -lm -ldl -std=gnu99 -ffunction-sections -pthread -fPIC)
target_compile_definitions(project_hw PRIVATE _GNU_SOURCE)
