# vim: noet:ts=4:sw=4:

fullmf := $(abspath $(lastword $(MAKEFILE_LIST)))
pathmf := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

# <<< NOTE: We are assuming this file provides TOOLCHAIN, INCLUDES, etc.
include makefile.daric.common

## Use g++ for C++ projects
LD        :=  $(TOOLCHAIN)g++

#############

TARGET = mbox

LD_FILES ?= $(pathmf)/gcc_daric_mbox.ld

# <<< MODIFIED: FIDO2 Integration Setup
FIDO2_ROOT := ../../threadx_mono

# <<< MODIFIED: Original project objects
PROJECT_OBJS = daric_util.o reram.o

# <<< MODIFIED: Add all source files from FIDO2/ThreadX integration
LIBC_HOOK_SRCS_C := libc-hooks.c

THREADX_SRCS_C := \
	init.c \
	tx_block_allocate.c \
	tx_block_pool_cleanup.c \
	tx_block_pool_create.c \
	tx_block_pool_delete.c \
	tx_block_pool_info_get.c \
	tx_block_pool_initialize.c \
	tx_block_pool_performance_info_get.c \
	tx_block_pool_performance_system_info_get.c \
	tx_block_pool_prioritize.c \
	tx_block_release.c \
	tx_byte_allocate.c \
	tx_byte_pool_cleanup.c \
	tx_byte_pool_create.c \
	tx_byte_pool_delete.c \
	tx_byte_pool_info_get.c \
	tx_byte_pool_initialize.c \
	tx_byte_pool_performance_info_get.c \
	tx_byte_pool_performance_system_info_get.c \
	tx_byte_pool_prioritize.c \
	tx_byte_pool_search.c \
	tx_byte_release.c \
	txe_block_allocate.c \
	txe_block_pool_create.c \
	txe_block_pool_delete.c \
	txe_block_pool_info_get.c \
	txe_block_pool_prioritize.c \
	txe_block_release.c \
	txe_byte_allocate.c \
	txe_byte_pool_create.c \
	txe_byte_pool_delete.c \
	txe_byte_pool_info_get.c \
	txe_byte_pool_prioritize.c \
	txe_byte_release.c \
	txe_event_flags_create.c \
	txe_event_flags_delete.c \
	txe_event_flags_get.c \
	txe_event_flags_info_get.c \
	txe_event_flags_set.c \
	txe_event_flags_set_notify.c \
	txe_mutex_create.c \
	txe_mutex_delete.c \
	txe_mutex_get.c \
	txe_mutex_info_get.c \
	txe_mutex_prioritize.c \
	txe_mutex_put.c \
	txe_queue_create.c \
	txe_queue_delete.c \
	txe_queue_flush.c \
	txe_queue_front_send.c \
	txe_queue_info_get.c \
	txe_queue_prioritize.c \
	txe_queue_receive.c \
	txe_queue_send.c \
	txe_queue_send_notify.c \
	txe_semaphore_ceiling_put.c \
	txe_semaphore_create.c \
	txe_semaphore_delete.c \
	txe_semaphore_get.c \
	txe_semaphore_info_get.c \
	txe_semaphore_prioritize.c \
	txe_semaphore_put.c \
	txe_semaphore_put_notify.c \
	txe_thread_create.c \
	txe_thread_delete.c \
	txe_thread_entry_exit_notify.c \
	txe_thread_info_get.c \
	txe_thread_preemption_change.c \
	txe_thread_priority_change.c \
	txe_thread_relinquish.c \
	txe_thread_reset.c \
	txe_thread_resume.c \
	txe_thread_suspend.c \
	txe_thread_terminate.c \
	txe_thread_time_slice_change.c \
	txe_thread_wait_abort.c \
	tx_event_flags_cleanup.c \
	tx_event_flags_create.c \
	tx_event_flags_delete.c \
	tx_event_flags_get.c \
	tx_event_flags_info_get.c \
	tx_event_flags_initialize.c \
	tx_event_flags_performance_info_get.c \
	tx_event_flags_performance_system_info_get.c \
	tx_event_flags_set.c \
	tx_event_flags_set_notify.c \
	tx_initialize_high_level.c \
	tx_initialize_kernel_enter.c \
	tx_initialize_kernel_setup.c \
	tx_misra.c \
	tx_mutex_cleanup.c \
	tx_mutex_create.c \
	tx_mutex_delete.c \
	tx_mutex_get.c \
	tx_mutex_info_get.c \
	tx_mutex_initialize.c \
	tx_mutex_performance_info_get.c \
	tx_mutex_performance_system_info_get.c \
	tx_mutex_prioritize.c \
	tx_mutex_priority_change.c \
	tx_mutex_put.c \
	tx_queue_cleanup.c \
	tx_queue_create.c \
	tx_queue_delete.c \
	tx_queue_flush.c \
	tx_queue_front_send.c \
	tx_queue_info_get.c \
	tx_queue_initialize.c \
	tx_queue_performance_info_get.c \
	tx_queue_performance_system_info_get.c \
	tx_queue_prioritize.c \
	tx_queue_receive.c \
	tx_queue_send.c \
	tx_queue_send_notify.c \
	tx_semaphore_ceiling_put.c \
	tx_semaphore_cleanup.c \
	tx_semaphore_create.c \
	tx_semaphore_delete.c \
	tx_semaphore_get.c \
	tx_semaphore_info_get.c \
	tx_semaphore_initialize.c \
	tx_semaphore_performance_info_get.c \
	tx_semaphore_performance_system_info_get.c \
	tx_semaphore_prioritize.c \
	tx_semaphore_put.c \
	tx_semaphore_put_notify.c \
	tx_thread_create.c \
	tx_thread_delete.c \
	tx_thread_entry_exit_notify.c \
	tx_thread_identify.c \
	tx_thread_info_get.c \
	tx_thread_initialize.c \
	tx_thread_performance_info_get.c \
	tx_thread_performance_system_info_get.c \
	tx_thread_preemption_change.c \
	tx_thread_priority_change.c \
	tx_thread_relinquish.c \
	tx_thread_reset.c \
	tx_thread_resume.c \
	tx_thread_shell_entry.c \
	tx_thread_sleep.c \
	tx_thread_stack_analyze.c \
	tx_thread_stack_error_handler.c \
	tx_thread_stack_error_notify.c \
	tx_thread_suspend.c \
	tx_thread_system_preempt_check.c \
	tx_thread_system_resume.c \
	tx_thread_system_suspend.c \
	tx_thread_terminate.c \
	tx_thread_timeout.c \
	tx_thread_time_slice.c \
	tx_thread_time_slice_change.c \
	tx_thread_wait_abort.c \
	tx_time_get.c \
	tx_timer_activate.c \
	tx_timer_change.c \
	tx_timer_create.c \
	tx_timer_deactivate.c \
	tx_timer_delete.c \
	tx_timer_expiration_process.c \
	tx_timer_info_get.c \
	tx_timer_initialize.c \
	tx_timer_performance_info_get.c \
	tx_timer_performance_system_info_get.c \
	tx_timer_system_activate.c \
	tx_timer_system_deactivate.c \
	tx_timer_thread_entry.c \
	tx_time_set.c \
	tx_trace_buffer_full_notify.c \
	tx_trace_disable.c \
	tx_trace_enable.c \
	tx_trace_event_filter.c \
	tx_trace_event_unfilter.c \
	tx_trace_initialize.c \
	tx_trace_interrupt_control.c \
	tx_trace_isr_enter_insert.c \
	tx_trace_isr_exit_insert.c \
	tx_trace_object_register.c \
	tx_trace_object_unregister.c \
	tx_trace_user_event_insert.c \
	platform_thread_attrdef.c \
	tx_log.c \
	tx_work_item_queue.c

# NOTE: If you do not need a file system, delete all the files in this list.
FILEX_SRCS_C := \
	reram_filex_app.c \
	fx_directory_attributes_read.c \
	fx_directory_attributes_set.c \
	fx_directory_create.c \
	fx_directory_default_get.c \
	fx_directory_default_get_copy.c \
	fx_directory_default_set.c \
	fx_directory_delete.c \
	fx_directory_entry_read.c \
	fx_directory_entry_write.c \
	fx_directory_first_entry_find.c \
	fx_directory_first_full_entry_find.c \
	fx_directory_free_search.c \
	fx_directory_information_get.c \
	fx_directory_local_path_clear.c \
	fx_directory_local_path_get.c \
	fx_directory_local_path_get_copy.c \
	fx_directory_local_path_restore.c \
	fx_directory_local_path_set.c \
	fx_directory_long_name_get.c \
	fx_directory_long_name_get_extended.c \
	fx_directory_name_extract.c \
	fx_directory_name_test.c \
	fx_directory_next_entry_find.c \
	fx_directory_next_full_entry_find.c \
	fx_directory_rename.c \
	fx_directory_search.c \
	fx_directory_short_name_get.c \
	fx_directory_short_name_get_extended.c \
	fxe_directory_attributes_read.c \
	fxe_directory_attributes_set.c \
	fxe_directory_create.c \
	fxe_directory_default_get.c \
	fxe_directory_default_get_copy.c \
	fxe_directory_default_set.c \
	fxe_directory_delete.c \
	fxe_directory_first_entry_find.c \
	fxe_directory_first_full_entry_find.c \
	fxe_directory_information_get.c \
	fxe_directory_local_path_clear.c \
	fxe_directory_local_path_get.c \
	fxe_directory_local_path_get_copy.c \
	fxe_directory_local_path_restore.c \
	fxe_directory_local_path_set.c \
	fxe_directory_long_name_get.c \
	fxe_directory_long_name_get_extended.c \
	fxe_directory_name_test.c \
	fxe_directory_next_entry_find.c \
	fxe_directory_next_full_entry_find.c \
	fxe_directory_rename.c \
	fxe_directory_short_name_get.c \
	fxe_directory_short_name_get_extended.c \
	fxe_fault_tolerant_enable.c \
	fxe_file_allocate.c \
	fxe_file_attributes_read.c \
	fxe_file_attributes_set.c \
	fxe_file_best_effort_allocate.c \
	fxe_file_close.c \
	fxe_file_create.c \
	fxe_file_date_time_set.c \
	fxe_file_delete.c \
	fxe_file_extended_allocate.c \
	fxe_file_extended_best_effort_allocate.c \
	fxe_file_extended_relative_seek.c \
	fxe_file_extended_seek.c \
	fxe_file_extended_truncate.c \
	fxe_file_extended_truncate_release.c \
	fxe_file_open.c \
	fxe_file_read.c \
	fxe_file_relative_seek.c \
	fxe_file_rename.c \
	fxe_file_seek.c \
	fxe_file_truncate.c \
	fxe_file_truncate_release.c \
	fxe_file_write.c \
	fxe_file_write_notify_set.c \
	fxe_media_abort.c \
	fxe_media_cache_invalidate.c \
	fxe_media_check.c \
	fxe_media_close.c \
	fxe_media_close_notify_set.c \
	fxe_media_extended_space_available.c \
	fxe_media_flush.c \
	fxe_media_format.c \
	fxe_media_open.c \
	fxe_media_open_notify_set.c \
	fxe_media_read.c \
	fxe_media_space_available.c \
	fxe_media_volume_get.c \
	fxe_media_volume_get_extended.c \
	fxe_media_volume_set.c \
	fxe_media_write.c \
	fxe_system_date_get.c \
	fxe_system_date_set.c \
	fxe_system_time_get.c \
	fxe_system_time_set.c \
	fxe_unicode_directory_create.c \
	fxe_unicode_directory_rename.c \
	fxe_unicode_file_create.c \
	fxe_unicode_file_rename.c \
	fxe_unicode_name_get.c \
	fxe_unicode_name_get_extended.c \
	fxe_unicode_short_name_get.c \
	fxe_unicode_short_name_get_extended.c \
	fx_fault_tolerant_add_bitmap_log.c \
	fx_fault_tolerant_add_checksum_log.c \
	fx_fault_tolerant_add_dir_log.c \
	fx_fault_tolerant_add_FAT_log.c \
	fx_fault_tolerant_apply_logs.c \
	fx_fault_tolerant_calculate_checksum.c \
	fx_fault_tolerant_cleanup_FAT_chain.c \
	fx_fault_tolerant_create_log_file.c \
	fx_fault_tolerant_enable.c \
	fx_fault_tolerant_read_directory_sector.c \
	fx_fault_tolerant_read_FAT.c \
	fx_fault_tolerant_read_log_file.c \
	fx_fault_tolerant_recover.c \
	fx_fault_tolerant_reset_log_file.c \
	fx_fault_tolerant_set_FAT_chain.c \
	fx_fault_tolerant_transaction_end.c \
	fx_fault_tolerant_transaction_fail.c \
	fx_fault_tolerant_transaction_start.c \
	fx_fault_tolerant_write_log_file.c \
	fx_file_allocate.c \
	fx_file_attributes_read.c \
	fx_file_attributes_set.c \
	fx_file_best_effort_allocate.c \
	fx_file_close.c \
	fx_file_create.c \
	fx_file_date_time_set.c \
	fx_file_delete.c \
	fx_file_extended_allocate.c \
	fx_file_extended_best_effort_allocate.c \
	fx_file_extended_relative_seek.c \
	fx_file_extended_seek.c \
	fx_file_extended_truncate.c \
	fx_file_extended_truncate_release.c \
	fx_file_open.c \
	fx_file_read.c \
	fx_file_relative_seek.c \
	fx_file_rename.c \
	fx_file_seek.c \
	fx_file_truncate.c \
	fx_file_truncate_release.c \
	fx_file_write.c \
	fx_file_write_notify_set.c \
	fx_media_abort.c \
	fx_media_boot_info_extract.c \
	fx_media_cache_invalidate.c \
	fx_media_check.c \
	fx_media_check_FAT_chain_check.c \
	fx_media_check_lost_cluster_check.c \
	fx_media_close.c \
	fx_media_close_notify_set.c \
	fx_media_extended_space_available.c \
	fx_media_flush.c \
	fx_media_format.c \
	fx_media_format_oem_name_set.c \
	fx_media_format_type_set.c \
	fx_media_format_volume_id_set.c \
	fx_media_open.c \
	fx_media_open_notify_set.c \
	fx_media_read.c \
	fx_media_space_available.c \
	fx_media_volume_get.c \
	fx_media_volume_get_extended.c \
	fx_media_volume_set.c \
	fx_media_write.c \
	fx_partition_offset_calculate.c \
	fx_ram_driver.c \
	fx_system_date_get.c \
	fx_system_date_set.c \
	fx_system_initialize.c \
	fx_system_time_get.c \
	fx_system_timer_entry.c \
	fx_system_time_set.c \
	fx_trace_event_insert.c \
	fx_trace_event_update.c \
	fx_trace_object_register.c \
	fx_trace_object_unregister.c \
	fx_unicode_directory_create.c \
	fx_unicode_directory_entry_change.c \
	fx_unicode_directory_entry_read.c \
	fx_unicode_directory_rename.c \
	fx_unicode_directory_search.c \
	fx_unicode_file_create.c \
	fx_unicode_file_rename.c \
	fx_unicode_length_get.c \
	fx_unicode_length_get_extended.c \
	fx_unicode_name_get.c \
	fx_unicode_name_get_extended.c \
	fx_unicode_short_name_get.c \
	fx_unicode_short_name_get_extended.c \
	fx_utility_16_unsigned_read.c \
	fx_utility_16_unsigned_write.c \
	fx_utility_32_unsigned_read.c \
	fx_utility_32_unsigned_write.c \
	fx_utility_64_unsigned_read.c \
	fx_utility_64_unsigned_write.c \
	fx_utility_absolute_path_get.c \
	fx_utility_FAT_entry_read.c \
	fx_utility_FAT_entry_write.c \
	fx_utility_FAT_flush.c \
	fx_utility_FAT_map_flush.c \
	fx_utility_FAT_sector_get.c \
	fx_utility_logical_sector_cache_entry_read.c \
	fx_utility_logical_sector_flush.c \
	fx_utility_logical_sector_read.c \
	fx_utility_logical_sector_write.c \
	fx_utility_memory_copy.c \
	fx_utility_memory_set.c \
	fx_utility_string_length_get.c \
	fx_utility_token_length_get.c

# <<< MODIFIED: Create object lists and combine them.
LIBC_HOOK_OBJS   := $(patsubst %.c,%.o,$(LIBC_HOOK_SRCS_C))
THREADX_OBJS     := $(patsubst %.c,%.o,$(THREADX_SRCS_C))
FILEX_OBJS       := $(patsubst %.c,%.o,$(FILEX_SRCS_C))
OBJS = $(PROJECT_OBJS) $(LIBC_HOOK_OBJS) $(THREADX_OBJS) $(FILEX_OBJS)

# <<< MODIFIED: Add VPATH for all new source directories.
VPATH += $(FIDO2_ROOT)/daric_sdk/middleware/libs/libc/newlib
VPATH += $(FIDO2_ROOT)/daric_sdk/middleware/threadx
VPATH += $(FIDO2_ROOT)/daric_sdk/middleware/threadx/util
VPATH += $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/common/src
VPATH += $(FIDO2_ROOT)/daric_sdk/middleware/threadx/filex/filex-6.4.1/common/src
VPATH += $(FIDO2_ROOT)/daric_sdk/middleware/threadx/filex/ports/daric


# <<< MODIFIED: Change FPU config to match fido2 build
COMMONFLAGS += -mfloat-abi=soft -DARMCM7
# COMMONFLAGS += -mfloat-abi=hard -mfpu=fpv5-d16 -DARMCM7_DP
# COMMONFLAGS += -mfloat-abi=hard -mfpu=fpv5-sp-d16 -DARMCM7_SP
COMMONFLAGS += -time -O3 -g -fno-exceptions

#### END OF COMMONFLAGS

# <<< MODIFIED: Add all definitions extracted from the fido2 build log
FIDO2_CONFIG_DEFS := \
	-DCONFIG_ARCH=arm \
	-DCONFIG_ARM=y \
	-DCONFIG_AW9523B=y \
	-DCONFIG_BATT_CHG=y \
	-DCONFIG_BOARD=Daric-EVB \
	-DCONFIG_BOARD_EVB=y \
	-DCONFIG_CPU_CORTEX_M7=y \
	-DCONFIG_DISABLE_CACHE=y \
	-DCONFIG_FINGERPRINT_IRQ_PIN=GPIO_PIN_1 \
	-DCONFIG_FINGERPRINT_IRQ_PORT=GPIOD \
	-DCONFIG_FINGERPRINT_MFM017RD1_UART=y \
	-DCONFIG_FINGERPRINT_POWER_METHOD=1 \
	-DCONFIG_FINGERPRINT_POWER_PIN=GPIO_PIN_9 \
	-DCONFIG_FINGERPRINT_POWER_PORT=GPIOB \
	-DCONFIG_FINGERPRINT_UART_ID=1 \
	-DCONFIG_FINGERPRINT_UART_RX_PIN=GPIO_PIN_13 \
	-DCONFIG_FINGERPRINT_UART_RX_PORT=GPIOD \
	-DCONFIG_FINGERPRINT_UART_TX_PIN=GPIO_PIN_14 \
	-DCONFIG_FINGERPRINT_UART_TX_PORT=GPIOD \
	-DCONFIG_GSENSOR=y \
	-DCONFIG_LCD_BUSY_IS_REQUIRED=false \
	-DCONFIG_LCD_BUSY_PIN=GPIO_PIN_13 \
	-DCONFIG_LCD_BUSY_PORT=GPIOC \
	-DCONFIG_LCD_COMMON=y \
	-DCONFIG_LCD_DC_IS_REQUIRED=true \
	-DCONFIG_LCD_DC_PIN=GPIO_PIN_9 \
	-DCONFIG_LCD_DC_PORT=GPIOD \
	-DCONFIG_LCD_RST_IS_REQUIRED=false \
	-DCONFIG_LCD_RST_PIN=GPIO_PIN_12 \
	-DCONFIG_LCD_RST_PORT=GPIOD \
	-DCONFIG_LCD_SPI_BAUDRATE=20000000 \
	-DCONFIG_LCD_SPI_ID=1 \
	-DCONFIG_LCD_SSD1685_SPI=y \
	-DCONFIG_LCD_VCIEN_IS_REQUIRED=false \
	-DCONFIG_LCD_VCIEN_PIN=GPIO_PIN_11 \
	-DCONFIG_LCD_VCIEN_PORT=GPIOD \
	-DCONFIG_MAIN_TASK_STACK_SIZE=10240 \
	-DCONFIG_MBEDTLS_LIB=y \
	-DCONFIG_NEWLIB_LIBC=y \
	-DCONFIG_NEWLIB_LIBC_NANO=y \
	-DCONFIG_PMIC=y \
	-DCONFIG_PMIC_AXP223=y \
	-DCONFIG_PMIC_I2C_ID=DARIC_I2C1_ID \
	-DCONFIG_PMIC_I2C_SPEED=100 \
	-DCONFIG_RGB_LED=y \
	-DCONFIG_RGB_LED_CHIP=1 \
	-DCONFIG_RGB_LED_I2C_ID=DARIC_I2C0_ID \
	-DCONFIG_RGB_LED_I2C_SPEED=100 \
	-DCONFIG_RTC_I2C_ID=DARIC_I2C0_ID \
	-DCONFIG_RTC_I2C_SPEED=100 \
	-DCONFIG_RTC_INS5T8900=y \
	-DCONFIG_SOC="Daric MPW A" \
	-DCONFIG_SOC_SERIES=daric \
	-DCONFIG_SYS_CLOCK_HW_CYCLES_PER_SEC=250000000 \
	-DCONFIG_SYS_CLOCK_TICKS_PER_SEC=1000 \
	-DCONFIG_TOUCH=y \
	-DCONFIG_TOUCH_IIC_ID=DARIC_I2C0_ID \
	-DCONFIG_TOUCH_IIC_SPEED=400 \
	-DCONFIG_TOUCH_INT_PIN=GPIO_PIN_2 \
	-DCONFIG_TOUCH_INT_PORT=GPIOB \
	-DCONFIG_TOUCH_RST_PIN=GPIO_PIN_1 \
	-DCONFIG_TOUCH_RST_PORT=GPIOB \
	-DCONFIG_USB_CDC=y \
	-DCONFIG_USB_DETEC=y \
	-DCONFIG_USB_HID=y \
	-DTX_INCLUDE_USER_DEFINE_FILE

# <<< MODIFIED: Add all include paths from the fido2 build log
FIDO2_CONFIG_INCLUDES := \
	-I$(FIDO2_ROOT)/daric_sdk/drivers/Inc \
	-I$(FIDO2_ROOT)/daric_sdk/middleware/cmsis/CMSIS/Core/Include \
	-I$(FIDO2_ROOT)/daric_sdk/devices/Include \
	-I$(FIDO2_ROOT)/project/fido2/user/inc/config \
	-I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports/cortex_m7/gnu/inc \
	-I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/include \
	-I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/common/inc \
	-I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/filex/filex-6.4.1/common/inc \
	-I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/filex/filex-6.4.1/ports/cortex_m7/gnu/inc


# C++ specific flags
CXXFLAGS += -fno-strict-aliasing -fstack-usage
CXXFLAGS += -std=c++17
CXXFLAGS += -fno-rtti

# <<< MODIFIED: Add FIDO2 configs to all compiler flags
ASFLAGS  += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(COMMONFLAGS) $(COMMONDEFS)
CFLAGS   += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(FIDO2_CONFIG_DEFS) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS)
CXXFLAGS += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(FIDO2_CONFIG_DEFS) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS)

######## Linker

# <<< MODIFIED: Add malloc undefines and nano.specs
MALLOC_UNDEFINES := -Wl,-u,malloc,-u,calloc,-u,free,-u,realloc,-u,_malloc_r,-u,_calloc_r,-u,_free_r,-u,_realloc_r,-u,aligned_alloc

LDFLAGS += $(COMMONFLAGS) -lm
LDFLAGS += -lstdc++
LDFLAGS += -Wl,--gc-sections,$(addprefix -T,$(LD_FILES))
LDFLAGS += --debug
LDFLAGS += $(MALLOC_UNDEFINES)
LDFLAGS += -Wl,--specs=nano.specs

######### Rules

.PHONY: all clean r disassemble show

.NOPARALLEL: clean

all: $(TARGET).bin

show:
	@echo CPP: $(CXX) $(CXXFLAGS) $(CPPFLAGS)
	@echo CC: $(CC) $(CFLAGS)
	@echo LD: $(LD) $(LDFLAGS)
	@echo LIBS: $(LIBS)
	@echo OBJS: $(OBJS)

%.o: %.S
	@echo $(AS) $<
	@$(AS) -c -o $@ $<

# <<< MODIFIED: Add specific rule for libc-hooks.c
libc-hooks.o: libc-hooks.c
	@echo "CC (custom rule) $<"
	$(CC) -c -o $@ $(CFLAGS) -fno-builtin $<

# Explicit rule for C files (if you have mixed C/C++)
%.o: %.c
	$(CC) -c -o $@ $(CFLAGS) $<

# C++ compilation rule
%.o: %.cpp
	$(CXX) -c -o $@ $(CPPFLAGS) $(CXXFLAGS) $<
%.o: %.cc
	$(CXX) -c -o $@ $(CPPFLAGS) $(CXXFLAGS) $<
%.o: %.cxx
	$(CXX) -c -o $@ $(CPPFLAGS) $(CXXFLAGS) $<

# <<< MODIFIED: Linking rule to handle all objects
$(TARGET).elf: $(TARGET).o $(OBJS)
	$(LD) -o $@ $^ $(LIBS) $(LDFLAGS)

%.bin: %.elf
	$(OBJCOPY) $(CPFLAGS) $^ $@
	$(OBJDUMP) $(ODFLAGS) $^ > $(@:.bin=.dmp)

%.dis : %.elf
	$(OBJDUMP) -d $< > $@

disassemble: $(TARGETS:.bin=.dis)

clean:
	rm -f *.o $(TARGET).bin $(TARGET).elf $(TARGET).dmp $(TARGET).map

r: clean | all