#include "tx_api.h"
#include "mbox.h"
#include "daric_hal.h"
#include "tflm_hello_world_wrapper.h"

// Declare the C-linkage function from the TFLM wrapper
extern void run_tflm_hello_world_test(void);

/*
 * This is the memory pool that our custom malloc() in libc-hooks.c
 * will use. It must be defined globally here.
 */
TX_BYTE_POOL DefaultHeap;

void tx_application_define(void *first_unused_memory)
{
    UINT status;

#define MALLOC_HEAP_SIZE (16 * 1024)
    static UCHAR malloc_heap_memory[MALLOC_HEAP_SIZE];

    status = tx_byte_pool_create(&DefaultHeap, "Default Heap",
                                 malloc_heap_memory, MALLOC_HEAP_SIZE);

    if (status != TX_SUCCESS)
    {
        print_string("ERROR: Failed to create ThreadX byte pool for malloc!\r\n");
        return;
    }

    print_string("Enabling DCache from tx_application_define()...\r\n");
    SCB_EnableDCache();
    if ((SCB->CCR & SCB_CCR_DC_Msk) != 0)
    {
        SCB_CleanDCache();
        print_string("DCache enabled and cleaned successfully!\r\n");
    }

/* --- Main Test Thread Objects --- */
// Use the larger stack size required by the TFLM test.
#define MAIN_TEST_THREAD_STACK_SIZE 8192
    static TX_THREAD main_test_thread;
    static UCHAR main_test_thread_stack[MAIN_TEST_THREAD_STACK_SIZE];

    // This function will now run both tests sequentially.
    void main_test_thread_entry(ULONG thread_input)
    {
        print_string("\r\nSUCCESS: Main test thread has started.\r\n");
        tx_thread_sleep(50);

        // 1. Run Libc tests
        run_libc_tests();

        // 2. Run TFLM Hello World test
        print_string("\r\nSUCCESS: Starting TFLM Hello World test.\r\n");
        run_tflm_hello_world_test();
        print_string("\r\nSUCCESS: TFLM Hello World test completed.\r\n");

        // All tests are done, enter an infinite loop.
        while (1)
        {
            tx_thread_sleep(1000);
        }
    }

    // Create a single thread to run all tests.
    status = tx_thread_create(&main_test_thread, "Main Test Thread", main_test_thread_entry,
                              0, main_test_thread_stack, MAIN_TEST_THREAD_STACK_SIZE,
                              15, 15, TX_NO_TIME_SLICE, TX_AUTO_START);

    if (status != TX_SUCCESS)
    {
        print_string("ERROR: Failed to create Main Test Thread!\r\n");
    }

    /* --- The TFLM Hello World thread has been removed as it is no longer needed. --- */
}