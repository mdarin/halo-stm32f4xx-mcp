# 36 Flexible static memory controller (FSMC)

This section applies to the whole STM32F40x/41x family only.

## 36.1 FSMC main features

The FSMC block is able to interface with synchronous and asynchronous memories and 16-bit PC memory cards. Its main purpose is to:

* Translate the AHB transactions into the appropriate external device protocol
* Meet the access timing requirements of the external devices

All external memories share the addresses, data and control signals with the controller. Each external device is accessed by means of a unique chip select. The FSMC performs only one access at a time to an external device.

The FSMC has the following main features:

* Interfaces with static memory-mapped devices including:
    * Static random access memory (SRAM)
    * NOR Flash memory/OneNAND Flash memory
    * PSRAM (4 memory banks)
* Two banks of NAND Flash with ECC hardware that checks up to 8 Kbytes of data
* 16-bit PC Card compatible devices
* Supports burst mode access to synchronous devices (NOR Flash and PSRAM)
* 8- or 16-bit wide databus
* Independent chip select control for each memory bank
* Independent configuration for each memory bank
* Programmable timings to support a wide range of devices, in particular:
    * Programmable wait states (up to 15)
    * Programmable bus turnaround cycles (up to 15)
    * Programmable output enable and write enable delays (up to 15)
    * Independent read and write timings and protocol, so as to support the widest variety of memories and timings
* Write enable and byte lane select outputs for use with PSRAM and SRAM devices
* Translation of 32-bit wide AHB transactions into consecutive 16-bit or 8-bit accesses to external 16-bit or 8-bit devices
* A Write FIFO, 2-word long (16-word long for STM32F42x and STM32F43x), each word is 32 bits wide, only stores data and not the address. Therefore, this FIFO only buffers AHB write burst transactions. This makes it possible to write to slow memories and free the AHB quickly for other operations. Only one burst at a time is buffered: if a new AHB burst or single transaction occurs while an operation is in progress, the FIFO is drained. The FSMC will insert wait states until the current memory access is complete.
* External asynchronous wait control

The FSMC registers that define the external device type and associated characteristics are usually set at boot time and do not change until the next reset or power-up. However, it is possible to change the settings at any time.

1544/1749
RM0090 Rev 18

--- Page 180 ---

RM0090
Flexible static memory controller (FSMC)

### 36.2 Block diagram

The FSMC consists of four main blocks:

*   The AHB interface (including the FSMC configuration registers)
*   The NOR Flash/PSRAM controller
*   The NAND Flash/PC Card controller
*   The external device interface

The block diagram is shown in Figure 434.

**Figure 434. FSMC block diagram**

FSMC interrupt to NVIC
From clock controller
HCLK
AHB bus
Configuration registers
NOR/PSRAM memory controller
NAND/PC Card memory controller
FSMC_NE[4:1]
FSMC_NL(or NADV)
FSMC_NBL[1:0]
FSMC_CLK
NOR/PSRAM signals
FSMC_A[25:0]
FSMC_D[15:0]
FSMC_NOE
FSMC_NWE
FSMC_NWAIT
Shared signals
FSMC_NCE[3:2]
FSMC_INT[3:2]
NAND signals
FSMC_INTR
FSMC_NCE4_1
FSMC_NCE4_2
FSMC_NIORD
FSMC_NIOWR
PC Card signals
FSMC_NREG
FSMC_CD
ai15591b

