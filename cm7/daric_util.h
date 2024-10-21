// See LICENSE file for license

#ifndef __daric_util_h_inclided__
#define __daric_util_h_inclided__

// #define CHAR_DEVICE (*(volatile char *)0x4004f0c0)

// #define UART_BUSY (*(volatile uint8_t*)0x40042008)
// #define UART_DOUT (*(volatile uint8_t *)0x40042000)
// #define MONDAT32 *(volatile uint32_t *)0X4004F000UL

typedef struct {
    volatile uint32_t cgusec; // 0x0000
    volatile uint32_t cgulp; // 0x0004
    volatile uint32_t dummy_00[2]; // 08, 0x
    volatile uint32_t cgusel0; // 0x0010
    volatile uint32_t fdfclk; // 0x0014
    volatile uint32_t fdaclk; // 0x0018
    volatile uint32_t fdhclk; // 0x001c
    volatile uint32_t fdiclk; // 0x0020
    volatile uint32_t fdpclk; // 0x0024
    volatile uint32_t dummy_01;
    volatile uint32_t cguset; // 0x002c
    volatile uint32_t cgusel1; // 0x0030
    volatile uint32_t dummy_32[3];
    volatile uint32_t cgufsfreq0; // 0x0040
    volatile uint32_t cgufsfreq1; // 0x0044
    volatile uint32_t cgufsfreq2; // 0x0048
    volatile uint32_t cgufsfreq3; // 0x004c
    volatile uint32_t cgufsvld; // 0x0050
    volatile uint32_t cgufscr; // 0x0054
    volatile uint32_t dummy_56[2];
    volatile uint32_t cguaclkgr; // 0x0060
    volatile uint32_t cguhclkgr; // 0x0064
    volatile uint32_t cguiclkgr; // 0x0068
    volatile uint32_t cgupclkgr; // 0x006c
} DARIC_SYSCTRL_CGU_T;

extern volatile DARIC_SYSCTRL_CGU_T *const DARIC_CGU;
#define DARIC_SYSCTRL_CGU_BASE         0x40040000UL


typedef struct{
    volatile uint32_t ar    ;  // 0x0090  0x00000000  CR  yes ipc_ar
    volatile uint32_t en    ;  // 0x0094  0x00000000  CR  yes ipc_en
    volatile uint32_t lpen  ;  // 0x0098  0x00000000  CR  yes ipc_lpen
    volatile uint32_t osc   ;  // 0x009c  0x00000000  CR  yes ipc_osc
    volatile uint32_t pll_mn;  // 0x00a0  0x00000000  CR  yes ipc_pll_mn for FPGA, [15:8] m, [7:0] n
                      //                                        for ASIC, 11:0   
    volatile uint32_t pll_f ;  // 0x00a4  0x00000000  CR  yes ipc_pll_f, 24-bit
    volatile uint32_t pll_q ;  // 0x00a8  0x00000000  CR  yes ipc_pll_q, 16-bit
    volatile uint32_t ipc;  // 0x00ac  0x00000000  CR  yes IP control, CR(TBD)
} DARIC_SYSCTRL_IPC_T;

extern volatile DARIC_SYSCTRL_IPC_T * const DARIC_IPC;
#define DARIC_SYSCTRL_IPC_BASE         0x40040090UL

typedef struct {
    volatile uint32_t cache;        // 00
    volatile uint32_t itcm;         // 04
    volatile uint32_t dtcm;         // 08 
    volatile uint32_t sram0;        // 0c
    volatile uint32_t sram1;        // 10
    volatile uint32_t vexram;       // 14
    volatile uint32_t DUMMY08[2];
    volatile uint32_t srambankerr;  // 20
} DARIC_CORE_SRAMCFG_T;

extern volatile DARIC_CORE_SRAMCFG_T * const DARIC_SRAMCFG;
#define DARIC_CORE_SRAMCFG_BASE         0x40014000UL

typedef struct{
    volatile uint32_t CFG_CG;
    volatile uint32_t CFG_EVENT;
    volatile uint32_t CFG_RST;
} UDMACORE_T;

extern volatile UDMACORE_T * const UDMACORE;

extern uint32_t SystemCoreClock;

typedef struct {
    volatile uint8_t TX;
    volatile uint8_t DUMMY1[3];
    volatile uint8_t EN;
    volatile uint8_t DUMMY5[3];
    volatile uint8_t BUSY;
    volatile uint8_t DUMMY9[3];
    volatile uint8_t ETU;
    volatile uint8_t DUMMYD[3];
} DUART_T;

extern volatile DUART_T * const DUART;

#ifdef __cplusplus
extern "C" {
#endif
    // int __io_putchar(int ch);
    void __uart_putchar(char ch);
    // __STATIC_INLINE
    // void printString(const char *s);

    void sendU32Hex(uint32_t x);
    void sendU8Hex(uint8_t x);

    void HardFault_Handler(void);
    uint16_t peekU16(size_t addr);
    void snapshotTime();
    void simDone();
    uint16_t writeU16(size_t addr, uint16_t value);

    void resetTicks(void);
    uint32_t snapTicks(const char* title);
    void setupTicks(void);
    void ticksDelay(uint64_t us);
    void usDelay(uint64_t us);
    uint64_t getTicks(void);
    uint32_t getTicks24(void);
    void initClockFPGA(uint32_t freqHz);
    void initClockASIC(uint32_t freqHz, uint32_t dutySRAM);

    void initSRAMWait(uint32_t waitcycles);
    void parsBitsICSR();
    void initDUART(uint32_t etu);

#ifdef __cplusplus
}
#endif

__attribute__((always_inline)) static inline
void printString(const char *s){
    char c;
    size_t i = 0;
    while ((c = s[i++]) != 0){
        __uart_putchar(c);
    }
    __uart_putchar('\n');
};

#endif  // __daric_util_h_inclided__
