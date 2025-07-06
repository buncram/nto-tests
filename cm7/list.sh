 arm-none-eabi-objdump -h mbox.elf > mbox.lst
 arm-none-eabi-nm -r --size-sort --print-size mbox.elf >> mbox.lst
 arm-none-eabi-objdump -r -S -d mbox.elf >> mbox.lst