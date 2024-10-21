/***********************************************************************
 * Copyright (c)  2023, XXXX Co.,Ltd .
 * All rights reserved.
 * Filename    : driver_conf.h
 * Description : driver module configuration
 * Author(s)   : Robin Wan
 * version     : 1.0
 * Modify date : 2023-02-22
 ***********************************************************************/
#ifndef __DRV_CONF_H__
#define __DRV_CONF_H__

/* Macros --------------------------------------------------------P--------------------*/
#define DRV_UDMA_ENABLED
#define DRV_UART_ENABLED
//#define DRV_I2C_ENABLED
//#define DRV_SPIM_ENABLED
//#define DRV_SDHC_ENABLED
//#define DRV_CPI_ENABLED
//#define DRV_SPIS_ENABLED
//#define DRV_SCE_ENABLED
//#define DRV_UDC_ENABLED
//#define DRV_SDDC_ENABLED
#define DRV_RERAM_ENABLED

//#define SOFT_DIVISION  //use software division instead of SCE ALU ,delete for next mpw version

#endif /* __DRV_CONF_H__ */

