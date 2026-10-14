# 37 Flexible memory controller (FMC)

The Flexible memory controller (FMC) includes three memory controllers:
* The NOR/PSRAM memory controller
* The NAND/PC Card memory controller
* The Synchronous DRAM (SDRAM/Mobile LPDDR SDRAM) controller

This section applies to STM32F42xxx and STM32F43xxx only.

## 37.1 FMC main features

The FMC functional block makes the interface with synchronous and asynchronous static memories, SDRAM memories, and 16-bit PC memory cards. Its main purposes are:
* to translate AHB transactions into the appropriate external device protocol
* to meet the access time requirements of the external memory devices

All external memories share the addresses, data and control signals with the controller. Each external device is accessed by means of a unique Chip Select. The FMC performs only one access at a time to an external device.

The main features of the FMC controller are the following:
* Interface with static-memory mapped devices including:
    * Static random access memory (SRAM)
    * NOR Flash memory/OneNAND Flash memory
    * PSRAM (4 memory banks)
    * 16-bit PC Card compatible devices
    * Two banks of NAND Flash memory with ECC hardware to check up to 8 Kbytes of data
* Interface with synchronous DRAM (SDRAM/Mobile LPDDR SDRAM) memories
* Burst mode support for faster access to synchronous devices such as NOR Flash memory, PSRAM and SDRAM
* Programmable continuous clock output for asynchronous and synchronous accesses
* 8-, 16- or 32-bit wide data bus
* Independent Chip Select control for each memory bank
* Independent configuration for each memory bank
* Write enable and byte lane select outputs for use with PSRAM, SRAM and SDRAM devices
* External asynchronous wait control
* Write Data FIFO with 16 x33-bit depth
* Write Address FIFO with 16x30-bit depth
* Cacheable Read FIFO with 6 x32-bit depth (6 x14-bit address tag) for SDRAM controller.

1602/1749
RM0090 Rev 18
ST

--- Page 43 ---

RM0090
Flexible memory controller (FMC)

The FMC embeds two Write FIFOs: a Write Data FIFO with a 16x33-bit depth and a Write Address FIFO with a 16x30-bit depth.

*   The Write Data FIFO stores the AHB data to be written to the memory (up to 32 bits) plus one bit for the AHB transfer (burst or not sequential mode)
*   The Write Address FIFO stores the AHB address (up to 28 bits) plus the AHB data size (up to 2 bits). When operating in burst mode, only the start address is stored except when crossing a page boundary (for PSRAM and SDRAM). In this case, the AHB burst is broken into two FIFO entries.

At startup the FMC pins must be configured by the user application. The FMC I/O pins which are not used by the application can be used for other purposes.

The FMC registers that define the external device type and associated characteristics are usually set at boot time and do not change until the next reset or power-up. However, the settings can be changed at any time.

