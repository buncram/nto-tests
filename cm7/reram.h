/***********************************************************************
 * Copyright (c)  2023, XXXX Co.,Ltd .
 * All rights reserved.
 * Filename    : reram.h
 * Description : reram driver
 * Author(s)   : xuliang
 * version     : 1.0
 * Modify date : 2023-02-02
 ***********************************************************************/
#ifndef __DRV_RERAM_H__
#define __DRV_RERAM_H__

#include <stdint.h>
#include "daric_cm7.h"
#include "drv_conf.h"

#define USE_LOOP_WRITE

#define REG8(addr)  (*(volatile uint8_t *)(addr))
#define REG16(addr) (*(volatile uint16_t *)(addr))
#define REG32(addr) (*(volatile uint32_t *)(addr))

#define RERAM_START (uint32_t) 0x60000000

#ifdef RERAM_TEST
#define RERAM_SIZE 0x100000
#else
// reram 总大小3M
#define RERAM_SIZE 0x400000
#endif

#define RERAM_END (RERAM_START + RERAM_SIZE)

#define RERAM_PAGE_SIZE 0x20 // 一块32个byte

#define RERAM_BLOCK_SIZE (0x20 * 0x20) // 32*32

#define RERAM_BLOCK_MAX 32 // 32*32

uint8_t ReramRead(uint32_t srcAddr, uint8_t *pRdBuf, uint32_t rdLen);
uint8_t ReramWrite(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen);

#endif
