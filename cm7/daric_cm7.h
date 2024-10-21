#ifndef __DARIC_CM7_H__
#define __DARIC_CM7_H__

/*
 * ==========================================================================
 * ---------- Interrupt Number Definition -----------------------------------
 * ==========================================================================
 */

typedef enum IRQn
{
    /******  Cortex M7 Processor Exceptions Numbers **************************************************/
    NonMaskableInt_IRQn        = -14,    /*!< -14 Non Maskable Interrupt                              */
    HardFault_IRQn             = -13,    /*!< -13 Cortex M7 Hard Fault Interrupt                     */
    MemoryManagement_IRQn      = -12,    /*!< -12  Memory Management, MPU mismatch, including Access Violation and No Match   */
    BusFault_IRQn              = -11,    /*!< -11  Bus Fault, Pre-Fetch-, Memory Access Fault, other address/memory related Fault */
    UsageFault_IRQn            = -10,     /*!< -10  Usage Fault, i.e. Undef Instruction, Illegal State Transition  */
    SVCall_IRQn                = -5,     /*!< 11 Cortex M7 SV Call Interrupt                       */
    DebugMonitor_IRQn          = -4,      /*!< -4 Debug Monitor                                                          */
    PendSV_IRQn                = -2,      /*!< -2 Pendable request for system service                                    */
    SysTick_IRQn               = -1,      /*!< -1 System Tick Timer  */

    /******  Daric Cortex M7 specific Interrupt Numbers ******************************************/
    //sce
    //SCE_IRQ00_IRQn             = 32 + 16,
    //SCE_IRQ01_IRQn             = 33 + 16,
    //SCE_IRQ02_IRQn             = 34 + 16,
    //SCE_IRQ03_IRQn             = 35 + 16,
    //SCE_IRQ04_IRQn             = 36 + 16,
    //SCE_IRQ05_IRQn             = 37 + 16,
    //SCE_IRQ06_IRQn             = 38 + 16,
    //SCE_IRQ07_IRQn             = 39 + 16,
    //SCE_IRQ08_IRQn             = 40 + 16,
    //SCE_IRQ09_IRQn             = 41 + 16,
    //SCE_IRQ10_IRQn             = 42 + 16,
    //SCE_IRQ11_IRQn             = 43 + 16,
    //SCE_IRQ12_IRQn             = 44 + 16,
    //SCE_IRQ13_IRQn             = 45 + 16,
    //SCE_IRQ14_IRQn             = 46 + 16,
    //SCE_IRQ15_IRQn             = 47 + 16,
    //SCE_IRQ16_IRQn             = 48 + 16,
    //SCE_IRQ17_IRQn             = 49 + 16,
    //SCE_IRQ18_IRQn             = 50 + 16,
    //SCE_IRQ19_IRQn             = 51 + 16,
    //SCE_IRQ20_IRQn             = 52 + 16,
    //SCE_IRQ21_IRQn             = 53 + 16,
    //SCE_IRQ22_IRQn             = 54 + 16,
    //SCE_IRQ23_IRQn             = 55 + 16,
    //SCE_IRQ24_IRQn             = 56 + 16,
    //SCE_IRQ25_IRQn             = 57 + 16,
    //SCE_IRQ26_IRQn             = 58 + 16,
    //SCE_IRQ27_IRQn             = 59 + 16,
    //SCE_IRQ28_IRQn             = 60 + 16,
    //SCE_IRQ29_IRQn             = 61 + 16,
    //SCE_IRQ30_IRQn             = 62 + 16,
    //SCE_IRQ31_IRQn             = 63 + 16,

    /* udma peripherals */

    //uarts
    UART0_IRQ_RX_IRQn           = 64 + 16,
    UART0_IRQ_TX_IRQn           = 65 + 16,
    UART0_IRQ_POLL_IRQn         = 66 + 16,
    UART0_IRQ_EOT_IRQn          = 67 + 16,
    UART1_IRQ_RX_IRQn           = 68 + 16,
    UART1_IRQ_TX_IRQn           = 69 + 16,
    UART1_IRQ_POLL_IRQn         = 70 + 16,
    UART1_IRQ_EOT_IRQn          = 71 + 16,
    UART2_IRQ_RX_IRQn           = 72 + 16,
    UART2_IRQ_TX_IRQn           = 73 + 16,
    UART2_IRQ_POLL_IRQn         = 74 + 16,
    UART2_IRQ_EOT_IRQn          = 75 + 16,
    UART3_IRQ_RX_IRQn           = 76 + 16,
    UART3_IRQ_TX_IRQn           = 77 + 16,
    UART3_IRQ_POLL_IRQn         = 78 + 16,
    UART3_IRQ_EOT_IRQn          = 79 + 16,

    //spims
    SPIM0_IRQ_RX_IRQn           = 80 + 16,
    SPIM0_IRQ_TX_IRQn           = 81 + 16,
    SPIM0_IRQ_POLL_IRQn         = 82 + 16,
    SPIM0_IRQ_EOT_IRQn          = 83 + 16,
    SPIM1_IRQ_RX_IRQn           = 84 + 16,
    SPIM1_IRQ_TX_IRQn           = 85 + 16,
    SPIM1_IRQ_POLL_IRQn         = 86 + 16,
    SPIM1_IRQ_EOT_IRQn          = 87 + 16,
    SPIM2_IRQ_RX_IRQn           = 88 + 16,
    SPIM2_IRQ_TX_IRQn           = 89 + 16,
    SPIM2_IRQ_POLL_IRQn         = 90 + 16,
    SPIM2_IRQ_EOT_IRQn          = 91 + 16,
    SPIM3_IRQ_RX_IRQn           = 92 + 16,
    SPIM3_IRQ_TX_IRQn           = 93 + 16,
    SPIM3_IRQ_POLL_IRQn         = 94 + 16,
    SPIM3_IRQ_EOT_IRQn          = 95 + 16,

    //i2cs
    I2C0_IRQ_RX_IRQn            = 96 + 16,
    I2C0_IRQ_TX_IRQn            = 97 + 16,
    //I2C0_IRQ2_IRQn            = 98 + 16,
    //I2C0_IRQ3_IRQn            = 99 + 16,
    I2C1_IRQ_RX_IRQn            = 100 + 16,
    I2C1_IRQ_TX_IRQn            = 101 + 16,
    //I2C1_IRQ2_IRQn            = 102 + 16,
    //I2C1_IRQ3_IRQn            = 103 + 16,
    I2C2_IRQ_RX_IRQn            = 104 + 16,
    I2C2_IRQ_TX_IRQn            = 105 + 16,
    //I2C2_IRQ2_IRQn            = 106 + 16,
    //I2C2_IRQ3_IRQn            = 107 + 16,
    I2C3_IRQ_RX_IRQn            = 108 + 16,
    I2C3_IRQ_TX_IRQn            = 109 + 16,
    //I2C3_IRQ2_IRQn            = 110 + 16,
    //I2C3_IRQ3_IRQn            = 111 + 16,

    //sdio
    //SDIO_IRQ0_IRQn            = 112 + 16,
    //SDIO_IRQ1_IRQn            = 113 + 16,
    //SDIO_IRQ2_IRQn            = 114 + 16,
    //SDIO_IRQ3_IRQn            = 115 + 16,

    //i2s
    I2S_IRQ_RX_IRQn             = 116 + 16,
    I2S_IRQ_TX_IRQn             = 117 + 16,
    //I2S_IRQ2_IRQn             = 118 + 16,
    //I2S_IRQ3_IRQn             = 119 + 16,

    //camif
    //CAMIF_IRQ0_IRQn           = 120 + 16,
    //CAMIF_IRQ1_IRQn           = 121 + 16,
    //CAMIF_IRQ2_IRQn           = 122 + 16,
    //CAMIF_IRQ3_IRQn           = 123 + 16,

    //scif
    //SCIF_IRQ0_IRQn            = 124 + 16,
    //SCIF_IRQ1_IRQn            = 125 + 16,
    //SCIF_IRQ2_IRQn            = 126 + 16,
    //SCIF_IRQ3_IRQn            = 127 + 16,

    //spi slave
    //SPIS0_IRQ0_IRQn           = 128 + 16,
    //SPIS0_IRQ1_IRQn           = 129 + 16,
    //SPIS0_IRQ2_IRQn           = 130 + 16,
    //SPIS0_IRQ3_IRQn           = 131 + 16,
    //SPIS1_IRQ0_IRQn           = 132 + 16,
    //SPIS1_IRQ1_IRQn           = 133 + 16,
    //SPIS1_IRQ2_IRQn           = 134 + 16,
    //SPIS1_IRQ3_IRQn           = 135 + 16,

    //RESERVED_IRQn             = 136 + 16,
    //RESERVED_IRQn             = 137 + 16,
    //RESERVED_IRQn             = 138 + 16,
    //RESERVED_IRQn             = 139 + 16,
    //RESERVED_IRQn             = 140 + 16,
    //RESERVED_IRQn             = 141 + 16,
    //RESERVED_IRQn             = 142 + 16,
    //RESERVED_IRQn             = 143 + 16,

    //gpios
    GPIO_IRQ0_IRQn            = 144 + 16
    //GPIO_IRQ1_IRQn            = 145 + 16,
    //GPIO_IRQ2_IRQn            = 146 + 16,
    //GPIO_IRQ3_IRQn            = 147 + 16,
    //GPIO_IRQ4_IRQn            = 148 + 16,
    //GPIO_IRQ5_IRQn            = 149 + 16,
    //GPIO_IRQ6_IRQn            = 150 + 16,
    //GPIO_IRQ7_IRQn            = 151 + 16,

    //RESERVED_IRQn             = 152 + 16,
    //RESERVED_IRQn             = 153 + 16,
    //RESERVED_IRQn             = 154 + 16,
    //RESERVED_IRQn             = 155 + 16,
    //RESERVED_IRQn             = 156 + 16,
    //RESERVED_IRQn             = 157 + 16,
    //RESERVED_IRQn             = 158 + 16,
    //RESERVED_IRQn             = 159 + 16,
    //RESERVED_IRQn             = 160 + 16,
    //RESERVED_IRQn             = 161 + 16,
    //RESERVED_IRQn             = 162 + 16,
    //RESERVED_IRQn             = 163 + 16,
    //RESERVED_IRQn             = 164 + 16,
    //RESERVED_IRQn             = 165 + 16,
    //RESERVED_IRQn             = 166 + 16,
    //RESERVED_IRQn             = 167 + 16,
    //RESERVED_IRQn             = 168 + 16,
    //RESERVED_IRQn             = 169 + 16,
    //RESERVED_IRQn             = 170 + 16,
    //RESERVED_IRQn             = 171 + 16,
    //RESERVED_IRQn             = 172 + 16,
    //RESERVED_IRQn             = 173 + 16,
    //RESERVED_IRQn             = 174 + 16,
    //RESERVED_IRQn             = 175 + 16,
    //RESERVED_IRQn             = 176 + 16,
    //RESERVED_IRQn             = 177 + 16,
    //RESERVED_IRQn             = 178 + 16,
    //RESERVED_IRQn             = 179 + 16,
    //RESERVED_IRQn             = 180 + 16,
    //RESERVED_IRQn             = 181 + 16,
    //RESERVED_IRQn             = 182 + 16,
    //RESERVED_IRQn             = 183 + 16,
    //RESERVED_IRQn             = 184 + 16,
    //RESERVED_IRQn             = 185 + 16,
    //RESERVED_IRQn             = 186 + 16,
    //RESERVED_IRQn             = 187 + 16,
    //RESERVED_IRQn             = 188 + 16,
    //RESERVED_IRQn             = 189 + 16,
    //RESERVED_IRQn             = 190 + 16,
    //RESERVED_IRQn             = 191 + 16,
    //RESERVED_IRQn             = 192 + 16,

    //Errors
    //RESERVED_IRQn             = 193 + 16,
    //RESERVED_IRQn             = 194 + 16,
    //RESERVED_IRQn             = 195 + 16,
    //RESERVED_IRQn             = 196 + 16,
    //RESERVED_IRQn             = 197 + 16,
    //RESERVED_IRQn             = 198 + 16,
    //RESERVED_IRQn             = 199 + 16,
    //RESERVED_IRQn             = 200 + 16,
    //RESERVED_IRQn             = 201 + 16,
    //RESERVED_IRQn             = 202 + 16,
    //RESERVED_IRQn             = 203 + 16,
    //RESERVED_IRQn             = 204 + 16,
    //RESERVED_IRQn             = 205 + 16,
    //RESERVED_IRQn             = 206 + 16,
    //RESERVED_IRQn             = 207 + 16,
    //RESERVED_IRQn             = 208 + 16,
    //RESERVED_IRQn             = 209 + 16,
    //RESERVED_IRQn             = 210 + 16,
    //RESERVED_IRQn             = 211 + 16,
    //RESERVED_IRQn             = 212 + 16,
    //RESERVED_IRQn             = 213 + 16,
    //RESERVED_IRQn             = 214 + 16,
    //RESERVED_IRQn             = 215 + 16,
    //RESERVED_IRQn             = 216 + 16,
    //RESERVED_IRQn             = 217 + 16,
    //RESERVED_IRQn             = 218 + 16,
    //RESERVED_IRQn             = 219 + 16,
    //RESERVED_IRQn             = 220 + 16,
    //RESERVED_IRQn             = 221 + 16,
    //RESERVED_IRQn             = 222 + 16,
    //RESERVED_IRQn             = 223 + 16
} IRQn_Type;

/*
 * ==========================================================================
 * ----------- Processor and Core Peripheral Section ------------------------
 * ==========================================================================
 */

/* Configuration of the Cortex M7 Processor and Core Peripherals */
#define __VTOR_PRESENT          1       /*!< cm7ikmcu can support the VTOR                       */
//#define __CM7_REV              0x0001U /*!< CM7 Core Revision                                                         */
#define __NVIC_PRIO_BITS       4       /*!< Number of Bits used for Priority Levels                                   */
#define __Vendor_SysTickConfig 0       /*!< Set to 1 if different SysTick Config is used                              */
#define __MPU_PRESENT          1       /*!< MPU present                                                               */
#define __FPU_PRESENT          1       /*!< FPU present                                                               */
#define __FPU_DP               0       /*!< Double Precision FPU                                                      */
#define __ICACHE_PRESENT       1       /*!< Instruction Cache present                                                 */
#define __DCACHE_PRESENT       1       /*!< Data Cache present                                                        */
#define __ITCM_PRESENT         0       /*!< Instruction TCM present                                                   */
#define __DTCM_PRESENT         0       /*!< Data TCM present                                                          */


#include "core_cm7.h"                   /* Cortex M7 processor and core peripherals              */
#include "system_daric.h"               /* daric System                                       */

//#define BIT(n)           (1 << n)
//#define MEM32_AT(adr)    (*(volatile unsigned int *) (adr))
//#define PRINT_VA(value)  MEM32_AT(SFR1_BASE + 0x3fc) = (0x77 << 24) | (value & 0xffffff);
//#define BRANCH_PASS(num) MEM32_AT(SFR1_BASE + 0x3fc) = (0xaaaaaa << 8) | (num & 0xff);
//#define BRANCH_FAIL(num) MEM32_AT(SFR1_BASE + 0x3fc) = (0x555555 << 8) | (num & 0xff);
//#define TEST_END()       MEM32_AT(SFR1_BASE + 0x3fc) = 0x66666666;

#ifndef __IM /*!< Fallback for older CMSIS versions                                         */
#define __IM __I
#endif
#ifndef __OM /*!< Fallback for older CMSIS versions                                         */
#define __OM __O
#endif
#ifndef __IOM /*!< Fallback for older CMSIS versions                                         */
#define __IOM __IO
#endif

#if defined(__CC_ARM)
#pragma anon_unions
#endif

/**
 * Initialize the system clock
 *
 * @param  none
 * @return none
 *
 * @brief  Setup the microcontroller system
 *         Initialize the PLL and update the SystemFrequency variable
 */
extern void SystemInit (void);

/**
 * Enable Caches
 */
extern void cache_enable(void);

/**
 * Invalidate Caches
 */
extern void cache_invalidate(void);

/******************************************************************************/
/*                  Cortex-M7 ETM registers structures                        */
/******************************************************************************/
#if 0

/*----------------------- Embedded Trace Macrocell ---------------------------*/
typedef struct
{
         uint32_t RESERVED0;
    __IO uint32_t TRCPRGCTLR;                  /*!< Offset: 0x004  ETM Programming Control Register                                        */
    __IO uint32_t TRCPROCSELR;                 /*!< Offset: 0x008  ETM PE Select Control Register                                          */
    __I  uint32_t TRCSTATR;                    /*!< Offset: 0x00C  ETM Trace Status Register                                               */
    __IO uint32_t TRCCONFIGR;                  /*!< Offset: 0x010  ETM Trace Configuration Register                                        */
         uint32_t RESERVED1;
    __IO uint32_t TRCAUXCTLR;                  /*!< Offset: 0x018  ETM Auxiliary Control Register                                          */
         uint32_t RESERVED2;
    __IO uint32_t TRCEVENTCTL0R;               /*!< Offset: 0x020  ETM Event Control Register 0                                            */
    __IO uint32_t TRCEVENTCTL1R;               /*!< Offset: 0x024  ETM Event Control Register 1                                            */
         uint32_t RESERVED3;
    __IO uint32_t TRCSTALLCTLR;                /*!< Offset: 0x02C  ETM Stall Control Register                                              */
    __IO uint32_t TRCTSCTLR;                   /*!< Offset: 0x030  ETM Global Timestamp Control Register                                   */
    __IO uint32_t TRCSYNCPR;                   /*!< Offset: 0x034  ETM Synchronization Period Register                                     */
    __IO uint32_t TRCCCCTLR;                   /*!< Offset: 0x038  ETM Cycle Count Control Register                                        */
    __IO uint32_t TRCBBCTLR;                   /*!< Offset: 0x03C  ETM Branch Broadcast Control Register                                   */
    __IO uint32_t TRCTRACEIDR;                 /*!< Offset: 0x040  ETM Trace ID Register                                                   */
         uint32_t RESERVED4[15];
    __IO uint32_t TRCVICTLR;                   /*!< Offset: 0x080  ETM ViewInst Main Control Register                                      */
    __IO uint32_t TRCVIIECTLR;                 /*!< Offset: 0x084  ETM ViewInst Include/Exclude Control Register                           */
    __IO uint32_t TRCVISSCTLR;                 /*!< Offset: 0x088  ETM ViewInst Start/Stop Control Register                                */
    __IO uint32_t TRCVIPCSSCTLR;               /*!< Offset: 0x08C  ETM ViewInst Start/Stop PE Comparator Control Register                  */
         uint32_t RESERVED5[4];
    __IO uint32_t TRCVDCTLR;                   /*!< Offset: 0x0A0  ETM ViewData Main Control Register                                      */
    __IO uint32_t TRCVDSACCTLR;                /*!< Offset: 0x0A4  ETM ViewData Include/Exclude Single Address Comparator Control Register */
    __IO uint32_t TRCVDARCCTLR;                /*!< Offset: 0x0A8  ETM ViewData Include/Exclude Address Range Comparator Control Register  */
         uint32_t RESERVED6[21];
    __IO uint32_t TRCSEQEVR0;                  /*!< Offset: 0x100  ETM Sequencer State Transition Control Register 0                       */
    __IO uint32_t TRCSEQEVR1;                  /*!< Offset: 0x104  ETM Sequencer State Transition Control Register 1                       */
    __IO uint32_t TRCSEQEVR2;                  /*!< Offset: 0x108  ETM Sequencer State Transition Control Register 2                       */
         uint32_t RESERVED7[3];
    __IO uint32_t TRCSEQRSTEVR;                /*!< Offset: 0x118  ETM Sequencer Reset Control Register                                    */
    __IO uint32_t TRCSEQSTR;                   /*!< Offset: 0x11C  ETM Sequencer State Register                                            */
    __IO uint32_t TRCEXTINSELR;                /*!< Offset: 0x120  ETM External Input Select Register                                      */
         uint32_t RESERVED8[7];
    __IO uint32_t TRCCNTRLDVR0;                /*!< Offset: 0x140  ETM Counter Reload Value Register 0                                     */
    __IO uint32_t TRCCNTRLDVR1;                /*!< Offset: 0x144  ETM Counter Reload Value Register 1                                     */
         uint32_t RESERVED9[2];
    __IO uint32_t TRCCNTCTLR0;                 /*!< Offset: 0x150  ETM Counter Control Register 0                                          */
    __IO uint32_t TRCCNTCTLR1;                 /*!< Offset: 0x154  ETM Counter Control Register 1                                          */
         uint32_t RESERVED10[2];
    __IO uint32_t TRCCNTVR0;                   /*!< Offset: 0x160  ETM Counter Value Register 0                                            */
    __IO uint32_t TRCCNTVR1;                   /*!< Offset: 0x164  ETM Counter Value Register 1                                            */
         uint32_t RESERVED11[6];
    __I  uint32_t TRCIDR8;                     /*!< Offset: 0x180  ETM ID Register 8                                                       */
    __I  uint32_t TRCIDR9;                     /*!< Offset: 0x184  ETM ID Register 9                                                       */
    __I  uint32_t TRCIDR10;                    /*!< Offset: 0x188  ETM ID Register 10                                                      */
    __I  uint32_t TRCIDR11;                    /*!< Offset: 0x18C  ETM ID Register 11                                                      */
    __I  uint32_t TRCIDR12;                    /*!< Offset: 0x190  ETM ID Register 12                                                      */
    __I  uint32_t TRCIDR13;                    /*!< Offset: 0x194  ETM ID Register 13                                                      */
         uint32_t RESERVED12[18];
    __I  uint32_t TRCIDR0;                     /*!< Offset: 0x1E0  ETM ID Register 0                                                       */
    __I  uint32_t TRCIDR1;                     /*!< Offset: 0x1E4  ETM ID Register 1                                                       */
    __I  uint32_t TRCIDR2;                     /*!< Offset: 0x1E8  ETM ID Register 2                                                       */
    __I  uint32_t TRCIDR3;                     /*!< Offset: 0x1EC  ETM ID Register 3                                                       */
    __I  uint32_t TRCIDR4;                     /*!< Offset: 0x1F0  ETM ID Register 4                                                       */
    __I  uint32_t TRCIDR5;                     /*!< Offset: 0x1F4  ETM ID Register 5                                                       */
    __I  uint32_t TRCIDR6;                     /*!< Offset: 0x1F8  ETM ID Register 6                                                       */
    __I  uint32_t TRCIDR7;                     /*!< Offset: 0x1FC  ETM ID Register 7                                                       */
         uint32_t RESERVED13[2];
    __IO uint32_t TRCRSCTLR2;                  /*!< Offset: 0x208  ETM Resource Selection Control Register 2                               */
    __IO uint32_t TRCRSCTLR3;                  /*!< Offset: 0x20C  ETM Resource Selection Control Register 3                               */
    __IO uint32_t TRCRSCTLR4;                  /*!< Offset: 0x210  ETM Resource Selection Control Register 4                               */
    __IO uint32_t TRCRSCTLR5;                  /*!< Offset: 0x214  ETM Resource Selection Control Register 5                               */
    __IO uint32_t TRCRSCTLR6;                  /*!< Offset: 0x218  ETM Resource Selection Control Register 6                               */
    __IO uint32_t TRCRSCTLR7;                  /*!< Offset: 0x21C  ETM Resource Selection Control Register 7                               */
    __IO uint32_t TRCRSCTLR8;                  /*!< Offset: 0x220  ETM Resource Selection Control Register 8                               */
    __IO uint32_t TRCRSCTLR9;                  /*!< Offset: 0x224  ETM Resource Selection Control Register 9                               */
    __IO uint32_t TRCRSCTLR10;                 /*!< Offset: 0x228  ETM Resource Selection Control Register 10                              */
    __IO uint32_t TRCRSCTLR11;                 /*!< Offset: 0x22C  ETM Resource Selection Control Register 11                              */
    __IO uint32_t TRCRSCTLR12;                 /*!< Offset: 0x230  ETM Resource Selection Control Register 12                              */
    __IO uint32_t TRCRSCTLR13;                 /*!< Offset: 0x234  ETM Resource Selection Control Register 13                              */
    __IO uint32_t TRCRSCTLR14;                 /*!< Offset: 0x238  ETM Resource Selection Control Register 14                              */
    __IO uint32_t TRCRSCTLR15;                 /*!< Offset: 0x23C  ETM Resource Selection Control Register 15                              */
         uint32_t RESERVED14[16];
    __IO uint32_t TRCSSCCR0;                   /*!< Offset: 0x280  ETM Single-Shot Comparator Control Register 0                           */
         uint32_t RESERVED15[7];
    __IO uint32_t TRCSSCSR0;                   /*!< Offset: 0x2A0  ETM Single-Shot Comparator Status Register 0                            */
         uint32_t RESERVED16[7];
    __IO uint32_t TRCSSPCICR0;                 /*!< Offset: 0x2C0  ETM Single-Shot PE Comparator Input Control Register 0                  */
         uint32_t RESERVED17[15];
    __O  uint32_t TRCOSLAR;                    /*!< ETM OS Lock Access Register                                             */
    __I  uint32_t TRCOSLSR;                    /*!< ETM OS Lock Status Register                                             */
         uint32_t RESERVED18[2];
    __IO uint32_t TRCPDCR;                     /*!< Offset: 0x310  ETM Power Down Control Register                                         */
    __I  uint32_t TRCPDSR;                     /*!< Offset: 0x314  ETM Power Down Status Register                                          */
         uint32_t RESERVED19[58];
    __IO uint32_t TRCACVR0;                    /*!< Offset: 0x410  ETM Address Comparator Value Register 0                                 */
         uint32_t RESERVED20;
    __IO uint32_t TRCACVR1;                    /*!< ETM Address Comparator Value Register 1                                 */
         uint32_t RESERVED21;
    __IO uint32_t TRCACVR2;                    /*!< ETM Address Comparator Value Register 2                                 */
         uint32_t RESERVED22;
    __IO uint32_t TRCACVR3;                    /*!< ETM Address Comparator Value Register 3                                 */
         uint32_t RESERVED23;
    __IO uint32_t TRCACVR4;                    /*!< ETM Address Comparator Value Register 4                                 */
         uint32_t RESERVED24;
    __IO uint32_t TRCACVR5;                    /*!< ETM Address Comparator Value Register 5                                 */
         uint32_t RESERVED25;
    __IO uint32_t TRCACVR6;                    /*!< ETM Address Comparator Value Register 6                                 */
         uint32_t RESERVED26;
    __IO uint32_t TRCACVR7;                    /*!< ETM Address Comparator Value Register 7                                 */
         uint32_t RESERVED27;
    __IO uint32_t TRCACVR8;                    /*!< ETM Address Comparator Value Register 8                                 */
         uint32_t RESERVED28;
    __IO uint32_t TRCACVR9;                    /*!< ETM Address Comparator Value Register 9                                 */
         uint32_t RESERVED29;
    __IO uint32_t TRCACVR10;                   /*!< ETM Address Comparator Value Register 10                                */
         uint32_t RESERVED30;
    __IO uint32_t TRCACVR11;                   /*!< ETM Address Comparator Value Register 11                                */
         uint32_t RESERVED31;
    __IO uint32_t TRCACVR12;                   /*!< ETM Address Comparator Value Register 12                                */
         uint32_t RESERVED32;
    __IO uint32_t TRCACVR13;                   /*!< ETM Address Comparator Value Register 13                                */
         uint32_t RESERVED33;
    __IO uint32_t TRCACVR14;                   /*!< ETM Address Comparator Value Register 14                                */
         uint32_t RESERVED34;
    __IO uint32_t TRCACVR15;                   /*!< ETM Address Comparator Value Register 15                                */
         uint32_t RESERVED35;
    __IO uint32_t TRCACATR0;                   /*!< ETM Address Comparator Access Type Register 0                           */
         uint32_t RESERVED36;
    __IO uint32_t TRCACATR1;                   /*!< ETM Address Comparator Access Type Register 1                           */
         uint32_t RESERVED37;
    __IO uint32_t TRCACATR2;                   /*!< ETM Address Comparator Access Type Register 2                           */
         uint32_t RESERVED38;
    __IO uint32_t TRCACATR3;                   /*!< ETM Address Comparator Access Type Register 3                           */
         uint32_t RESERVED39;
    __IO uint32_t TRCACATR4;                   /*!< ETM Address Comparator Access Type Register 4                           */
         uint32_t RESERVED40;
    __IO uint32_t TRCACATR5;                   /*!< ETM Address Comparator Access Type Register 5                           */
         uint32_t RESERVED41;
    __IO uint32_t TRCACATR6;                   /*!< ETM Address Comparator Access Type Register 6                           */
         uint32_t RESERVED42;
    __IO uint32_t TRCACATR7;                   /*!< ETM Address Comparator Access Type Register 7                           */
         uint32_t RESERVED43;
    __IO uint32_t TRCACATR8;                   /*!< ETM Address Comparator Access Type Register 8                           */
         uint32_t RESERVED44;
    __IO uint32_t TRCACATR9;                   /*!< ETM Address Comparator Access Type Register 9                           */
         uint32_t RESERVED45;
    __IO uint32_t TRCACATR10;                  /*!< ETM Address Comparator Access Type Register 10                          */
         uint32_t RESERVED46;
    __IO uint32_t TRCACATR11;                  /*!< ETM Address Comparator Access Type Register 11                          */
         uint32_t RESERVED47;
    __IO uint32_t TRCACATR12;                  /*!< ETM Address Comparator Access Type Register 12                          */
         uint32_t RESERVED48;
    __IO uint32_t TRCACATR13;                  /*!< ETM Address Comparator Access Type Register 13                          */
         uint32_t RESERVED49;
    __IO uint32_t TRCACATR14;                  /*!< ETM Address Comparator Access Type Register 14                          */
         uint32_t RESERVED50;
    __IO uint32_t TRCACATR15;                  /*!< ETM Address Comparator Access Type Register 15                          */
         uint32_t RESERVED51;
    __IO uint32_t TRCDVCVR0;                   /*!< ETM Data Value Comparator Value Register 0                              */
         uint32_t RESERVED52[3];
    __IO uint32_t TRCDVCVR1;                   /*!< ETM Data Value Comparator Value Register 1                              */
         uint32_t RESERVED53[27];
    __IO uint32_t TRCDVCMR0;                   /*!< ETM Data Value Comparator Mask Register 0                               */
         uint32_t RESERVED54[3];
    __IO uint32_t TRCDVCMR1;                   /*!< ETM Data Value Comparator Mask Register 1                               */
         uint32_t RESERVED55[603];
    __IO uint32_t TRCITCTRL;                   /*!< ETM Integration Mode Control Register                                   */
         uint32_t RESERVED56[39];
    __IO uint32_t TRCCLAIMSET;                 /*!< ETM Claim Tag Set Register                                              */
    __IO uint32_t TRCCLAIMCLR;                 /*!< ETM Claim Tag Clear Register                                            */
    __I  uint32_t TRCDEVAFF0;                  /*!< ETM Device Affinity Register 0                                          */
    __I  uint32_t TRCDEVAFF1;                  /*!< ETM Device Affinity Register 1                                          */
    __O  uint32_t TRCLAR;                      /*!< ETM Software Lock Access Register                                       */
    __I  uint32_t TRCLSR;                      /*!< ETM Software Lock Status Register                                       */
    __I  uint32_t TRCAUTHSTATUS;               /*!< ETM Authentication Status Register                                      */
    __I  uint32_t TRCDEVARCH;                  /*!< ETM Device Architecture Register                                        */
         uint32_t RESERVED57[2];
    __I  uint32_t TRCDEVID;                    /*!< ETM Device ID Register                                                  */
    __I  uint32_t TRCDEVTYPE;                  /*!< ETM Device Type Register                                                */
    __I  uint32_t TRCPIDR4;                    /*!< ETM CoreSight Peripheral ID4 Register                                   */
    __I  uint32_t TRCPIDR5;                    /*!< ETM CoreSight Peripheral ID5 Register                                   */
    __I  uint32_t TRCPIDR6;                    /*!< ETM CoreSight Peripheral ID6 Register                                   */
    __I  uint32_t TRCPIDR7;                    /*!< ETM CoreSight Peripheral ID7 Register                                   */
    __I  uint32_t TRCPIDR0;                    /*!< ETM CoreSight Peripheral ID0 Register                                   */
    __I  uint32_t TRCPIDR1;                    /*!< ETM CoreSight Peripheral ID1 Register                                   */
    __I  uint32_t TRCPIDR2;                    /*!< ETM CoreSight Peripheral ID2 Register                                   */
    __I  uint32_t TRCPIDR3;                    /*!< ETM CoreSight Peripheral ID3 Register                                   */
    __I  uint32_t TRCCIDR0;                    /*!< ETM CoreSight Component ID0 Register                                    */
    __I  uint32_t TRCCIDR1;                    /*!< ETM CoreSight Component ID1 Register                                    */
    __I  uint32_t TRCCIDR2;                    /*!< ETM CoreSight Component ID2 Register                                    */
    __I  uint32_t TRCCIDR3;                    /*!< ETM CoreSight Component ID3 Register                                    */
} ETM_Type;

#define ETM_TRCPRGCTLR_EN_Pos            0                                      /*!< ETM TRCPRGCTLR: Trace Enable Position            */
#define ETM_TRCPRGCTLR_EN_Msk            (1UL << ETM_TRCPRGCTLR_EN_Pos)         /*!< ETM TRCPRGCTLR: Trace Enable Mask                */

#define ETM_TRCPROCSELR_PROCSEL_Pos      0                                      /*!< ETM TRCPROCSELR: Processor Select Position       */
#define ETM_TRCPROCSELR_PROCSEL_Msk      (0x3UL << ETM_TRCPROCSELR_PROCSEL_Pos) /*!< ETM TRCPROCSELR: Processor Select Mask           */

#define ETM_TRCSTATR_IDLE_Pos            0                                      /*!< ETM TRCSTATR: Idle Status Position               */
#define ETM_TRCSTATR_IDLE_Msk            (1UL << ETM_TRCSTATR_IDLE_Pos)         /*!< ETM TRCSTATR: Idle Status Mask                   */

#define ETM_TRCSTATR_PMSTABLE_Pos        0                                      /*!< ETM TRCSTATR: Programmers' Model Status Position */
#define ETM_TRCSTATR_PMSTABLE_Msk        (1UL << ETM_TRCSTATR_PMSTABLE_Pos)     /*!< ETM TRCSTATR: Programmers' Model Status Mask     */

#define ETM_TRCCONFIGR_INSTP0_Pos        1
#define ETM_TRCCONFIGR_INSTP0_Msk        (0x3UL << ETM_TRCCONFIGR_INSTP0_Pos)

#define ETM_TRCCONFIGR_BB_Pos            3
#define ETM_TRCCONFIGR_BB_Msk            (1UL << ETM_TRCCONFIGR_BB_Pos)

#define ETM_TRCCONFIGR_CCI_Pos           4
#define ETM_TRCCONFIGR_CCI_Msk           (1UL << ETM_TRCCONFIGR_CCI_Pos)

#define ETM_TRCCONFIGR_COND_Pos          8
#define ETM_TRCCONFIGR_COND_Msk          (0x7UL << ETM_TRCCONFIGR_COND_Pos)

#define ETM_TRCCONFIGR_TS_Pos            11
#define ETM_TRCCONFIGR_TS_Msk            (1UL << ETM_TRCCONFIGR_TS_Pos)

#define ETM_TRCCONFIGR_RS_Pos            12
#define ETM_TRCCONFIGR_RS_Msk            (1UL << ETM_TRCCONFIGR_RS_Pos)

#define ETM_TRCCONFIGR_DA_Pos            16
#define ETM_TRCCONFIGR_DA_Msk            (1UL << ETM_TRCCONFIGR_DA_Pos)

#define ETM_TRCCONFIGR_DV_Pos            17
#define ETM_TRCCONFIGR_DV_Msk            (1UL << ETM_TRCCONFIGR_DV_Pos)

#define ETM_TRCSYNCPR_PERIOD_Pos         0
#define ETM_TRCSYNCPR_PERIOD_Msk         (0x1FUL << ETM_TRCSYNCPR_PERIOD_Pos)

#define ETM_TRCTRACEIDR_TRACEID_Pos      0
#define ETM_TRCTRACEIDR_TRACEID_Msk      (0x7FUL << ETM_TRCTRACEIDR_TRACEID_Pos)

#define ETM_TRCVICTLR_EVENT_Pos          0
#define ETM_TRCVICTLR_EVENT_Msk          (0xFFUL << ETM_TRCVICTLR_EVENT_Pos)

#define ETM_TRCVICTLR_SSSTATUS_Pos       9
#define ETM_TRCVICTLR_SSSTATUS_Msk       (1UL << ETM_TRCVICTLR_SSSTATUS_Pos)

#define ETM_TRCVICTLR_TRCRESET_Pos       10
#define ETM_TRCVICTLR_TRCRESET_Msk       (1UL << ETM_TRCVICTLR_TRCRESET_Pos)

#define ETM_TRCVICTLR_TRCERR_Pos         11
#define ETM_TRCVICTLR_TRCERR_Msk         (1UL << ETM_TRCVICTLR_TRCERR_Pos)

#define ETM_TRCVICTLR_EXLEVEL_S_Pos      16
#define ETM_TRCVICTLR_EXLEVEL_S_Msk      (0xFUL << ETM_TRCVICTLR_EXLEVEL_S_Pos)

#define ETM_TRCVICTLR_EXLEVEL_NS_Pos     20
#define ETM_TRCVICTLR_EXLEVEL_NS_Msk     (0xFUL << ETM_TRCVICTLR_EXLEVEL_NS_Pos)

#define ETM_TRCPIDR4_DES_2_Pos           0
#define ETM_TRCPIDR4_DES_2_Msk           (0xFUL << ETM_TRCPIDR4_DES_2_Pos)

#define ETM_TRCPIDR4_SIZE_Pos            4
#define ETM_TRCPIDR4_SIZE_Msk            (0xFUL << ETM_TRCPIDR4_SIZE_Pos)

#define ETM_TRCPIDR0_PART_0_Pos          0
#define ETM_TRCPIDR0_PART_0_Msk          (0xFFUL << ETM_TRCPIDR4_PART_0_Pos)

#define ETM_TRCPIDR1_PART_1_Pos          0
#define ETM_TRCPIDR1_PART_1_Msk          (0xFUL << ETM_TRCPIDR1_PART_1_Pos)

#define ETM_TRCPIDR1_DES_0_Pos           4
#define ETM_TRCPIDR1_DES_0_Msk           (0xFUL << ETM_TRCPIDR1_DES_0_Pos)

#define ETM_TRCPIDR2_DES_1_Pos           0
#define ETM_TRCPIDR2_DES_1_Msk           (0x7UL << ETM_TRCPIDR2_DES_1_Pos)

#define ETM_TRCPIDR2_REVISION_Pos        4
#define ETM_TRCPIDR2_REVISION_Msk        (0xFUL << ETM_TRCPIDR2_REVISION_Pos)

#define ETM_TRCPIDR3_CMOD_Pos            0
#define ETM_TRCPIDR3_CMOD_Msk            (0xFUL << ETM_TRCPIDR3_CMOD_Pos)

#define ETM_TRCPIDR3_REVAND_Pos          4
#define ETM_TRCPIDR3_REVAND_Msk          (0xFUL << ETM_TRCPIDR3_REVAND_Pos)

#define ETM_TRCLAR_KEY_UNLOCK            0xC5ACCE55UL

/******************************************************************************/
/*                  Cortex-M7 CTI registers structures                       */
/******************************************************************************/

/*--------------------------- Cross Trigger Interface ------------------------*/
typedef struct
{
    __IO uint32_t CONTROL;                     /*!< Offset: 0x000 (R/W) CTI Control Register                   */
         uint32_t RESERVED0[3];
    __O  uint32_t INTACK;                      /*!< Offset: 0x010 ( /W) CTI Output Trigger Ack Register        */
    __IO uint32_t APPSET;                      /*!< Offset: 0x014 (R/w) CTI Application Trigger Set Register   */
    __O  uint32_t APPCLR;                      /*!< Offset: 0x018 ( /W) CTI Application Trigger Clear Register */
    __O  uint32_t APPPULSE;                    /*!< Offset: 0x01C ( /W) CTI Application Pulse Register         */
    __IO uint32_t INEN0;                       /*!< Offset: 0x020 (R/W) CTI Input Trigger to Output Chn Enable */
         uint32_t RESERVED1[31];
    __IO uint32_t OUTEN0;                      /*!< Offset: 0x0A0 (R/W) CTI Input Channel to Output Trg Enable */
         uint32_t RESERVED2;
    __IO uint32_t OUTEN2;                      /*!< Offset: 0x0A8 (R/W) CTI Input Channel to Output Trg Enable */
    __IO uint32_t OUTEN3;                      /*!< Offset: 0x0AC (R/W) CTI Input Channel to Output Trg Enable */
    __IO uint32_t OUTEN4;                      /*!< Offset: 0x0B0 (R/W) CTI Input Channel to Output Trg Enable */
    __IO uint32_t OUTEN5;                      /*!< Offset: 0x0B4 (R/W) CTI Input Channel to Output Trg Enable */
         uint32_t RESERVED3;
    __IO uint32_t OUTEN7;                      /*!< Offset: 0x0BC (R/W) CTI Input Channel to Output Trg Enable */
         uint32_t RESERVED4[28];
    __I  uint32_t TRIGINSTATUS;                /*!< Offset: 0x130 (R/ ) CTI Trigger In Status Register         */
    __I  uint32_t TRIGOUTSTATUS;               /*!< Offset: 0x134 (R/ ) CTI Trigger Out Status Register        */
    __I  uint32_t CHINSTATUS;                  /*!< Offset: 0x138 (R/ ) CTI Channel In Status Register         */
    __I  uint32_t CHOUTSTATUS;                 /*!< Offset: 0x13C (R/ ) CTI Channel Out Status Register        */
    __IO uint32_t GATE;                        /*!< Offset: 0x140 (R/W) CTI Channel Gate Enable Register       */
         uint32_t RESERVED5[870];
    __O  uint32_t ITCHINACK;                   /*!< Offset: 0xEDC ( /W) CTI Integration Register               */
    __O  uint32_t ITTRIGINACK;                 /*!< Offset: 0xEE0 ( /W) CTI Integration Register               */
    __O  uint32_t ITCHOUT;                     /*!< Offset: 0xEE4 ( /W) CTI Integration Register               */
    __O  uint32_t ITTRIGOUT;                   /*!< Offset: 0xEE8 ( /W) CTI Integration Register               */
    __I  uint32_t ITCHOUTACK;                  /*!< Offset: 0xEEC ( /W) CTI Integration Register               */
    __I  uint32_t ITTRIGOUTACK;                /*!< Offset: 0xEF0 ( /W) CTI Integration Register               */
    __I  uint32_t ITCHIN;                      /*!< Offset: 0xEF4 ( /W) CTI Integration Register               */
    __I  uint32_t ITTRIGIN;                    /*!< Offset: 0xEF8 ( /W) CTI Integration Register               */
         uint32_t RESERVED6;
    __IO uint32_t ITCTRL;                      /*!< Offset: 0xF00 (R/W) CTI Integration Mode Control Register  */
         uint32_t RESERVED7[39];
    __IO uint32_t CLAIMSET;                    /*!< Offset: 0xFA0 (R/W) CTI Claim Set Register                 */
    __IO uint32_t CLAIMCLR;                    /*!< Offset: 0xFA4 (R/W) CTI Claim Clear Register               */
    __I  uint32_t DEVAFF0;                     /*!< Offset: 0xFA8 (R/W) CTI Device Affinity Register 0         */
    __I  uint32_t DEVAFF1;                     /*!< Offset: 0xFAC (R/W) CTI Device Affinity Register 1         */
    __O  uint32_t LOCKACCESS;                  /*!< Offset: 0xFB0 ( /W) CTI Lock Access Register               */
    __I  uint32_t LOCKSTATUS;                  /*!< Offset: 0xFB4 (R/ ) CTI Lock Status Register               */
    __I  uint32_t AUTHSTATUS;                  /*!< Offset: 0xFB8 (R/ ) CTI Authentication Status Register     */
    __I  uint32_t DEVARCH;                     /*!< Offset: 0xFBC (R/ ) CTI Device Architecture Register       */
    __I  uint32_t DEVID2;                      /*!< Offset: 0xFC0 (R/ ) CTI Device Configuration Register      */
    __I  uint32_t DEVID1;                      /*!< Offset: 0xFC4 (R/ ) CTI Device Configuration Register      */
    __I  uint32_t DEVID;                       /*!< Offset: 0xFC8 (R/ ) CTI Device Configuration Register      */
    __I  uint32_t DEVTYPE;                     /*!< Offset: 0xFCC (R/ ) CTI Device Type Register               */
    __I  uint32_t PID4;                        /*!< Offset: 0xFD0 (R/ ) CoreSight register                     */
    __I  uint32_t PID5;                        /*!< Offset: 0xFD4 (R/ ) CoreSight register                     */
    __I  uint32_t PID6;                        /*!< Offset: 0xFD8 (R/ ) CoreSight register                     */
    __I  uint32_t PID7;                        /*!< Offset: 0xFDC (R/ ) CoreSight register                     */
    __I  uint32_t PID0;                        /*!< Offset: 0xFE0 (R/ ) CoreSight register                     */
    __I  uint32_t PID1;                        /*!< Offset: 0xFE4 (R/ ) CoreSight register                     */
    __I  uint32_t PID2;                        /*!< Offset: 0xFE8 (R/ ) CoreSight register                     */
    __I  uint32_t PID3;                        /*!< Offset: 0xFEC (R/ ) CoreSight register                     */
    __I  uint32_t CID0;                        /*!< Offset: 0xFF0 (R/ ) CoreSight register                     */
    __I  uint32_t CID1;                        /*!< Offset: 0xFF4 (R/ ) CoreSight register                     */
    __I  uint32_t CID2;                        /*!< Offset: 0xFF8 (R/ ) CoreSight register                     */
    __I  uint32_t CID3;                        /*!< Offset: 0xFFC (R/ ) CoreSight register                     */
} CTI_Type;

/******************************************************************************/
/*                  Cortex-M7 TPIU registers structures                       */
/******************************************************************************/

/*--------------------- Trace Port Interface Unit ----------------------------*/
typedef struct
{
    __I  uint32_t SSPSR;                        /*!< Offset 0x000 (R ) TPIU Supported Synchronous Port Size register */
    __IO uint32_t CSPSR;                        /*!< Offset 0x004 (RW) TPIU Current Synchronous Port Size register   */
         uint32_t RESERVED0[2];
    __IO uint32_t COSDR;                        /*!< Offset 0x010 (RW) TPIU Current Output Speed Divisor register    */
         uint32_t RESERVED1[55];
    __IO uint32_t SPPR;                         /*!< Offset 0x0F0 (RW) TPIU Selected Pin Protocol register           */
         uint32_t RESERVED2[3];
    __IO uint32_t SPTMR;                        /*!< Offset 0x100  TPIU Supported Trigger Modes Register         */
    __IO uint32_t TCVR;                         /*!< Offset 0x104  TPIU Trigger Count Value Register             */
    __IO uint32_t TMR;                          /*!< Offset 0x108  TPIU Trigger Multiplier Register              */
         uint32_t RESERVED3[125];
    __I  uint32_t FFSR;                         /*!< Offset 0x300 (R ) TPIU Formatter and Flush Status Register      */
    __IO uint32_t FFCR;                         /*!< Offset 0x304 (RW) TPIU Formatter and Flush Control Register     */
         uint32_t RESERVED4[760];
    __I  uint32_t ITTRFLIN;                     /*!< Offset 0xEE8 (R ) TPIU Test Trigger In and Flush In Register    */
    __I  uint32_t ITFTTD0;                      /*!< Offset 0xEEC (R ) TPIU Integration ETM Data Register 0          */
    __O  uint32_t ITATBCTR2;                    /*!< Offset 0xEF0 ( W) TPIU Integration ATB Control2 Register        */
    __I  uint32_t ITATBCTR;                     /*!< Offset 0xEF4 (R ) TPIU Integration ITM Data Register            */
         uint32_t RESERVED7;
    __I  uint32_t ITFTTD1;                      /*!< Offset 0xEFC (R ) TPIU Integration ETM Data Register 1          */
    __IO uint32_t ITCTRL;                       /*!< Offset 0xF00 (RW) TPIU Integration Control Register             */
         uint32_t RESERVED5[39];
    __IO uint32_t CLAIMSET;                     /*!< Offset 0xFA0 (RW) TPIU Claim Tag Set Register                   */
    __IO uint32_t CLAIMCLR;                     /*!< Offset 0xFA4 (RW) TPIU Claim Tag Clear Register                 */
         uint32_t RESERVED6[8];
    __I  uint32_t DEVID;                        /*!< Offset 0xFC8 (R ) TPIU Device ID Register                       */
    __I  uint32_t DEVTYPE;                      /*!< Offset 0xFCC (R ) TPIU Device Type Register                     */
    __I  uint32_t PID4;                         /*!< Offset 0xFD0 (R ) CoreSight register                            */
         uint32_t RESERVED8[3];
    __I  uint32_t PID0;                         /*!< Offset 0xFE0 (R ) CoreSight register                            */
    __I  uint32_t PID1;                         /*!< Offset 0xFE4 (R ) CoreSight register                            */
    __I  uint32_t PID2;                         /*!< Offset 0xFE8 (R ) CoreSight register                            */
    __I  uint32_t PID3;                         /*!< Offset 0xFEC (R ) CoreSight register                            */
    __I  uint32_t CID0;                         /*!< Offset 0xFF0 (R ) CoreSight register                            */
    __I  uint32_t CID1;                         /*!< Offset 0xFF4 (R ) CoreSight register                            */
    __I  uint32_t CID2;                         /*!< Offset 0xFF8 (R ) CoreSight register                            */
    __I  uint32_t CID3;                         /*!< Offset 0xFFC (R ) CoreSight register                            */
} TPIU_Type;

#define TPIU_FFCR_STOPF1_Pos      12
#define TPIU_FFCR_STOPF1_MSK      (0x1UL << TPIU_FFCR_STOPF1_Pos)

#define TPIU_FFSR_FLINPROG_Pos    0
#define TPIU_FFSR_FLINPROG_MSK    (0x1UL << TPIU_FFCR_STOPF1_Pos)

#define TPIU_SPPR_TXMODE_Pos      0
#define TPIU_SPPR_TXMODE_Msk      (0x3UL << TPIU_SPPR_TXMODE_Pos)

#define TPIU_PIN_TRACEPORT        0          /*!< TPIU Selected Pin Protocol Parallel Port  */
#define TPIU_PIN_SWO_MANCHESTER   1          /*!< TPIU Selected Pin Protocol SWO Manchester */
#define TPIU_PIN_SWO_NRZ          2          /*!< TPIU Selected Pin Protocol SWO NRZ (uart) */

/******************************************************************************/
/*                Device Specific Peripheral registers structures             */
/******************************************************************************/

/*--------------------- General Purpose Input and Ouptut ---------------------*/
typedef union
{
    __IO uint32_t WORD;
    __IO uint16_t HALFWORD[2];
    __IO uint8_t  BYTE[4];
} GPIO_Data_TypeDef;

typedef struct
{
    GPIO_Data_TypeDef DATA [256];
    GPIO_Data_TypeDef DIR;
    uint32_t RESERVED[3];
    GPIO_Data_TypeDef IE;
} GPIO_TypeDef;

/******************************************************************************/
/*          CoreSight and other Peripheral memory map                         */
/******************************************************************************/
/* CoreSight Components */
#define TPIU_BASE                 0xE0040000UL
#define ETM_BASE                  0xE0041000UL
#define CTI_BASE                  0xE0042000UL
#define FPB_BASE                  0xE0002000UL

#define DWT_CTRL_SYNCTAP24        1 << 10
#define DWT_CTRL_SYNCTAP26        2 << 10
#define DWT_CTRL_SYNCTAP28        3 << 10

#define DWT_FUNC_SAMP_PC          0x1
#define DWT_FUNC_SAMP_DATA        0x2
#define DWT_FUNC_SAMP_PC_DATA     0x3
#define DWT_FUNC_PC_WPT           0x4
#define DWT_FUNC_TRIG_PC          0x8
#define DWT_FUNC_TRIG_RD          0x9
#define DWT_FUNC_TRIG_WR          0xA
#define DWT_FUNC_TRIG_RW          0xB

#define DWT_CTRL_POSTPRESET_10    0xA

#define ITM_TER_STIM0             1 << 0
#define ITM_TER_STIM1             1 << 1
#define ITM_TER_STIM2             1 << 2

#define ITM_TCR_TS_GLOBAL_128     0x01 << ITM_TCR_GTSFREQ_Pos
#define ITM_TCR_TS_GLOBAL_8192    0x10 << ITM_TCR_GTSFREQ_Pos
#define ITM_TCR_TS_GLOBAL_ALL     0x11 << ITM_TCR_GTSFREQ_Pos
#endif

/* Peripheral and SRAM base address */
#define SRAM_BASE                 0x20000000UL
#define PERIPH_BASE               0x40000000UL

/* Peripheral memory map */
//#define GPIO_BASE              PERIPH_BASE
//
//#define GPIO0_BASE              (GPIO_BASE)
//#define GPIO1_BASE              (GPIO_BASE       + 0x0800UL)
//#define GPIO2_BASE              (GPIO_BASE       + 0x1000UL)

/******************************************************************************/
/*                         Declaration                                        */
/******************************************************************************/
//#define ETM                       ((ETM_Type     *) ETM_BASE)
//#define TPIU                      ((TPIU_Type    *) TPIU_BASE)
//#define CTI                       ((CTI_Type     *) CTI_BASE)

//#define GPIO0                  ((GPIO_TypeDef *) GPIO0_BASE)
//#define GPIO1                  ((GPIO_TypeDef *) GPIO1_BASE)
//#define GPIO2                  ((GPIO_TypeDef *) GPIO2_BASE)

#endif
