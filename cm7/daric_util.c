#if defined (ARMCM7)
    #include "ARMCM7.h"
#elif defined (ARMCM7_SP)
    #include "ARMCM7_SP.h"
#elif defined (ARMCM7_DP)
    #include "ARMCM7_DP.h"
#else
    #error device not specified!
#endif

#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include "daric_util.h"

#include "printf.h"

// TODO not working

static const char * const HEX = "0123456789abcdef";

volatile DARIC_SYSCTRL_CGU_T    * const DARIC_CGU   = (DARIC_SYSCTRL_CGU_T*)DARIC_SYSCTRL_CGU_BASE;
volatile DARIC_SYSCTRL_IPC_T    * const DARIC_IPC   = (DARIC_SYSCTRL_IPC_T*)DARIC_SYSCTRL_IPC_BASE;
volatile DARIC_CORE_SRAMCFG_T   * const DARIC_SRAMCFG= (DARIC_CORE_SRAMCFG_T*)DARIC_CORE_SRAMCFG_BASE;

volatile UDMACORE_T * const UDMACORE = (UDMACORE_T *)0x50100000;

volatile DUART_T * const DUART = (DUART_T *) 0x40042000;

//static 
uint32_t volatile systick24 = 0;

// #define __uart_putchar(x) putchar(x)

#if 0
//__STATIC_INLINE 
void printString(const char *s){
    char c;
    size_t i = 0;
    while ((c = s[i++]) != 0){
        __uart_putchar(c);
    }
    __uart_putchar('\n');
};
#endif

//__STATIC_INLINE 
inline void sendU32Hex(uint32_t x){
    for (uint8_t i = 0 ; i < 8; i++){
        __uart_putchar(HEX[(x >> 28) & 0x0f]);
        x <<= 4;
    }
    __uart_putchar(' ');
}

//__STATIC_INLINE 
inline void sendU8Hex(uint8_t x){
    __uart_putchar(HEX[(x >> 4) & 0x0f]);
    __uart_putchar(HEX[x & 0x0f]);
    __uart_putchar(' ');
}

void HardFault_Handler(void) {
    static uint32_t t0;
    static uint32_t stack[8];
    static uint8_t b;
    static const char *CFSRERRS[32] = {
        "IACCVIOL",  // MMFSR  
        "DACCVIOL",   
        "RSVD",
        "MUNSTKERR",
        "MSTKERR",             
        "MLSPERR",    
        "RSVD",
        "MMARVALID",

        "IBUSERR", // BFSR     
        "PRECISERR",  
        "IMPRECISERR",
        "UNSTKERR",
        "STKERR",              
        "LSPERR",     
        "RSVD",
        "BFARVALID",        
                               
        "UNDEFINSTR", // UFSR
        "INVSTATE",
        "INVPC",
        "NOCP",                
        "RSVD",       
        "RSVD",
        "RSVD",
        "RSVD",                
                                             
        "UNALIGNED",
        "DIVBYZERO",
        "RSVD",                
        "RSVD",       
        "RSVD",
        "RSVD",
        "RSVD",                
        "RSVD",       
    };

    // dump Stack, do not use function call or stack variable
    asm volatile ("mov %0, sp\n\t"
        : "=r" (t0)
    );
    stack[0] = *(uint32_t *)((t0 + 16) & 0xfffffffcul);
    stack[1] = *(uint32_t *)((t0 + 20) & 0xfffffffcul);
    stack[2] = *(uint32_t *)((t0 + 24) & 0xfffffffcul);
    stack[3] = *(uint32_t *)((t0 + 28) & 0xfffffffcul);
    stack[4] = *(uint32_t *)((t0 + 32) & 0xfffffffcul);
    stack[5] = *(uint32_t *)((t0 + 36) & 0xfffffffcul);
    stack[6] = *(uint32_t *)((t0 + 40) & 0xfffffffcul);
    stack[7] = *(uint32_t *)((t0 + 44) & 0xfffffffcul);
    
    printString("\nSP @:");
    sendU32Hex(t0);
    printString("\nSatck new<->old:");
    sendU32Hex(stack[0]);
    sendU32Hex(stack[1]);
    sendU32Hex(stack[2]);
    sendU32Hex(stack[3]);
    sendU32Hex(stack[4]);
    sendU32Hex(stack[5]);
    sendU32Hex(stack[6]);
    sendU32Hex(stack[7]);

    printString("\n\nHardFault!!!\n");
    printString("systick: ");
    sendU32Hex(SysTick->VAL);

    t0 = SCB->CFSR;
    printString("\nCFSR");
    sendU32Hex(SCB->CFSR );
    printString("\n");
    for (b = 0; b < 32; b++){
        if (0 != (t0 & (1UL << b))){                                                       
            printString(CFSRERRS[b]);
        }
    }
    printString("\nHFSR");
    sendU32Hex(SCB->HFSR );
    printString("\nDFSR");
    sendU32Hex(SCB->DFSR );
    printString("\nMMFAR");
    sendU32Hex(SCB->MMFAR);
    printString("\nBFAR");
    sendU32Hex(SCB->BFAR );
    printString("\nAFSR");
    sendU32Hex(SCB->AFSR );
    printString("\nABFSR");
    sendU32Hex(SCB->ABFSR);
    // *((volatile uint8_t* )0x4004f0ff) = 0x55; // simDone
    
    printString("\n\nHardFault. Halted here!!!\n");

    while(1){
        ;
    }
}

uint16_t peekU16(size_t addr){
    return *((volatile uint16_t*)addr);
}

inline void snapshotTime(){
    *((volatile uint8_t* )0x4004f0c4) = 0x55;
}

inline void simDone(){
    // sim done
    *((volatile uint8_t* )0x4004f0ff) = 0x55;
    while(1){
        *((volatile uint8_t* )0x4004f0ff) = 0x55;
    }
}

uint16_t writeU16(size_t addr, uint16_t value){
    *((volatile uint16_t*)addr) = value;
    return *((volatile uint16_t*)addr);
}

void resetTicks(void){
    SysTick->VAL = SysTick->LOAD; 
    systick24 = 0;
    __DSB();
}
    
uint32_t snapTicks(const char* title){
    uint32_t tick;
    tick = 0x00ffffff ^ SysTick->VAL;
    // fprintf(stderr, "%s time: %lu ticks.\n", title, tick);
    return tick;
}

void setupTicks(void){
    // TODO tick div
    systick24 = 0;
    SysTick_Config(0x01000000); //
    // NVIC_EnableIRQ(SysTick_IRQn); // useless, NVIC_EnableIRQ only handles interrupt 1-240
    __DSB();
}


void SysTick_Handler(void){
    systick24++;
    //printString("\nSysTick\n");
}

/*
void NMI_Handler(void){
    static uint32_t  counter = 0;
    static char const nmi[] = "NMI\n";
    if (0 == (counter & 0xffff)){
        DUART->TX = nmi[(counter >> 16) & 0x03];
    }
    counter++;
}
*/

uint64_t getTicks(void){
    // TODO atomic this
    uint32_t a = systick24;
    uint32_t b = SysTick->VAL;
    uint32_t c = systick24;
    uint32_t d = SysTick->VAL;

    uint64_t r = 0;
    if (a == c){
        r = a;
        r <<= 24;
        r |= ((0x00fffffful ^ b) & SysTick_LOAD_RELOAD_Msk);
    } else {
        r = c;
        r <<= 24;
        r |= ((0x00fffffful ^ d) & SysTick_LOAD_RELOAD_Msk);
    }
    return r;
}

uint32_t getTicks24(void){
    return (0x00fffffful ^ SysTick->VAL);
}

void ticksDelay(uint64_t ticks){ // Tick should be 1MHz
    uint64_t start = getTicks();
    uint64_t end = (start + ticks); // 24bit
    if (0 == ticks){
    // } else if (start == 0x00ffffffUL) {
    // } else if (end == 0x00ffffffUL) {
    } else if (end > start){ // TODO: boundry values
        volatile uint64_t now = start;
        while (now >= start && now < end){
            now = getTicks();
        }
    } else {  // end < start, overflew
        // printf("overflew\n");
        volatile uint64_t now = start;
        while (now >= start || now < end){
            now = getTicks();
        }
    }
}

// TODO: use 
void usDelay(uint64_t us){
    ticksDelay(us * (SystemCoreClock / 1000000));
}

void initClockFPGA(uint32_t freqHz){
    uint16_t cpuclkm = (uint16_t)(freqHz / 781250UL);
    if (cpuclkm > 0xf0u){
        cpuclkm = 0xf0u;
    }
    DARIC_IPC->pll_mn = 0x4000u | (cpuclkm & 0x00ffu);
    SystemCoreClock = freqHz;
    // clk_set_osc();
    __DSB();

    DARIC_CGU->fdfclk = 0x00ff;
    DARIC_CGU->fdaclk = 0x00ff;
    DARIC_CGU->fdhclk = 0x00ff;
    DARIC_CGU->fdiclk = 0x00ff;
    DARIC_CGU->fdpclk = 0x007f;
    __DSB();
    DARIC_CGU->cguset = 0x0032;
    __DSB();
    UDMACORE->CFG_CG = 0xffffffff;
    __DSB();
}   

static inline uint32_t log_2(const uint32_t x){
    return (x == 0) ? 0:  (31 - __builtin_clz (x));
}

void initClockASIC(uint32_t freqHz, uint32_t dutySRAM){
    const uint32_t UNIT_MHZ = 1000UL * 1000UL;
    const uint32_t PFD_F_MHZ = 16;
    const uint32_t FREQ_0 = 16UL * UNIT_MHZ;
    const uint32_t FREQ_OSC_MHZ = 48; // Actually 48MHz
    const uint32_t M = (FREQ_OSC_MHZ / PFD_F_MHZ); //  - 1;  // OSC input was 24, replace with 48

    static const uint16_t TBL_Q[] = {
        // keep later DIV even number as possible
        0x7777, // 16-32 MHz 
        0x7737, // 32-64 
        0x3733, // 64-128
        0x3313, // 128-256 
        0x3311, // 256-512 // keep ~ 100MHz
        0x3301, // 512-1024
        0x3301, // 1024-1600
    };
    static const uint32_t TBL_MUL[] = {
        64, // 16-32 MHz 
        32, // 32-64 
        16, // 64-128
        8, // 128-256 
        4, // 256-512
        2, // 512-1024
        2, // 1024-1600
    };
    
    if ((0 == (DARIC_IPC->pll_mn & 0x0001F000)) || (0 == (DARIC_IPC->pll_mn & 0x00000fff))){
        // for SIM, avoid div by 0 if unconfigurated
        // , default VCO 48MHz / 48 * 1200 = 1.2GHz
        // TODO magic numbers
        DARIC_IPC->pll_mn = ((M << 12) & 0x0001F000) | ((1200) & 0x00000fff);
        DARIC_IPC->pll_f = 0; // ??
        __DSB();
        DARIC_IPC->ar     = 0x0032;  // commit
        __DSB();
    }

    // TODO select int/ext osc/xtal
    DARIC_CGU->cgusel1 = 1; // 0: RC, 1: XTAL
    DARIC_CGU->cgufscr = FREQ_OSC_MHZ; // external crystal is 48MHz
    __DSB();
    DARIC_CGU->cguset = 0x32UL;
    __DSB();
    
    if (freqHz < 1000000){
        DARIC_IPC->osc = freqHz;
        __DSB();
        DARIC_IPC->ar     = 0x0032;  // commit, must write 32
        __DSB();
    }
    // switch to OSC
    DARIC_CGU->cgusel0 = 0; // clktop sel, 0:clksys, 1:clkpll0
    __DSB();
    DARIC_CGU->cguset = 0x32; // commit
    __DSB();
    

    if (freqHz < 1000000){
    } else {
        uint64_t n_fxp24 = 0; // fixed point
        uint32_t f16MHzLog2 = log_2(freqHz / FREQ_0);
        
        // PD PLL
        DARIC_IPC->lpen |= 0x02 ;
        __DSB();
        DARIC_IPC->ar     = 0x0032;  // commit, must write 32
        __DSB();
        
        // delay 
        for (uint32_t i = 0; i < 1024; i++){
            __NOP();
        }

        n_fxp24 = (((uint64_t)freqHz << 24) * TBL_MUL[f16MHzLog2] + PFD_F_MHZ * UNIT_MHZ / 2) / (PFD_F_MHZ * UNIT_MHZ); // rounded
        uint32_t n_frac = (uint32_t)(n_fxp24 & 0x00ffffff);
        
        // printf ("%s(%4" PRIu32 "MHz) M = %4" PRIu32 ", N = %4" PRIu32 ".%08" PRIu32 ", Q = %2lu, QDiv = 0x%04" PRIx16 "\n",
        //      __FUNCTION__, freqHz / 1000000, M, (uint32_t)(n_fxp24 >> 24), (uint32_t)((uint64_t)(n_fxp24 & 0x00ffffff) * 100000000/(1UL <<24)), TBL_MUL[f16MHzLog2], TBL_Q[f16MHzLog2]);
        DARIC_IPC->pll_mn = ((M << 12) & 0x0001F000) | ((n_fxp24 >> 24) & 0x00000fff); // 0x1F598; // ??
        DARIC_IPC->pll_f = n_frac | ((0 == n_frac) ? 0 : (1UL << 24)); // ??
        DARIC_IPC->pll_q = TBL_Q[f16MHzLog2]; // ?? TODO select DIV for VCO freq
        //               VCO bias   CPP bias   CPI bias
        //                1          2          3
        //DARIC_IPC->ipc = (3 << 6) | (5 << 3) | (5); 
        DARIC_IPC->ipc = (1 << 6) | (2 << 3) | (3); 
        __DSB();
        DARIC_IPC->ar     = 0x0032;  // commit
        __DSB();

        DARIC_IPC->lpen &= ~0x02;
        __DSB();
        DARIC_IPC->ar     = 0x0032;  // commit
        __DSB();

        // delay 
        for (uint32_t i = 0; i < 1024; i++){
            __NOP();
        }
        //printf("read reg a0 : %08" PRIx32"\n", *((volatile uint32_t* )0x400400a0));
        //printf("read reg a4 : %04" PRIx16"\n", *((volatile uint16_t* )0x400400a4));
        //printf("read reg a8 : %04" PRIx16"\n", *((volatile uint16_t* )0x400400a8));
        
        // TODO wait/poll lock status?
        DARIC_CGU->cgusel0 = 1; // clktop sel, 0:clksys, 1:clkpll0
        __DSB();
        DARIC_CGU->cguset = 0x32; // commit
        __DSB();

        // printf ("    MN: 0x%05x, F: 0x%06x, Q: 0x%04x\n",
        //     DARIC_IPC->pll_mn, DARIC_IPC->pll_f, DARIC_IPC->pll_q);
        // printf ("    LPEN: 0x%01x, OSC: 0x%04x, BIAS: 0x%04x,\n",
        //     DARIC_IPC->lpen, DARIC_IPC->osc, DARIC_IPC->ipc);
    }
    
    DARIC_CGU->fdfclk = 0x7fff; // CPU
    if (0 == dutySRAM){
        DARIC_CGU->fdaclk = 0x3f7f; // SRAM
    } else {
        DARIC_CGU->fdaclk = dutySRAM;
    }
    DARIC_CGU->fdhclk = 0x1f3f;
    DARIC_CGU->fdiclk = 0x0f1f;
    DARIC_CGU->fdpclk = 0x070f;
    __DSB();
    DARIC_CGU->cguset = 0x00000032; // commit
    __DSB();

    UDMACORE->CFG_CG = 0xffffffff; //everything on
    __DSB();

    SystemCoreClock = freqHz;
}

void initSRAMWait(uint32_t waitcycles){
    DARIC_SRAMCFG->sram0    = (DARIC_SRAMCFG->sram0 & ~0x00000018) | ((waitcycles << 3) & 0x00000018);
    DARIC_SRAMCFG->sram1    = (DARIC_SRAMCFG->sram1 & ~0x00000018) | ((waitcycles << 3) & 0x00000018);
    __DSB();
}



void parsBitsICSR(){
    uint32_t t = SCB->ICSR;
    struct {
        uint32_t bit;
        char* msg;
    } icsrBits[] = {
        {31, "NMI pending"},
        {28, "PendSV pending"},
        {26, "SysTick pending"},
        {22, "NMI and Faults pending"},
        {11, "RETTOBASE"},
    };
    //printf("ICSR %08" PRIx32 ",", t);
    printf("ICSR %08x,", t);
    for (uint32_t i = 0; i < sizeof(icsrBits) / sizeof(icsrBits[0]); i++){
        if (t & (1UL << icsrBits[i].bit)){
            printf(" %s,", icsrBits[i].msg);
        }
    }
    printf("\n");
    printf("VECTACTIVE %02" PRIx32 "\n", t & 0xff);
    printf("VECTPENDING %03" PRIx32 "\n", (t >> 12) & 0x1ff);
    printf("SysTick Val %06" PRIx32 "\n", SysTick->VAL);
}

void initDUART(uint32_t etu){
    DUART->EN = 0; // freq of 32MHz RC is low
    __DSB();
    DUART->ETU = etu; // freq of 32MHz RC is low
    __DSB();
    DUART->EN = 1; // freq of 32MHz RC is low
    __DSB();
}


