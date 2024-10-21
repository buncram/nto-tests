

#ifndef __types_h__
#define __types_h__

#include <stdint.h>

typedef   signed           char INT8;
typedef   signed short     int  INT16;
typedef   signed           int  INT32;

/* exact-width unsigned integer types */
typedef unsigned           char UINT8;
typedef unsigned short     int  UINT16;
typedef unsigned           int  UINT32;

typedef unsigned           char BYTE;
typedef unsigned short     int  WORD;
typedef unsigned           int  DWORD;
typedef unsigned           char * PBYTE;
typedef unsigned short     int  * PWORD;
typedef unsigned           int  * PDWORD;

#define TRUE  1
#define FALSE 0

#define reg_write8(add, val) (*(volatile unsigned char *)(long)(add) = val)
#define reg_write16(add, val) (*(volatile unsigned short *)(long)(add) = val)
#define reg_write32(add, val) (*(volatile unsigned int *)(long)(add) = val)
#define reg_write(add, val) (*(volatile unsigned int *)(long)(add) = val)

#define reg_read8(add) (*(volatile unsigned char *)(long)(add))
#define reg_read16(add) (*(volatile unsigned short *)(long)(add))
#define reg_read32(add) (*(volatile unsigned int *)(long)(add))
#define reg_read(add) (*(volatile unsigned int *)(long)(add))


#endif




