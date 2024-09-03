#define main main2
#include "solve.c"

#include <stdio.h>
#include <stdlib.h>
#include "algorithm.h"

/**
 * @brief  算法接口
 * @param  input            输入参数
 * @param  output           输出参数
 * @return int32_t          返回成功或者失败，RETURN_OK 或 RETURN_ERROR
 */
int32_t IOScheduleAlgorithm(const InputParam *input, OutputParam *output)
{
    solve((void *)input, (void *)output);
    return RETURN_OK;
}

/**
 * @brief  算法运行的主入口
 * @param  input            输入参数
 * @param  output           输出参数
 * @return uint32_t          返回成功或者失败，RETURN_OK 或 RETURN_ERROR
 */
int32_t AlgorithmRun(const InputParam *input, OutputParam *output)
{
    int32_t ret;

    ret = IOScheduleAlgorithm(input, output);

    return RETURN_OK;
}
