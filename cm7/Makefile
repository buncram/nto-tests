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

# =========================================================================
# <<< FINALIZED: Source Code Configuration >>>
# =========================================================================

FIDO2_ROOT := ../../threadx_mono

# --- Define paths to the required source code directories ---
THREADX_COMMON_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/common/src
THREADX_PORT_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports_arch/ARMv7-M/threadx/gnu/src
THREADX_PORT_EXAMPLE_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports_arch/ARMv7-M/threadx/gnu/example_build
THREADX_PORT_INCLUDE_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports_arch/ARMv7-M/threadx/inc
LIBC_HOOK_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/libs/libc/newlib
APP_SETUP_PATH := .

# --- Automatically find all source files in those directories ---
THREADX_COMMON_SRCS_C := $(wildcard $(THREADX_COMMON_PATH)/*.c)
THREADX_PORT_SRCS_C   := $(wildcard $(THREADX_PORT_PATH)/*.c)
THREADX_PORT_SRCS_S   := $(wildcard $(THREADX_PORT_PATH)/*.S)
THREADX_INIT_SRC_S    := $(wildcard $(THREADX_PORT_EXAMPLE_PATH)/tx_initialize_low_level.S)

# --- Create final source lists ---
PROJECT_SRCS = mbox.c daric_util.c reram.c threadx_app_setup.c
LIBC_HOOK_SRCS = libc-hooks.c

C_SRCS := \
    $(PROJECT_SRCS) \
    $(APP_SRCS) \
    $(LIBC_HOOK_SRCS) \
    $(notdir $(THREADX_COMMON_SRCS_C)) \
    $(notdir $(THREADX_PORT_SRCS_C))

S_SRCS := \
    $(notdir $(THREADX_PORT_SRCS_S)) \
    $(notdir $(THREADX_INIT_SRC_S))

# --- Create the final object lists from the source lists ---
OBJS_C := $(patsubst %.c,%.o,$(C_SRCS))
OBJS_S := $(patsubst %.S,%.o,$(S_SRCS))
OBJS   := $(OBJS_C) $(OBJS_S)

# --- Add all source directories to VPATH so 'make' can find the files ---
VPATH := $(APP_SETUP_PATH) \
         $(LIBC_HOOK_PATH) \
         $(THREADX_COMMON_PATH) \
         $(THREADX_PORT_PATH) \
         $(THREADX_PORT_EXAMPLE_PATH)

# =========================================================================

# --- Finalized Compiler Flags ---
# Use soft-float for libc compatibility, but tell the toolchain the FPU exists for assembly/context-switching.
COMMONFLAGS += -mfloat-abi=soft -mfpu=fpv5-d16 -DARMCM7
COMMONFLAGS += -time -O3 -g -fno-exceptions

CXXFLAGS += -fno-strict-aliasing -fstack-usage -std=c++17 -fno-rtti

FIDO2_CONFIG_DEFS := \
    -DCONFIG_ARCH=arm -DCONFIG_ARM=y -DCONFIG_AW9523B=y -DCONFIG_BATT_CHG=y \
    -DCONFIG_BOARD=Daric-EVB -DCONFIG_BOARD_EVB=y -DCONFIG_CPU_CORTEX_M7=y \
    -DCONFIG_DISABLE_CACHE=y -DCONFIG_FINGERPRINT_IRQ_PIN=GPIO_PIN_1 \
    -DCONFIG_FINGERPRINT_IRQ_PORT=GPIOD -DCONFIG_FINGERPRINT_MFM017RD1_UART=y \
    -DCONFIG_FINGERPRINT_POWER_METHOD=1 -DCONFIG_FINGERPRINT_POWER_PIN=GPIO_PIN_9 \
    -DCONFIG_FINGERPRINT_POWER_PORT=GPIOB -DCONFIG_FINGERPRINT_UART_ID=1 \
    -DCONFIG_FINGERPRINT_UART_RX_PIN=GPIO_PIN_13 -DCONFIG_FINGERPRINT_UART_RX_PORT=GPIOD \
    -DCONFIG_FINGERPRINT_UART_TX_PIN=GPIO_PIN_14 -DCONFIG_FINGERPRINT_UART_TX_PORT=GPIOD \
    -DCONFIG_GSENSOR=y -DCONFIG_LCD_BUSY_IS_REQUIRED=false \
    -DCONFIG_LCD_BUSY_PIN=GPIO_PIN_13 -DCONFIG_LCD_BUSY_PORT=GPIOC -DCONFIG_LCD_COMMON=y \
    -DCONFIG_LCD_DC_IS_REQUIRED=true -DCONFIG_LCD_DC_PIN=GPIO_PIN_9 \
    -DCONFIG_LCD_DC_PORT=GPIOD -DCONFIG_LCD_RST_IS_REQUIRED=false \
    -DCONFIG_LCD_RST_PIN=GPIO_PIN_12 -DCONFIG_LCD_RST_PORT=GPIOD -DCONFIG_LCD_SSD1685_SPI=y \
    -DCONFIG_LCD_VCIEN_IS_REQUIRED=false -DCONFIG_LCD_VCIEN_PIN=GPIO_PIN_11 \
    -DCONFIG_LCD_VCIEN_PORT=GPIOD -DCONFIG_MAIN_TASK_STACK_SIZE=10240 \
    -DCONFIG_MBEDTLS_LIB=y -DCONFIG_NEWLIB_LIBC=y -DCONFIG_NEWLIB_LIBC_NANO=y \
    -DCONFIG_PMIC=y -DCONFIG_PMIC_AXP223=y -DCONFIG_PMIC_I2C_ID=DARIC_I2C1_ID \
    -DCONFIG_PMIC_I2C_SPEED=100 -DCONFIG_RGB_LED=y -DCONFIG_RGB_LED_CHIP=1 \
    -DCONFIG_RGB_LED_I2C_ID=DARIC_I2C0_ID -DCONFIG_RGB_LED_I2C_SPEED=100 \
    -DCONFIG_RTC_I2C_ID=DARIC_I2C0_ID -DCONFIG_RTC_I2C_SPEED=100 \
    -DCONFIG_RTC_INS5T8900=y -DCONFIG_SOC="Daric MPW A" -DCONFIG_SOC_SERIES=daric \
    -DCONFIG_SYS_CLOCK_HW_CYCLES_PER_SEC=250000000 -DCONFIG_SYS_CLOCK_TICKS_PER_SEC=1000 \
    -DCONFIG_TOUCH=y -DCONFIG_TOUCH_IIC_ID=DARIC_I2C0_ID -DCONFIG_TOUCH_IIC_SPEED=400 \
    -DCONFIG_TOUCH_INT_PIN=GPIO_PIN_2 -DCONFIG_TOUCH_INT_PORT=GPIOB \
    -DCONFIG_TOUCH_RST_PIN=GPIO_PIN_1 -DCONFIG_TOUCH_RST_PORT=GPIOB -DCONFIG_USB_CDC=y \
    -DCONFIG_USB_DETEC=y -DCONFIG_USB_HID=y -DTX_INCLUDE_USER_DEFINE_FILE

FIDO2_CONFIG_INCLUDES := \
    -I$(FIDO2_ROOT)/daric_sdk/drivers/Inc \
    -I$(FIDO2_ROOT)/daric_sdk/middleware/cmsis/CMSIS/Core/Include \
    -I$(FIDO2_ROOT)/daric_sdk/devices/Include \
    -I$(FIDO2_ROOT)/project/fido2/user/inc/config \
	 -I$(FIDO2_ROOT)/daric_sdk/devices/Include \
    -I$(THREADX_PORT_PATH) \
    -I$(THREADX_PORT_INCLUDE_PATH) \
    -I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/include \
    -I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/common/inc

# Combine all flags. CFLAGS are used for both .c and .S files.
CFLAGS   += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(FIDO2_CONFIG_DEFS) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS)
CXXFLAGS += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(FIDO2_CONFIG_DEFS) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS)

# --- Finalized Linker Flags ---
MALLOC_UNDEFINES := -Wl,-u,malloc,-u,calloc,-u,free,-u,realloc,-u,_malloc_r,-u,_calloc_r,-u,_free_r,-u,_realloc_r,-u,aligned_alloc
LDFLAGS += $(COMMONFLAGS) -Wl,--gc-sections,$(addprefix -T,$(LD_FILES)) --debug $(MALLOC_UNDEFINES) --specs=nano.specs
LDLIBS   = -lm -lstdc++

######### Rules #########
.PHONY: all clean show

all: $(TARGET).bin

show:
	@echo "--- C Sources ---"
	@echo "$(C_SRCS)"
	@echo "--- Asm Sources ---"
	@echo "$(S_SRCS)"
	@echo "--- Objects ---"
	@echo "$(OBJS)"

# Rule for assembling .S files (using CC)
%.o: %.S
	@echo "CC (for assembly) $<"
	@$(CC) -c $(CFLAGS) -o $@ $<

# Rule for compiling libc-hooks.c with special flags
libc-hooks.o: libc-hooks.c
	@echo "CC (custom rule) $<"
	@$(CC) -c -o $@ $(CFLAGS) -fno-builtin $<

# Rule for compiling all other .c files
%.o: %.c
	@echo "CC $<"
	@$(CC) -c -o $@ $(CFLAGS) $<

# Rule for linking the final executable
$(TARGET).elf: $(OBJS)
	@echo "LD $@"
	@$(LD) -o $@ $^ $(LDFLAGS) $(LDLIBS)

# Rule for creating the .bin file
%.bin: %.elf
	@echo "OBJCOPY $@"
	@$(OBJCOPY) $(CPFLAGS) $^ $@
	@$(OBJDUMP) $(ODFLAGS) $^ > $(@:.bin=.dmp)

clean:
	@echo "Cleaning..."
	@rm -f $(OBJS) $(TARGET).bin $(TARGET).elf $(TARGET).dmp $(TARGET).mapcle