#include <stdint.h>
#include <stdlib.h>
#include <math.h>

#if defined (ARMCM7)
    #include "ARMCM7.h"
#elif defined (ARMCM7_SP)
    #include "ARMCM7_SP.h"
#elif defined (ARMCM7_DP)
    #include "ARMCM7_DP.h"
#else
    #error device not specified!
#endif

#include "daric_util.h"
#include "core_cm7.h"
#include "constants.h"
uint8_t ReramWrite(uint32_t dstAddr, uint8_t *pWtBuf, uint32_t wtLen);

#define USE_DELAY 0

#define SRAM_TEXT_START 0x61100000UL

extern uint32_t __INITIAL_SP;

typedef enum {
    TO_RV_OP_INVALID = 0,

    TO_RV_OP_RET_KNOCK = 128,
    TO_RV_OP_RET_DCT_8X8 = 129,
    TO_RV_OP_RET_CLIFFORD = 130,
    TO_RV_OP_RET_FLASHWRITE = 131,
} to_rv_op;

typedef enum {
    TO_CM7_OP_INVALID = 0,
    TO_CM7_OP_KNOCK = 1,
    TO_CM7_OP_DCT_8X8 = 2,
    TO_CM7_OP_CLIFFORD = 3,
    TO_CM7_OP_FLASHWRITE = 4,
} to_cm7_op;

typedef struct {
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

#define CCR     *((volatile uint32_t *) 0xE000ED14)

#define UNROLL 0

#if defined ( __GNUC__ )
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpedantic"
#endif

#define MBOX_WDATA  (*((volatile uint32_t *) 0x40013000))
#define MBOX_RDATA  (*((volatile uint32_t *) 0x40013004))

#define MBOX_STATUS (*((volatile uint32_t *) 0x40013008))
#define STATUS_RX_AVAIL  ((MBOX_STATUS >> 0) & 1)
#define STATUS_TX_AVAIL  ((MBOX_STATUS >> 1) & 1)
#define STATUS_ABORT_IN_PROGRESS   ((MBOX_STATUS >> 2) & 1)
#define STATUS_ABORT_ACK           ((MBOX_STATUS >> 3) & 1)
#define TX_ERR           ((MBOX_STATUS >> 4) & 1)
#define RX_ERR           ((MBOX_STATUS >> 5) & 1)

#define EXPECT_RX_AVAIL(x)  if (STATUS_RX_AVAIL) { x = MBOX_RDATA; } else { return -1; }
#define EXPECT_TX_AVAIL(x)  if (STATUS_TX_AVAIL) { MBOX_WDATA = x; } else { return -1; }

#define MBOX_ABORT  (*((volatile uint32_t *) 0x40013018))

#define MBOX_DONE   (*((volatile uint32_t *) 0x4001301C))
#define TRIGGER_DONE  MBOX_DONE = 1

#define MAX_PKT_LEN 128

#define MBOX_AVAIL_NVIC  ((IRQn_Type) 18)
#define MBOX_ABORT_NVIC  ((IRQn_Type) 19)

const VECTOR_TABLE_Type __VECTOR_TABLE[36] __VECTOR_TABLE_ATTRIBUTE = {
    (VECTOR_TABLE_Type)(&__INITIAL_SP),       /* 0 Initial Stack Pointer */
    Reset_Handler,                            /* 1 Reset Handler */
    NMI_Handler,                              /* 2 NMI Handler */
    nothing,                                  /* 3 Hard Fault Handler */
    nothing,                                  /* 4 MPU Fault Handler */
    nothing,                                  /* 5 Bus Fault Handler */
    nothing,                                  /* 6 Usage Fault Handler */
    nothing, // 7
    nothing, // 8
    nothing, // 9
    nothing, // 10
    nothing, // 11
    nothing, // 12
    nothing, // 13
    nothing, // 14
    nothing, // 15 (systick handler)
    nothing, // NVIC IRQ0
    nothing, // NV1
    nothing, // NV2
    nothing, // NV3
    nothing, // NV4
    nothing, // NV5
    nothing, // NV6
    nothing, // NV7
    nothing, // NV8
    nothing, // NV9
    nothing, // NV10
    nothing, // NV11
    nothing, // NV12
    nothing, // NV13
    nothing, // NV14
    nothing, // NV15
    nothing, // NV16
    nothing, // NV17
    Mbox_Handler, // NV18 -> mbox_available
    Mbox_Abort, // NV19 -> mbox abort
};

#if defined ( __GNUC__ )
#pragma GCC diagnostic pop
#endif

size_t const IFRAMSIZE = 1024UL * 128UL * 2UL; // 128kiB * 2
volatile uint64_t * const IFRAM64 = (uint64_t *)0x50000000;

size_t const SRAMSIZE = 1024UL * 1024UL * 2UL; // 1 MiB * 2
volatile uint64_t * const SRAM64 = (uint64_t *)0x61000000;

static const char * const HEX = "0123456789abcdef";
uint32_t SystemCoreClock = 800000000;

__attribute__((always_inline)) static inline
void print_string(const char *s) {
    char c;
    size_t i = 0;
    while ((c = s[i++]) != 0){
        __uart_putchar(c);
    }
    __uart_putchar('\n');
};

static void send_u32_hex(uint32_t x) {
    for (uint8_t i = 0 ; i < 8; i++){
        __uart_putchar(HEX[(x >> 28) & 0x0f]);
        x <<= 4;
    }
    __uart_putchar(' ');
}

void  __uart_putchar(char ch) {
    uint32_t timeout = 0x10000;
    while((DUART->BUSY != 0)  &&  (timeout != 0)){
        timeout--;
    }
    DUART->TX = ch;
    __DSB();
}

void enable_fpu() {
    // send_u32_hex(SCB->CPACR);
    // __uart_putchar('\n');
    // __uart_putchar('\r');
    SCB->CPACR |= 0x00F00000; /* set CP10 and CP11 Full Access */
    __DSB();
    // send_u32_hex(SCB->CPACR);
    // __uart_putchar('\n');
    // __uart_putchar('\r');
}

void Reset_Handler(void) {
#if 0
    if (*((uint32_t *) 0x61100000) != 0xCAFEFACE) {
        *((uint32_t *) 0x61100000) = 0xCAFEFACE;
        __DSB();
        // uint32_t val = DUART->ETU;
        // __DSB();
        // initClockASIC(100000000, 0);
        initDUART(24);

        print_string("CLK SETUP\n\r");
        send_u32_hex(DUART->ETU);
        print_string("\n\r");

       // Flip a GPIO
        *((uint32_t *) (0x5012f000 + 0x8)) = 0x5550; // AFSEL
        *((uint32_t *) (0x5012f000 + 0x14c)) = 0x1803; // OESEL
        __DSB();
        for (int i = 0; i < 1000; i++) {
            *((uint32_t *) (0x5012f000 + 0x134)) ^= 2;
            __DSB();
        }
        /*
        setupTicks();
        for(int i = 0; i < 10; i++) {
            ticksDelay(10000);
        }
        */
        print_string("reboot.\n\r");
        uint32_t val = DUART->ETU;
        send_u32_hex(val);
        // reset the system
        *((uint32_t *) 0x40040080) = 0x55aa;
        // *((uint32_t *) 0x40040084) = 0x55aa;
        __DSB();
    } else {
        // uint32_t val = DUART->ETU;
        // __DSB();
        initDUART(24);
        print_string("OK\n\r");
        // send_u32_hex(val);
        // print_string("\n\r");
    }
#endif
    /*
    for (int i = 0; i < 1000; i++) {
        print_string("Hello from CM7!\r");
    } */
    *((unsigned int *) 0x40014004) = 5;
    *((unsigned int *) 0x40014008) = 5;
    NVIC_SetPriority(MBOX_AVAIL_NVIC, 1);
    NVIC_EnableIRQ(MBOX_AVAIL_NVIC);
    NVIC_SetPriority(MBOX_ABORT_NVIC, 1);
    NVIC_EnableIRQ(MBOX_ABORT_NVIC);

    __enable_irq();

    main_loop();
}

void nothing() {}

void NMI_Handler() {
    /*
    for (size_t register i = 0; i < IFRAMSIZE / sizeof(uint64_t); i++){
        IFRAM64[i] = 0;
    }
    __DSB();
    for (size_t register i = 0; i < SRAMSIZE / sizeof(uint64_t) - 4; i++){// keep stack
        SRAM64[i] = 0;
    }
    __DSB();
    */
}

void Mbox_Abort() {
    print_string("Abort\r");
    // ack the abort by setting this bit
    MBOX_ABORT = 0x1;
    // clear the pending bit
    NVIC->ICPR[MBOX_ABORT_NVIC >> 5] = (1 << (MBOX_ABORT_NVIC & 0x1F));
}

void Mbox_Handler() {
    uint32_t target_addr = 0;
    uint32_t target_len = 0;

    // allocate incoming packet
    mbox_pkt_t mbox_pkt;
    uint32_t packet_data[MAX_PKT_LEN];
    mbox_pkt.version = 0;
    mbox_pkt.opcode = TO_CM7_OP_INVALID;
    mbox_pkt.data = (uint32_t *) packet_data;
    for (int i = 0; i < MAX_PKT_LEN; i++) {
        packet_data[i] = 0;
    }
    // allocate response packet
    mbox_pkt_t resp_pkt;
    uint32_t resp_data[MAX_PKT_LEN];
    resp_pkt.version = 0;
    resp_pkt.opcode = TO_RV_OP_INVALID;
    resp_pkt.data = (uint32_t *) resp_data;
    for (int i = 0; i < MAX_PKT_LEN; i++) {
        resp_data[i] = 0;
    }

    uint32_t rx_len = 0;
    print_string("RX available detected\r");
    rx_len = deserialize_rx(&mbox_pkt);
    if (rx_len >= 0) {
        send_u32_hex((uint32_t) mbox_pkt.opcode);
        __uart_putchar('\r');
        __uart_putchar('\n');
        switch (mbox_pkt.opcode) {
            case TO_CM7_OP_KNOCK:
                print_string("Rx CM7_OP_KNOCK\r");
                // This test just checks if the mailbox protocol even works
                // XOR all the values in the data field together, and return it
                uint32_t retval = 0;
                for (int i = 0; i < mbox_pkt.arg_len; i++) {
                    retval ^= mbox_pkt.data[i];
                }
                resp_pkt.opcode = TO_RV_OP_RET_KNOCK;
                resp_pkt.arg_len = 1;
                resp_pkt.data[0] = retval;
                serialize_tx(&resp_pkt);
                break;
            case TO_CM7_OP_DCT_8X8:
                print_string("DCT8x8\r");
                // this test checks if the CM7 can be used to outsource DSP ops
                int8_t data_in[8][8];
                int16_t data_out[8][8]; // this is uninit: assume dct_naive fully populates all values!
                // super dangerous deserialization
                for (int i = 0; i < 16; i++) {
                    ((uint32_t *) data_in)[i] = mbox_pkt.data[i];
                }
                dct_naive(data_in, data_out);
                resp_pkt.opcode = TO_RV_OP_RET_DCT_8X8;
                resp_pkt.arg_len = 32;
                // super dangerous serialization
                for (int i = 0; i < 32; i ++) {
                    resp_pkt.data[i] = ((uint32_t *) data_out)[i];
                }
                serialize_tx(&resp_pkt);
                break;
            case TO_CM7_OP_CLIFFORD:
                print_string("CLIFFORD\r");
                // the output buffer is passed as a pointer to physical memory
                // this test checks simultaneous access to main memory
                uint8_t *buf = (uint8_t *) mbox_pkt.data[0];
                send_u32_hex((uint32_t) buf);
                // compute the clifford attractor
                clifford(buf);
                // notify the caller that we finished
                resp_pkt.opcode = TO_RV_OP_RET_CLIFFORD;
                resp_pkt.arg_len = 0;
                serialize_tx(&resp_pkt);
                break;
            case TO_CM7_OP_FLASHWRITE:
                print_string("FLASHWRITE\r");
                // packet data format:
                // first word is the target address
                // second word is the length to write, *in bytes*
                // remaining words are the data
                target_addr = *((uint32_t *) &mbox_pkt.data[0]);
                target_len = *((uint32_t *) &mbox_pkt.data[1]);
                if ((target_addr >= 0x60000000) && (target_addr < 0x60400000) && (target_len < 4088)) {
                    ReramWrite(target_addr, (uint8_t *) &mbox_pkt.data[2], target_len);
                    resp_pkt.opcode = TO_RV_OP_RET_FLASHWRITE;
                    resp_pkt.arg_len = 1;
                    resp_pkt.data[0] = target_len;
                    serialize_tx(&resp_pkt);
                } else {
                    resp_pkt.opcode = TO_RV_OP_RET_FLASHWRITE;
                    resp_pkt.arg_len = 1;
                    resp_pkt.data[0] = 0;
                    serialize_tx(&resp_pkt);
                }
                break;
            case TO_CM7_OP_INVALID:
                print_string("Rx CM7_OP_INVALID\r");
                // do nothing for now
                break;
            default:
                print_string("DEFAULT\r");
                break;
        }
    } else {
        print_string("Rx failure\r");
        send_u32_hex(rx_len);
        print_string("\r\n");
    }
    // clear the pending bit
    NVIC->ICPR[MBOX_AVAIL_NVIC >> 5] = (1 << (MBOX_AVAIL_NVIC & 0x1F));
}

// Transmit a packet through the mailbox.
// Arguments: pointer to the outgoing packet
// Returns:
//    Success: length of the data section of the outgoing packet (range 0:MAX_PKT_LEN)
//    Error: a negative value
int32_t serialize_tx(mbox_pkt_t *resp_pkt) {
    if (resp_pkt->arg_len <= MAX_PKT_LEN) {
        EXPECT_TX_AVAIL(resp_pkt->version)
        EXPECT_TX_AVAIL(((uint32_t)resp_pkt->opcode) | (((uint32_t)resp_pkt->arg_len) << 16))
        for (int i = 0; i < resp_pkt->arg_len; i++) {
            EXPECT_TX_AVAIL(resp_pkt->data[i])
        }
        TRIGGER_DONE;
        return (int32_t) resp_pkt->arg_len;
    } else {
        return -1;
    }
}

// Receive a packet through the mailbox.
// Arguments: storage for the incoming packet.
// Returns:
//    Success: length of the data section of the incoming packet (range 0:MAX_PKT_LEN)
//    Error: a negative value
int32_t deserialize_rx(mbox_pkt_t *mbox_pkt) {
    uint32_t word;
    mbox_pkt->version = MBOX_RDATA;
    EXPECT_RX_AVAIL(word)
    mbox_pkt->opcode = (uint16_t) (word & 0xFFFF);
    mbox_pkt->arg_len = (uint16_t) ((word >> 16) & 0xFFFF);
    if (mbox_pkt->arg_len <= MAX_PKT_LEN) {
        for (int i = 0; i < mbox_pkt->arg_len; i++) {
            EXPECT_RX_AVAIL(mbox_pkt->data[i])
        }
    } else {
        return -1;
    }
    while (STATUS_RX_AVAIL != 0) {
        uint32_t dummy = MBOX_RDATA;
        print_string("Extra Rx:\r");
        send_u32_hex(dummy);
        __uart_putchar('\r');
    }
    return (int32_t) mbox_pkt->arg_len;
}

void clifford(uint8_t *buf) {
    uint32_t WIDTH = 128;
    uint32_t HEIGHT = 128;
    // width & height chosen to force resize & rotation
    float X_CENTER = (WIDTH / 2.0);
    float Y_CENTER = (HEIGHT / 2.0);
    float SCALE = WIDTH / 5.1;
    uint8_t STEP = 16;
#if UNROLL
    uint32_t ITERATIONS = 200000 / 16;
#else
    uint32_t ITERATIONS = 200000;
#endif
    float a = -2.0;
    float b = -2.4;
    float c = 1.1;
    float d = -0.9;
    float x = 0.0;
    float y = 0.0;
    float x1 = 0.0;
    float y1 = 0.0;

    // initialize values
    for (int i = 0; i < WIDTH * HEIGHT; i++) {
        buf[i] = 255;
    }

    // enable caches -- this crashes the system.
    /*
    print_string("cache config: ");
    send_u32_hex(CCR);
    SCB_EnableICache();
    SCB_EnableDCache();
    print_string("cache config: ");
    send_u32_hex(CCR);
    */

    print_string("generator iteration: ");
    for (int i = 0; i < ITERATIONS; i++) {
        // invalidate all caches to force bus traffic
        /*
        (*(volatile int *) 0xE000EF50) = 0; // icache
        (*(volatile int *) 0xE000EF5C) = 0; // dcache
        __DSB();
        __ISB();
        */

        if ((i % 4096) == 0) {
            send_u32_hex(i);
        }
        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        uint32_t a_prime = lround(x * SCALE + X_CENTER);
        uint32_t b_prime = lround(y * SCALE + Y_CENTER);
        uint32_t index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        // aggressively unroll the loop to force
        // instruction traffic to memory
#if UNROLL
        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

        x1 = sin(a * y) + c * cos(a * x);
        y1 = sin(b * x) + d * cos(b * y);
        x = x1;
        y = y1;
        a_prime = lround(x * SCALE + X_CENTER);
        b_prime = lround(y * SCALE + Y_CENTER);
        index = a_prime + WIDTH * b_prime;
        if (buf[index] >= STEP) {
            buf[index] -= STEP;
        }

#endif
    }
    __DSB();
}

void main_loop() {
    setupTicks();
#if USE_DELAY
    for (int i = 0; i < 15000; i++) {
        ticksDelay(10000);
        // resetTicks();
    }
    for (int i = 0; i < 5; i++) {
        print_string("CM7 power on delay done\r");
    }
#else
    ticksDelay(10000);
    print_string("CM7 up\r\r");
#endif
    enable_fpu();

    while (1) {
        __WFI();
    }
}


#define ROUND_INT8(f) ((int8_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))
#define ROUND_INT16(f) ((int16_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))
#define ROUND_UINT8(f) ((uint8_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))
#define ROUND_UINT16(f) ((uint16_t)(f >= 0.0 ? (f + 0.5) : (f - 0.5)))

// 32-element lookup table.
// cos_lookup[x] == cos(x * pi / 16)
const double cos_lookup[32] =
{
    1.0, // cos(0)
    0.980785280403230449126182236134239036973933730893336095002, // pi/16
    0.923879532511286756128183189396788286822416625863642486115, // 2pi/16
    0.831469612302545237078788377617905756738560811987249963446, // 3pi/16
    0.707106781186547524400844362104849039284835937688474036588, // 4pi/16
    0.555570233019602224742830813948532874374937190754804045924, // 5pi/16
    0.382683432365089771728459984030398866761344562485627041433, // 6pi/16
    0.195090322016128267848284868477022240927691617751954807754, // 7pi/16
    0.0, // cos(pi/2)
    -0.195090322016128267848284868477022240927691617751954807754, // 9pi/16
    -0.382683432365089771728459984030398866761344562485627041433, // 10pi/16
    -0.555570233019602224742830813948532874374937190754804045924, // 11pi/16
    -0.707106781186547524400844362104849039284835937688474036588, // 12pi/16
    -0.831469612302545237078788377617905756738560811987249963446, // 13pi/16
    -0.923879532511286756128183189396788286822416625863642486115, // 14pi/16
    -0.980785280403230449126182236134239036973933730893336095002, // 15pi/16
    -1.0, // cos(pi)
    -0.980785280403230449126182236134239036973933730893336095002,
    -0.923879532511286756128183189396788286822416625863642486115,
    -0.831469612302545237078788377617905756738560811987249963446,
    -0.707106781186547524400844362104849039284835937688474036588,
    -0.555570233019602224742830813948532874374937190754804045924,
    -0.382683432365089771728459984030398866761344562485627041433,
    -0.195090322016128267848284868477022240927691617751954807754,
    0.0, // cos(3pi/2)
    0.195090322016128267848284868477022240927691617751954807754,
    0.382683432365089771728459984030398866761344562485627041433,
    0.555570233019602224742830813948532874374937190754804045924,
    0.707106781186547524400844362104849039284835937688474036588,
    0.831469612302545237078788377617905756738560811987249963446,
    0.923879532511286756128183189396788286822416625863642486115,
    0.980785280403230449126182236134239036973933730893336095002
};

// input: 8x8 array, output: 8x8 array.
// only optimization is the cosine lookup table.
void dct_naive(int8_t data_in[8][8], int16_t data_out[8][8])
{
    int u, v, i, j;
    // X(u,v) = (C(u)/2)*(C(v)/2) * sigma[i=0 to 7]( sigma[j=0 to 7]( x(i,j)*cos((2i+1)*u*pi/16)*cos((2j+1)*v*pi/16) ) )
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
                double cos_u = cos_lookup[((2*i + 1) * u) % 32];
                for (j = 0; j < 8; ++j)
                {
                    double cos_v = cos_lookup[((2*j + 1) * v) % 32];
                    inner_sum += data_in[i][j] * cos_u * cos_v;
                }
                outer_sum += inner_sum;
            }
            // NB: this result could be outside [-128, 127]; it will fail in that case.
            double temp_result = c_u * c_v * outer_sum / 4;
            data_out[u][v] = ROUND_INT16(temp_result);
        }
    }
}