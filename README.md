Proof of concept
================

An attempt to build softdevice s110 using rust.

Software used
* rustc 1.7.0-nightly (bfb4212ee 2016-01-01)
* arm-none-eabi-gcc (GNU Tools for ARM Embedded Processors) 5.2.1 20151202 (release) [ARM/embedded-5-branch revision 231848]
* SEGGER J-Link Commander V5.11a (Compiled Dec 21 2015 18:23:42)
* OSX 10.11.2 (BuildVersion: 15C50)

## Building the sample
1. Make sure that arm-none-eabi- tools are on the PATH
2. make all
3. make flash-softdevice
4. make flash
5. make gdbserver
6. make debug

