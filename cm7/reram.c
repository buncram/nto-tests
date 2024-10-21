#include <stdio.h>
#include <string.h>
#include "types.h"
#include "daric_util.h"
#include "reram.h"

uint8_t Reram_PAGE_Write(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen)
{
    uint8_t i, j;

    // 此条件为bootloader中的写入限制，防止破坏bootloader
    // if (wtLen != RERAM_PAGE_SIZE || (dstAddr < USR_RUNCOS_ADDR && g_writeFlag == FALSE) || dstAddr > RERAM_END)
    // {
    //     return FALSE;
    // }

    if (wtLen != RERAM_PAGE_SIZE || dstAddr > RERAM_END)
    {
        return FALSE;
    }

    for (i = 0, j = 0; i < 4; i++)
    {
        *((volatile uint64_t *)(dstAddr + (i * 0x8))) = *(volatile uint64_t *)(pWtBuf + j);
        j += 8;
        __DSB();
    }


    // step 2  config  reram write command mode
    *((volatile uint16_t *)0x40000000) = 0x2; // config write command mode
    __DSB();

    // step 2-1  send load command
    *((volatile uint32_t *)dstAddr) = 0x5200; // 5200  load cammand
    __DSB();

    // step 2-2  send write command
    *((volatile uint32_t *)dstAddr) = 0x9528; // 9528  write cammond
    __DSB();

    *((volatile uint16_t *)0x40000000) = 0x0; // clear write command mode
    __DSB();

    return TRUE;
}

// 最大一次性可以load RERAM_BLOCK_MAX(32)PAGE的数据
uint8_t ReramWrite_BLOCK_Load(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen)
{
    uint8_t i, j;

    // 此条件为bootloader中的写入限制，防止破坏bootloader
    // if (wtLen != RERAM_PAGE_SIZE || (dstAddr < USR_RUNCOS_ADDR && g_writeFlag == FALSE) || dstAddr > RERAM_END)
    // {
    //     return FALSE;
    // }

    if (wtLen != RERAM_PAGE_SIZE || dstAddr > RERAM_END)
    {
        return FALSE;
    }

    for (i = 0, j = 0; i < 4; i++)
    {
        *((volatile uint64_t *)(dstAddr + (i * 0x8))) = *(volatile uint64_t *)(pWtBuf + j);
        j += 8;
        __DSB();
    }

    // step 2  config  reram write command mode
    *((volatile uint16_t *)0x40000000) = 0x2; // config write command mode
    __DSB();

    // step 2-1  send load command
    *((volatile uint32_t *)dstAddr) = 0x5200; // 5200  load cammand
    __DSB();
    *((volatile uint16_t *)0x40000000) = 0x0; // clear write command mode
    __DSB();

    return TRUE;
}

uint8_t ReramWrite_BLOCK_Write(uint32_t dstAddr)
{
    // step 2  config  reram write command mode
    *((volatile uint16_t *)0x40000000) = 0x2; // config write command mode
    __DSB();

    // step 2-2  send write command
    *((volatile uint32_t *)dstAddr) = 0x9528; // 9528  write cammond
    __DSB();

    *((volatile uint16_t *)0x40000000) = 0x0; // clear write command mode
    __DSB();

    return TRUE;
}

uint8_t ReramRead(uint32_t srcAddr, uint8_t *pRdBuf, uint32_t rdLen)
{
    uint32_t i;

    for (i = 0; i < rdLen; i++)
    {
        *(pRdBuf + i) = REG8(srcAddr + i);
    }
    return TRUE;
}

uint8_t ReramWrite(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen)
{
    volatile uint32_t writeAddr, loopAddr, tempAddr;
    uint8_t oft;
    uint32_t len;
    uint8_t buf[RERAM_PAGE_SIZE], loop;
    uint8_t *p = pWtBuf;

    // 计算第一次写的长度，考虑是否存在跨页
    oft = dstAddr % RERAM_PAGE_SIZE;
    len = ((oft + wtLen) > RERAM_PAGE_SIZE) ? (RERAM_PAGE_SIZE - oft) : wtLen;

    // 将写起始地址转换为页的起始地址
    writeAddr = (uint32_t)(-1);
    writeAddr = dstAddr & (writeAddr - (RERAM_PAGE_SIZE - 1));

    // 读取起始页的数据内容至RAM中
    ReramRead(writeAddr, buf, RERAM_PAGE_SIZE);

    // 将对应的数据更新至RAM中
    memcpy(&buf[oft], p, len);
    p += len;

    // 更新reram数据
    if (Reram_PAGE_Write(writeAddr, buf, RERAM_PAGE_SIZE) == FALSE)
    {
        return FALSE;
    }

    // 若存在跨页，继续操作
    wtLen = wtLen - len;
    writeAddr += RERAM_PAGE_SIZE;

#ifdef USE_LOOP_WRITE // 计算当前块已经使用的block数
    loop = 0;
    loopAddr = (uint32_t)NULL;
    tempAddr = (uint32_t)(-1);
    tempAddr = writeAddr & (tempAddr - (RERAM_BLOCK_SIZE - 1)); // 块起始地址
    loop = (writeAddr - tempAddr) / RERAM_PAGE_SIZE;            // 块已写入的page数
#endif

    for (; wtLen > RERAM_PAGE_SIZE;)
    {
        // 准备写入的整页数据
        memcpy(buf, p, RERAM_PAGE_SIZE);

#ifdef USE_LOOP_WRITE
        if (loopAddr == (uint32_t)NULL)
        {
            loopAddr = writeAddr;
        }
        // 更新reram数据
        if (ReramWrite_BLOCK_Load(writeAddr, buf, RERAM_PAGE_SIZE) == FALSE)
        {
            return FALSE;
        }
        loop++;
        if (loop == RERAM_BLOCK_MAX)
        {
            if (ReramWrite_BLOCK_Write(loopAddr) == FALSE)
            {
                return FALSE;
            }
            loop = 0;
            loopAddr = (uint32_t)NULL;
        }
#else
        // 更新reram数据
        if (Reram_PAGE_Write(writeAddr, buf, RERAM_PAGE_SIZE) == FALSE)
        {
            return FALSE;
        }
#endif
        p += RERAM_PAGE_SIZE;
        wtLen -= RERAM_PAGE_SIZE;
        writeAddr += RERAM_PAGE_SIZE;
    }

#ifdef USE_LOOP_WRITE
    if (loop != 0)
    {
        if (ReramWrite_BLOCK_Write(loopAddr) == FALSE)
        {
            return FALSE;
        }
        loop = 0;
    }
#endif

    // 若存在尾页，则继续操作
    if (wtLen > 0)
    {
        // 读取尾页的数据内容至RAM中
        ReramRead(writeAddr, buf, RERAM_PAGE_SIZE);

        // 将对应的数据更新至RAM中
        memcpy(buf, p, wtLen);

        // 更新Flash目的页
        if (Reram_PAGE_Write(writeAddr, buf, RERAM_PAGE_SIZE) == FALSE)
        {
            return FALSE;
        }
    }

    return TRUE;
}

uint8_t Reram_BOOT_Write(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen)
{
    uint8_t i, j;

    if (wtLen != RERAM_PAGE_SIZE || ((dstAddr % RERAM_PAGE_SIZE) != 0))
    {
        return FALSE;
    }
    for (i = 0, j = 0; i < 4; i++)
    {
        *((volatile uint64_t *)(dstAddr + (i * 0x8))) = *(volatile uint64_t *)(pWtBuf + j);
        j += 8;
        __DSB();
    }

    // step 2  config  reram write command mode
    *((volatile uint16_t *)0x40000000) = 0x2; // config write command mode
    __DSB();

    // step 2-1  send load command
    *((volatile uint32_t *)dstAddr) = 0x5200; // 5200  load cammand
    __DSB();

    // step 2-2  send write command
    *((volatile uint32_t *)dstAddr) = 0x9528; // 9528  write cammond
    __DSB();

    *((volatile uint16_t *)0x40000000) = 0x0; // clear write command mode
    __DSB();

    return TRUE;
}
