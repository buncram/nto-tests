#include <stdint.h>
#include <stdlib.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#include "daric_hal.h"

extern uint32_t __etext;
extern uint32_t __data_start__;
extern uint32_t __data_end__;
extern uint32_t __bss_start__;
extern uint32_t __bss_end__;

// These are headers specific to our test application.
#include "daric_util.h"
#include "constants.h"
#include "tx_api.h"
#include "mbox.h"

void PendSV_Handler(void);
void SysTick_Handler(void);
extern void __libc_init_array(void);

uint8_t ReramWrite(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen);

#define USE_DELAY 0

// #define SRAM_TEXT_START 0x61100000UL

extern uint32_t __INITIAL_SP;

typedef enum
{
    TO_RV_OP_INVALID = 0,

    TO_RV_OP_RET_KNOCK = 128,
    TO_RV_OP_RET_DCT_8X8 = 129,
    TO_RV_OP_RET_CLIFFORD = 130,
    TO_RV_OP_RET_FLASHWRITE = 131,
} to_rv_op;

typedef enum
{
    TO_CM7_OP_INVALID = 0,
    TO_CM7_OP_KNOCK = 1,
    TO_CM7_OP_DCT_8X8 = 2,
    TO_CM7_OP_CLIFFORD = 3,
    TO_CM7_OP_FLASHWRITE = 4,
} to_cm7_op;

typedef struct
{
    uint32_t version;
    uint16_t opcode;
    uint16_t arg_len;
    uint32_t *data;
} mbox_pkt_t;

void Reset_Handler(void);
void NMI_Handler(void);
void nothing();
void main_loop();
void Mbox_Handler(void);
void Mbox_Abort(void);
int32_t serialize_tx(mbox_pkt_t *resp_pkt);
int32_t deserialize_rx(mbox_pkt_t *mbox_pkt);

void dct_naive(int8_t data_in[8][8], int16_t data_out[8][8]);
void clifford(uint8_t *buf);

#define CCR_REG *((volatile uint32_t *)0xE000ED14)

#define UNROLL 0

#if defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpedantic"
#endif

#define MBOX_WDATA (*((volatile uint32_t *)0x40013000))
#define MBOX_RDATA (*((volatile uint32_t *)0x40013004))

#define MBOX_STATUS (*((volatile uint32_t *)0x40013008))
#define STATUS_RX_AVAIL ((MBOX_STATUS >> 0) & 1)
#define STATUS_TX_AVAIL ((MBOX_STATUS >> 1) & 1)
#define STATUS_ABORT_IN_PROGRESS ((MBOX_STATUS >> 2) & 1)
#define STATUS_ABORT_ACK ((MBOX_STATUS >> 3) & 1)
#define TX_ERR ((MBOX_STATUS >> 4) & 1)
#define RX_ERR ((MBOX_STATUS >> 5) & 1)

#define EXPECT_RX_AVAIL(x) \
    if (STATUS_RX_AVAIL)   \
    {                      \
        x = MBOX_RDATA;    \
    }                      \
    else                   \
    {                      \
        return -1;         \
    }
#define EXPECT_TX_AVAIL(x) \
    if (STATUS_TX_AVAIL)   \
    {                      \
        MBOX_WDATA = x;    \
    }                      \
    else                   \
    {                      \
        return -1;         \
    }

#define MBOX_ABORT (*((volatile uint32_t *)0x40013018))

#define MBOX_DONE (*((volatile uint32_t *)0x4001301C))
#define TRIGGER_DONE MBOX_DONE = 1

#define MAX_PKT_LEN 128

#define MBOX_AVAIL_NVIC ((IRQn_Type)18)
#define MBOX_ABORT_NVIC ((IRQn_Type)19)

const VECTOR_TABLE_Type __VECTOR_TABLE[36] __VECTOR_TABLE_ATTRIBUTE = {
    (VECTOR_TABLE_Type)(&__INITIAL_SP), /* 0 Initial Stack Pointer */
    Reset_Handler,                      /* 1 Reset Handler */
    NMI_Handler,                        /* 2 NMI Handler */
    nothing,                            /* 3 Hard Fault Handler */
    nothing,                            /* 4 MPU Fault Handler */
    nothing,                            /* 5 Bus Fault Handler */
    nothing,                            /* 6 Usage Fault Handler */
    nothing,                            // 7
    nothing,                            // 8
    nothing,                            // 9
    nothing,                            // 10
    nothing,                            // 11
    nothing,                            // 12
    nothing,                            // 13
    PendSV_Handler,                     // 14 <<< CORRECT PendSV Handler
    SysTick_Handler,                    // 15 <<< CORRECT SysTick Handler
    nothing,                            // NVIC IRQ0
    nothing,                            // NV1
    nothing,                            // NV2
    nothing,                            // NV3
    nothing,                            // NV4
    nothing,                            // NV5
    nothing,                            // NV6
    nothing,                            // NV7
    nothing,                            // NV8
    nothing,                            // NV9
    nothing,                            // NV10
    nothing,                            // NV11
    nothing,                            // NV12
    nothing,                            // NV13
    nothing,                            // NV14
    nothing,                            // NV15
    nothing,                            // NV16
    nothing,                            // NV17
    Mbox_Handler,                       // NV18 -> mbox_available
    Mbox_Abort,                         // NV19 -> mbox abort
};

#if defined(__GNUC__)
#pragma GCC diagnostic pop
#endif

size_t const IFRAMSIZE = 1024UL * 128UL * 2UL; // 128kiB * 2
volatile uint64_t *const IFRAM64 = (uint64_t *)0x50000000;

size_t const SRAMSIZE = 1024UL * 1024UL * 2UL; // 1 MiB * 2
volatile uint64_t *const SRAM64 = (uint64_t *)0x61000000;

static const char *const HEX = "0123456789abcdef";
uint32_t SystemCoreClock = 800000000;

__attribute__((always_inline)) inline void print_string(const char *s)
{
    char c;
    size_t i = 0;
    while ((c = s[i++]) != 0)
    {
        __uart_putchar(c);
    }
    __uart_putchar('\n');
};

void send_u32_hex(uint32_t x)
{
    for (uint8_t i = 0; i < 8; i++)
    {
        __uart_putchar(HEX[(x >> 28) & 0x0f]);
        x <<= 4;
    }
    __uart_putchar(' ');
}

void __uart_putchar(char ch)
{
    uint32_t timeout = 0x10000;
    while ((DUART->BUSY != 0) && (timeout != 0))
    {
        timeout--;
    }
    DUART->TX = ch;
    __DSB();
}

void enable_fpu()
{
    SCB->CPACR |= 0x00F00000; /* set CP10 and CP11 Full Access */
    __DSB();
}

void Reset_Handler(void)
{
    // Now, continue with your original hardware setup
    for (volatile int i = 0; i < 5000000; i++)
    {
        __asm__("nop");
    }

    for (int i = 0; i < 10; i++)
    {
        print_string("Hello from CM7!\r");
    }

    // --- START: C/C++ DATA INITIALIZATION ---
    // This code must run before any other C code.

    // 1. Copy the .data section from Flash to RAM
    uint32_t *pSrc = &__etext;
    uint32_t *pDest = &__data_start__;
    while (pDest < &__data_end__)
    {
        *pDest++ = *pSrc++;
    }

    // 2. Zero out the .bss section in RAM
    pDest = &__bss_start__;
    while (pDest < &__bss_end__)
    {
        *pDest++ = 0;
    }
    // --- END: C/C++ DATA INITIALIZATION ---

    *((volatile uint32_t *)0x4001400C) = 0x8;

    *((volatile uint32_t *)0x40014000) = 0x3; // sramcfg.cach:ema[2:0]=0x4 (default for 0.8V), 0x3 for 0.9V
    *((volatile uint32_t *)0x40014014) = 0x1; // sramcfg.vexram:ema[2:0]=0x4 (default for 0.8V), 0x1 for 0.9V

    *((unsigned int *)0x40014004) = 5;
    *((unsigned int *)0x40014008) = 5;
    NVIC_SetPriority(MBOX_AVAIL_NVIC, 1);
    NVIC_EnableIRQ(MBOX_AVAIL_NVIC);
    NVIC_SetPriority(MBOX_ABORT_NVIC, 1);
    NVIC_EnableIRQ(MBOX_ABORT_NVIC);

    __enable_irq();

    main_loop();
}

void nothing() {}

void NMI_Handler()
{
}
void _init(void)
{
}

void Mbox_Abort()
{
    print_string("Abort\r");
    MBOX_ABORT = 0x1;
    NVIC->ICPR[MBOX_ABORT_NVIC >> 5] = (1 << (MBOX_ABORT_NVIC & 0x1F));
}

void Mbox_Handler()
{
    uint32_t target_addr = 0;
    uint32_t target_len = 0;

    mbox_pkt_t mbox_pkt;
    uint32_t packet_data[MAX_PKT_LEN];
    mbox_pkt.version = 0;
    mbox_pkt.opcode = TO_CM7_OP_INVALID;
    mbox_pkt.data = (uint32_t *)packet_data;
    for (int i = 0; i < MAX_PKT_LEN; i++)
    {
        packet_data[i] = 0;
    }

    mbox_pkt_t resp_pkt;
    uint32_t resp_data[MAX_PKT_LEN];
    resp_pkt.version = 0;
    resp_pkt.opcode = TO_RV_OP_INVALID;
    resp_pkt.data = (uint32_t *)resp_data;
    for (int i = 0; i < MAX_PKT_LEN; i++)
    {
        resp_data[i] = 0;
    }

    uint32_t rx_len = 0;
    print_string("RX available detected\r");
    rx_len = deserialize_rx(&mbox_pkt);
    if (rx_len >= 0)
    {
        send_u32_hex((uint32_t)mbox_pkt.opcode);
        __uart_putchar('\r');
        __uart_putchar('\n');
        switch (mbox_pkt.opcode)
        {
        case TO_CM7_OP_KNOCK:
            print_string("Rx CM7_OP_KNOCK\r");
            uint32_t retval = 0;
            for (int i = 0; i < mbox_pkt.arg_len; i++)
            {
                retval ^= mbox_pkt.data[i];
            }
            resp_pkt.opcode = TO_RV_OP_RET_KNOCK;
            resp_pkt.arg_len = 1;
            resp_pkt.data[0] = retval;
            serialize_tx(&resp_pkt);
            break;
        case TO_CM7_OP_DCT_8X8:
            print_string("DCT8x8\r");
            int8_t data_in[8][8];
            int16_t data_out[8][8];
            for (int i = 0; i < 16; i++)
            {
                ((uint32_t *)data_in)[i] = mbox_pkt.data[i];
            }
            dct_naive(data_in, data_out);
            resp_pkt.opcode = TO_RV_OP_RET_DCT_8X8;
            resp_pkt.arg_len = 32;
            for (int i = 0; i < 32; i++)
            {
                resp_pkt.data[i] = ((uint32_t *)data_out)[i];
            }
            serialize_tx(&resp_pkt);
            break;
        case TO_CM7_OP_CLIFFORD:
            print_string("CLIFFORD\r");
            uint8_t *buf = (uint8_t *)mbox_pkt.data[0];
            send_u32_hex((uint32_t)buf);
            clifford(buf);
            resp_pkt.opcode = TO_RV_OP_RET_CLIFFORD;
            resp_pkt.arg_len = 0;
            serialize_tx(&resp_pkt);
            break;
        case TO_CM7_OP_FLASHWRITE:
            print_string("FLASHWRITE\r");
            target_addr = *((uint32_t *)&mbox_pkt.data[0]);
            target_len = *((uint32_t *)&mbox_pkt.data[1]);
            if ((target_addr >= 0x60000000) && (target_addr < 0x60400000) && (target_len < 4088))
            {
                ReramWrite(target_addr, (uint8_t *)&mbox_pkt.data[2], target_len);
                resp_pkt.opcode = TO_RV_OP_RET_FLASHWRITE;
                resp_pkt.arg_len = 1;
                resp_pkt.data[0] = target_len;
                serialize_tx(&resp_pkt);
            }
            else
            {
                resp_pkt.opcode = TO_RV_OP_RET_FLASHWRITE;
                resp_pkt.arg_len = 1;
                resp_pkt.data[0] = 0;
                serialize_tx(&resp_pkt);
            }
            break;
        case TO_CM7_OP_INVALID:
            print_string("Rx CM7_OP_INVALID\r");
            break;
        default:
            print_string("DEFAULT\r");
            break;
        }
    }
    else
    {
        print_string("Rx failure\r");
        send_u32_hex(rx_len);
        print_string("\r\n");
    }
    NVIC->ICPR[MBOX_AVAIL_NVIC >> 5] = (1 << (MBOX_AVAIL_NVIC & 0x1F));
}

int32_t serialize_tx(mbox_pkt_t *resp_pkt)
{
    if (resp_pkt->arg_len <= MAX_PKT_LEN)
    {
        EXPECT_TX_AVAIL(resp_pkt->version)
        EXPECT_TX_AVAIL(((uint32_t)resp_pkt->opcode) | (((uint32_t)resp_pkt->arg_len) << 16))
        for (int i = 0; i < resp_pkt->arg_len; i++)
        {
            EXPECT_TX_AVAIL(resp_pkt->data[i])
        }
        TRIGGER_DONE;
        return (int32_t)resp_pkt->arg_len;
    }
    else
    {
        return -1;
    }
}

int32_t deserialize_rx(mbox_pkt_t *mbox_pkt)
{
    uint32_t word;
    mbox_pkt->version = MBOX_RDATA;
    EXPECT_RX_AVAIL(word)
    mbox_pkt->opcode = (uint16_t)(word & 0xFFFF);
    mbox_pkt->arg_len = (uint16_t)((word >> 16) & 0xFFFF);
    if (mbox_pkt->arg_len <= MAX_PKT_LEN)
    {
        for (int i = 0; i < mbox_pkt->arg_len; i++)
        {
            EXPECT_RX_AVAIL(mbox_pkt->data[i])
        }
    }
    else
    {
        return -1;
    }
    while (STATUS_RX_AVAIL != 0)
    {
        uint32_t dummy = MBOX_RDATA;
        print_string("Extra Rx:\r");
        send_u32_hex(dummy);
        __uart_putchar('\r');
    }
    return (int32_t)mbox_pkt->arg_len;
}

void clifford(uint8_t *buf)
{
    uint32_t WIDTH = 128;
    uint32_t HEIGHT = 128;
    float X_CENTER = (WIDTH / 2.0);
    float Y_CENTER = (HEIGHT / 2.0);
    float SCALE = WIDTH / 5.1;
    uint8_t STEP = 16;
    uint32_t ITERATIONS = 200000;
    float a = -2.0;
    float b = -2.4;
    float c = 1.1;
    float d = -0.9;
    float x = 0.0;
    float y = 0.0;
    float x1 = 0.0;
    float y1 = 0.0;

    for (int i = 0; i < WIDTH * HEIGHT; i++)
    {
        buf[i] = 255;
    }

    print_string("generator iteration: ");
    for (int i = 0; i < ITERATIONS; i++)
    {
        if ((i % 4096) == 0)
        {
            send_u32_hex(i);
        }
        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        uint32_t a_prime = lround(x * SCALE + X_CENTER);
        uint32_t b_prime = lround(y * SCALE + Y_CENTER);
        uint32_t index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP)
        {
            buf[index] -= STEP;
        }
    }
    __DSB();
}

static void print_test_result(const char *test_name, int success)
{
    // print_string("Test - ");
    print_string(test_name);
    // print_string(": ");
    if (success)
    {
        print_string("SUCCESS\r\n");
    }
    else
    {
        print_string("FAILURE\r\n");
    }
}

void run_libc_tests()
{
    print_string("\r\n--- Running Libc Integration Tests ---\r\n");

    // // Test 1: Simple malloc and free
    // char *test_str = (char *)malloc(20);
    // print_string("Test 1: malloc(20) returned address: ");
    // send_u32_hex((uint32_t)test_str);
    // print_string("\r\n");
    // print_test_result("malloc not NULL", test_str != NULL);
    // if (test_str)
    // {
    //     free(test_str);
    // }

    // // Test 2: Malloc, write, read, and free
    // int *test_int_ptr = (int *)malloc(sizeof(int));
    // int test_int_success = 0;
    // if (test_int_ptr)
    // {
    //     *test_int_ptr = 0xCAFEFACE;
    //     if (*test_int_ptr == 0xCAFEFACE)
    //     {
    //         test_int_success = 1;
    //     }
    //     free(test_int_ptr);
    // }
    // print_test_result("malloc, write, read", test_int_success);

    // // Test 3: Realloc
    // int realloc_success = 0;
    // char *realloc_ptr = (char *)malloc(10);
    // if (realloc_ptr)
    // {
    //     strcpy(realloc_ptr, "testing");
    //     char *realloc_ptr2 = (char *)realloc(realloc_ptr, 100);
    //     if (realloc_ptr2 && strcmp(realloc_ptr2, "testing") == 0)
    //     {
    //         realloc_success = 1;
    //     }
    //     free(realloc_ptr2);
    // }
    // print_test_result("realloc", realloc_success);

    // Test 4: snprintf
    char buffer[100];
    int val = 12345;
    snprintf(buffer, 100, "The magic number is %d!", val);
    print_string("Test 4: snprintf result: ");
    print_string(buffer);
    print_string("\r\n");
    print_test_result("snprintf format", strcmp(buffer, "The magic number is 12345!") == 0);

    // Test 5: sscanf
    int sscanf_success = 0;
    char sscanf_src[] = "CMD:1,VAL:987";
    int cmd_val = 0, val_val = 0;
    int items_scanned = sscanf(sscanf_src, "CMD:%d,VAL:%d", &cmd_val, &val_val);
    if (items_scanned == 2 && cmd_val == 1 && val_val == 987)
    {
        sscanf_success = 1;
    }
    print_test_result("sscanf parsing", sscanf_success);

    print_string("--- Libc Integration Tests Complete ---\r\n\r\n");
}

static void Platform_Init(void)
{
    /* MPU configuration table, defined in config_mpu_default.h */
    static ARM_MPU_Region_t mpu_config_table[] = DARIC_MPU_CONFIG;

    /* Disable MPU */
    ARM_MPU_Disable();

    /* Load the new MPU configuration */
#if 0
    ARM_MPU_Load(&mpu_config_table[0],
                 sizeof(mpu_config_table) / sizeof(mpu_config_table[0]));

    /* Enable MPU with default private memory background access */
    ARM_MPU_Enable(MPU_CTRL_PRIVDEFENA_Msk);
#endif
}

// Add this function to debug your MPU configuration
void debug_mpu_regions(void)
{
    print_string("=== MPU Region Debug ===\r\n");

    for (int i = 0; i < 8; i++)
    {
        // Select region
        MPU->RNR = i;

        print_string("Region ");
        send_u32_hex(i);
        print_string(": RBAR=");
        send_u32_hex(MPU->RBAR);
        print_string(" RASR=");
        send_u32_hex(MPU->RASR);
        print_string("\r\n");
    }

    print_string("MPU CTRL: ");
    send_u32_hex(MPU->CTRL);
    print_string("\r\n");
}

static void Platform_Init_MPU(void)
{
    print_string("Configuring MPU for safe caching (Write-Through)...\r\n");

    // This table is a copy of DARIC_MPU_CONFIG from the SDK.
    // We are now using the original Write-Through setting for SRAM.
    static ARM_MPU_Region_t mpu_config_table[] = {
        /* Region 0: ITCM, 256KB, Non-cacheable */
        {ARM_MPU_RBAR(0, 0x00000000), ARM_MPU_RASR(0, ARM_MPU_AP_RO, 1, 0, 0, 0, 0, ARM_MPU_REGION_SIZE_256KB)},
        /* Region 1: DTCM, 64KB, Non-cacheable */
        {ARM_MPU_RBAR(1, 0x20000000), ARM_MPU_RASR(1, ARM_MPU_AP_FULL, 1, 0, 0, 0, 0, ARM_MPU_REGION_SIZE_64KB)},
        /* Region 2: IFRAM, 256KB, Non-cacheable */
        {ARM_MPU_RBAR(2, 0x50000000), ARM_MPU_RASR(1, ARM_MPU_AP_FULL, 1, 0, 0, 0, 0, ARM_MPU_REGION_SIZE_256KB)},
        /* Region 3: ReRAM boot, 256B, Write-Through */
        {ARM_MPU_RBAR(3, 0x60000000), ARM_MPU_RASR(0, ARM_MPU_AP_RO, 0, 0, 1, 0, 0, ARM_MPU_REGION_SIZE_256B)},
        /* Region 4: ReRAM firmware, 2MB, Write-Through */
        {ARM_MPU_RBAR(4, 0x60040000), ARM_MPU_RASR(0, ARM_MPU_AP_RO, 0, 0, 1, 0, 0, ARM_MPU_REGION_SIZE_2MB)},
        /* Region 5: ReRAM data, 1MB, Write-Through */
        {ARM_MPU_RBAR(5, 0x60240000), ARM_MPU_RASR(0, ARM_MPU_AP_FULL, 0, 0, 1, 0, 0, ARM_MPU_REGION_SIZE_1MB)},
        /* Region 6: ReRAM nvram, 512KB, Write-Through */
        {ARM_MPU_RBAR(6, 0x60340000), ARM_MPU_RASR(0, ARM_MPU_AP_FULL, 0, 0, 1, 0, 0, ARM_MPU_REGION_SIZE_512KB)},
        /* Region 7: ReRAM aon, 256B, Write-Through */
        {ARM_MPU_RBAR(7, 0x603C0000), ARM_MPU_RASR(0, ARM_MPU_AP_RO, 0, 0, 1, 0, 0, ARM_MPU_REGION_SIZE_256B)},

        /* == REVERTED REGION == */
        /* Region 8: SRAM, 2MB, rw, Normal memory, Write-THROUGH, shareable */
        // Reverting B (Bufferable) bit from 1 to 0 to select Write-Through.
        {ARM_MPU_RBAR(8, 0x61000000), ARM_MPU_RASR(0, ARM_MPU_AP_FULL, 0, 1, 1, 0, 0, ARM_MPU_REGION_SIZE_2MB)},

        /* Region 9: Peripherals */
        {ARM_MPU_RBAR(9, 0x40000000), ARM_MPU_RASR(1, ARM_MPU_AP_FULL, 2, 0, 0, 0, 0, ARM_MPU_REGION_SIZE_1GB)}};

    ARM_MPU_Disable();
    ARM_MPU_Load(&mpu_config_table[0], sizeof(mpu_config_table) / sizeof(mpu_config_table[0]));
    ARM_MPU_Enable(MPU_CTRL_PRIVDEFENA_Msk);

    print_string("MPU configured successfully.\r\n");
}

void main_loop()
{
    // Basic CPU and C/C++ runtime setup
    enable_fpu();
    __libc_init_array();

    // 1. Initialize the MPU with our new, complete, and optimized configuration.
    // Platform_Init_MPU();

    // 2. Enable the Instruction Cache.
    // print_string("Enabling I-Cache...\r\n");
    // SCB_EnableICache();
    // print_string("I-Cache enabled.\r\n");

    // // 3. Enable the Data Cache.
    // print_string("Enabling D-Cache...\r\n");
    // SCB_EnableDCache();
    // print_string("hi\r\n");
    // if ((SCB->CCR & SCB_CCR_DC_Msk) == 0) // Only if it's off
    // {
    //     print_string("IN THE IF\r\n");
    //     SCB_InvalidateDCache(); // Good practice before enabling
    //     print_string("A\r\n");
    //     SCB->CCR |= SCB_CCR_DC_Msk;
    //     print_string("B\r\n");
    //     __DSB(); // Wait for memory operations to complete
    //     print_string("C\r\n");
    //     __ISB(); // Flush the pipeline
    //     print_string("D-Cache enabled.\r\n");
    // }

    // Now, enter the RTOS with caches fully enabled.
    print_string("Entering ThreadX...\r\n");
    tx_kernel_enter();

    // Should never reach here
    print_string("ERROR: Returned from ThreadX!\r\n");
}

#define ROUND_INT8(f) ((int8_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))
#define ROUND_INT16(f) ((int16_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))
#define ROUND_UINT8(f) ((uint8_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))
#define ROUND_UINT16(f) ((uint16_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))

const double cos_lookup[32] =
    {
        1.0, 0.9807852804032304, 0.9238795325112867, 0.8314696123025452,
        0.7071067811865475, 0.5555702330196022, 0.3826834323650897, 0.1950903220161282,
        0.0, -0.1950903220161282, -0.3826834323650897, -0.5555702330196022,
        -0.7071067811865475, -0.8314696123025452, -0.9238795325112867, -0.9807852804032304,
        -1.0, -0.9807852804032304, -0.9238795325112867, -0.8314696123025452,
        -0.7071067811865475, -0.5555702330196022, -0.3826834323650897, -0.1950903220161282,
        0.0, 0.1950903220161282, 0.3826834323650897, 0.5555702330196022,
        0.7071067811865475, 0.8314696123025452, 0.9238795325112867, 0.9807852804032304};

void dct_naive(int8_t data_in[8][8], int16_t data_out[8][8])
{
    int u, v, i, j;
    for (u = 0; u < 8; ++u)
    {
        double c_u = u == 0 ? SQRT_2_INV : 1.0;
        for (v = 0; v < 8; ++v)
        {
            double c_v = v == 0 ? SQRT_2_INV : 1.0;
            double outer_sum = 0;
            for (i = 0; i < 8; ++i)
            {
                double inner_sum = 0;
                double cos_u = cos_lookup[((2 * i + 1) * u) % 32];
                for (j = 0; j < 8; ++j)
                {
                    double cos_v = cos_lookup[((2 * j + 1) * v) % 32];
                    inner_sum += data_in[i][j] * cos_u * cos_v;
                }
                outer_sum += inner_sum;
            }
            double temp_result = c_u * c_v * outer_sum / 4;
            data_out[u][v] = ROUND_INT16(temp_result);
        }
    }
}