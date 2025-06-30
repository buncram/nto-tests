#include "tx_api.h"
#include "mbox.h" // For run_libc_tests() and print_string()
#include "daric_hal.h"

/*
 * This is the memory pool that our custom malloc() in libc-hooks.c
 * will use. It must be defined globally here.
 */
TX_BYTE_POOL DefaultHeap;

/**
 * @brief  This is the main entry point for defining ThreadX objects.
 * The kernel calls this function during startup.
 * @param  first_unused_memory: Pointer to the first unused memory address.
 * @retval None
 */
void tx_application_define(void *first_unused_memory)
{
    UINT status;

/*
 * Define the heap area. It's important that this memory is not used for
 * anything else (like the stack or global variables). A large static
 * array is a safe way to reserve it.
 */
#define MALLOC_HEAP_SIZE (16 * 1024) // 16KB heap
    static UCHAR malloc_heap_memory[MALLOC_HEAP_SIZE];

    /*
     * Create the default heap memory pool for malloc() to use.
     */
    status = tx_byte_pool_create(&DefaultHeap,
                                 "Default Heap",
                                 malloc_heap_memory,
                                 MALLOC_HEAP_SIZE);

    /* If the heap fails, we can't continue. */
    if (status != TX_SUCCESS)
    {
        return;
    }

/* --- Test Thread Objects --- */
#define TEST_THREAD_STACK_SIZE 8192 // 8KB stack for safety
    static TX_THREAD libc_test_thread;
    static UCHAR libc_test_thread_stack[TEST_THREAD_STACK_SIZE];

    /* Entry function for the test thread */
    void libc_test_thread_entry(ULONG thread_input)
    {
        print_string("\r\nSUCCESS: Libc test thread has started.\r\n");

        // Wait a moment for ThreadX to fully stabilize
        tx_thread_sleep(50);

        // Re-enable DCache now that ThreadX is running
        print_string("Re-enabling DCache from ThreadX task...\r\n");
        SCB_EnableDCache();
        print_string("DCache re-enabled successfully!\r\n");

        // Now run the libc tests with DCache enabled
        run_libc_tests();

        /* Loop forever after tests are done */
        while (1)
        {
            tx_thread_sleep(500);
        }
    }

    /* Create the thread that will run our libc tests */
    status = tx_thread_create(&libc_test_thread,
                              "LibC Test Thread",
                              libc_test_thread_entry,
                              0, // No input value
                              libc_test_thread_stack,
                              TEST_THREAD_STACK_SIZE,
                              15, // Priority
                              15, // Preemption-Threshold
                              TX_NO_TIME_SLICE,
                              TX_AUTO_START); // Start the thread automatically
}