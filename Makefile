PROJ=blumenplace
MODE=debug


TOOLCHAIN=arm-none-eabi
CC=${TOOLCHAIN}-gcc
AR=${TOOLCHAIN}-ar
LD=${TOOLCHAIN}-gcc
OBJCOPY=${TOOLCHAIN}-objcopy
OBJDUMP=${TOOLCHAIN}-objdump
SIZE=${TOOLCHAIN}-size
GDB=${TOOLCHAIN}-gdb


JLINKOPT=-device nrf51822_xxaa -if swd -speed 4000
JLINK=JLinkExe ${JLINKOPT}
JLINKGDBSERVER=JLinkGDBServer ${JLINKOPT}
GDB_PORT_NUMBER=2331


SOFTDEVICE=./third_party/nrf51_sdk/softdevice/s110/hex/s110_nrf51_8.0.0_softdevice.hex
SOFTDEVICE_OUTPUT=./target/$(notdir $(SOFTDEVICE))


RUST_TARGET=thumbv6m-none-eabi
OUT_DIR=./target
OUT=${OUT_DIR}/${PROJ}
OUT_FILE=${OUT_DIR}/${RUST_TARGET}/${MODE}/${PROJ}
FLASH_START_ADDRESS = $(shell $(OBJDUMP) -h $(OUT_FILE) -j .text | grep .text | awk '{print $$4}')


all: ${OUT}.bin ${OUT}.S
	# srec_cat ${SOFTDEVICE} -intel ${OUT}.bin -binary -offset 0x18000 -o ${OUT}.hex -intel
	${SIZE} -d ${OUT_FILE}

${OUT_FILE}: src/main.rs
	cargo rustc --target ${RUST_TARGET} -- -g -Z no-landing-pads -C no-redzone=yes --emit=dep-info,link -C ar=${AR} -C linker=${LD}

${OUT}.bin: ${OUT_FILE}
	${OBJCOPY} -O binary ${OUT_FILE} ${OUT}.bin

${OUT}.S: ${OUT_FILE}
	${OBJDUMP} -d ${OUT_FILE} > ${OUT}.S

flash: ${OUT}.bin
	printf "r\nloadbin ${OUT}.bin $(FLASH_START_ADDRESS)\nr\ng\nexit\n" | ${JLINK}

reset:
	printf "r\ng\nexit\n" | ${JLINK}

pin-reset:
	printf "w4 40000544 1\nr\nexit\n" | ${JLINK}

erase-all:
	# Write to NVMC to enable erase,
	# do erase all,
	# wait for completion.
	# reset
	printf "w4 4001e504 2\nw4 4001e50c 1\nsleep 100\nr\nexit\n" | ${JLINK}

recover-cmd:
	printf "si 0\nt0\nsleep 1\ntck1\nsleep 1\nt1\nsleep 2\nt0\nsleep 2\nt1\nsleep 2\nt0\nsleep 2\nt1\nsleep 2\nt0\nsleep 2\nt1\nsleep 2\nt0\nsleep 2\nt1\nsleep 2\nt0\nsleep 2\nt1\nsleep 2\nt0\nsleep 2\nt1\nsleep 2\ntck0\nsleep 100\nsi 1\nr\nexit\n" | ${JLINK}

recover: recover-cmd erase-all pin-reset
	echo "Recover finished"

flash-softdevice: erase-all
	# Assumes device is erased.
	${OBJCOPY} -Iihex -Obinary $(SOFTDEVICE) $(SOFTDEVICE_OUTPUT:.hex=.bin)
	# Write to NVMC to enable write.
	# Write mainpart, write UICR.
	printf "w4 4001e504 1\nloadbin \"$(SOFTDEVICE_OUTPUT:.hex=.bin)\" 0\nr\ng\nexit\n" | ${JLINK}

debug:
	cgdb -d ${GDB} -- ${OUT_FILE} -ex "target extended-remote :${GDB_PORT_NUMBER}"

gdbserver:
	# openocd --file ./openocd.cfg
	${JLINKGDBSERVER} -port ${GDB_PORT_NUMBER}

clean-bin:
	rm -Rf ${OUT}.S
	rm -Rf ${OUT}.bin
	rm -Rf ${OUT_FILE}

clean:
	rm -Rf ${OUT_DIR}

.PHONY: flash flash-softdevice erase-all debug

