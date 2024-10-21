# vim: noet:ts=4:sw=4:

fullmf := $(abspath $(lastword $(MAKEFILE_LIST)))
pathmf := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

include makefile.daric.common

## TODO if project has cpp, use g++
LD        :=  $(TOOLCHAIN)gcc

#############

TARGET = mbox

LD_FILES ?= $(pathmf)/gcc_daric_mbox.ld

######## SYS obj ########

# OBJS += sha256.o

# remove all other
OBJS = daric_util.o reram.o

## FPU config, pick only ONE
COMMONFLAGS += -mfloat-abi=hard -mfpu=fpv5-d16 -DARMCM7_DP
#COMMONFLAGS += -mfloat-abi=hard -mfpu=fpv5-sp-d16 -DARMCM7_SP
#COMMONFLAGS += -mfloat-abi=soft -DARMCM7

COMMONFLAGS += -time -O3 -g -fno-exceptions

#### END OF COMMONFLAGS

# CFLAGS += -fno-strict-aliasing -fstack-usage
# CFLAGS += -DMSC_CLOCK -DITERATIONS=5
# CFLAGS += -DNOMSG

# CFLAGS += -nostdlib


# CXXFLAGS += -fno-strict-aliasing -fstack-usage

ASFLAGS  += $(INCLUDES) $(COMMONFLAGS) $(COMMONDEFS)
CFLAGS   += $(INCLUDES) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS)
CXXFLAGS += $(INCLUDES) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS)

######## Linker
## LD script must use absolute path

LDFLAGS += $(COMMONFLAGS) -lm
# for gcc as ld
LDFLAGS += -Wl,--gc-sections,$(addprefix -T,$(LD_FILES))
# for real ld
#LDFLAGS += $(addprefix -T,$(LD_FILES))

## map file , useless, use .dmp dump file
# ,-Map=$(TARGETS).map
# LDFLAGS +=  -Wa,-mimplicit-thumb
# LDFLAGS += -lgcc -lm
LDFLAGS += --debug

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

#%.o: %.c  using Implicit Rules

%.o: %.cpp
	$(CXX) -c -o $@ $(CPPFLAGS) $(CXXFLAGS) $<

$(TARGET).elf: $(TARGET).o $(OBJS) $(LIBS) 
	$(LD) -o $@ $(OBJS) $(LIBS) $< $(LDFLAGS)

%.bin: %.elf
	$(OBJCOPY) $(CPFLAGS) $^ $@
	$(OBJDUMP) $(ODFLAGS) $^ > $(@:.bin=.dmp)

%.dis : %.elf
	$(OBJDUMP) -d $< > $@

disassemble: $(TARGETS:.bin=.dis)

clean:
	rm -f *.o $(TARGET).bin $(TARGET).elf $(TARGET).dmp $(TARGET).map $(TARGET).o

r: clean | all

