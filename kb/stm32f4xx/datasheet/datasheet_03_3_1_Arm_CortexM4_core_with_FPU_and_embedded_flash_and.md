## 3 Functional overview

Figure 5. STM32F40xxx block diagram

1. The camera interface and ethernet are available only on STM32F407xx devices.

20/206
DS8626 Rev 12

MS19920V5

--- Page 21 ---

STM32F405xx, STM32F407xx
Functional overview

### 3.1 Arm® Cortex®-M4 core with FPU and embedded flash and SRAM

The Arm Cortex-M4 processor with FPU is the latest generation of Arm processors for embedded systems. It was developed to provide a low-cost platform that meets the needs of MCU implementation, with a reduced pin count and low-power consumption, while delivering outstanding computational performance and an advanced response to interrupts.

The Arm Cortex-M4 32-bit RISC processor with FPU features exceptional code-efficiency, delivering the high-performance expected from an Arm core in the memory size usually associated with 8- and 16-bit devices.

The processor supports a set of DSP instructions which allow efficient signal processing and complex algorithm execution.

Its single precision FPU (floating point unit) speeds up software development by using metalanguage development tools, while avoiding saturation.

The STM32F405xx and STM32F407xx family is compatible with all Arm tools and software.

*Figure 5 shows the general block diagram of the STM32F40xxx family.*

**Note:** Cortex-M4 with FPU is binary compatible with Cortex-M3.

### 3.2 Adaptive real-time memory accelerator (ART Accelerator)

The ART Accelerator is a memory accelerator which is optimized for STM32 industry-standard Arm® Cortex®-M4 with FPU processors. It balances the inherent performance advantage of the Arm Cortex-M4 with FPU over flash memory technologies, which normally requires the processor to wait for the flash memory at higher frequencies.

To release the processor full 210 DMIPS performance at this frequency, the accelerator implements an instruction prefetch queue and branch cache, which increases program execution speed from the 128-bit flash memory. Based on CoreMark benchmark, the performance achieved thanks to the ART accelerator is equivalent to 0 wait state program execution from flash memory at a CPU frequency up to 168 MHz.

### 3.3 Memory protection unit

The memory protection unit (MPU) is used to manage the CPU accesses to memory to prevent one task to accidentally corrupt the memory or resources used by any other active task. This memory area is organized into up to 8 protected areas that can in turn be divided up into 8 subareas. The protection area sizes are between 32 bytes and the whole 4 gigabytes of addressable memory.

The MPU is especially helpful for applications where some critical or certified code has to be protected against the misbehavior of other tasks. It is usually managed by an RTOS (real-time operating system). If a program accesses a memory location that is prohibited by the MPU, the RTOS can detect it and take action. In an RTOS environment, the kernel can dynamically update the MPU area setting, based on the process to be executed.

The MPU is optional and can be bypassed for applications that do not need it.

ST
DS8626 Rev 12
21/206

--- Page 22 ---

Functional overview
STM32F405xx, STM32F407xx

### 3.4 Embedded flash memory

The STM32F40xxx devices embed a flash memory of 512 Kbytes or 1 Mbytes available for storing programs and data, plus 512 bytes of OTP memory.

### 3.5 CRC (cyclic redundancy check) calculation unit

The CRC (cyclic redundancy check) calculation unit is used to get a CRC code from a 32-bit data word and a fixed generator polynomial.

Among other applications, CRC-based techniques are used to verify data transmission or storage integrity. In the scope of the EN/IEC 60335-1 standard, they offer a means of verifying the flash memory integrity. The CRC calculation unit helps compute a software signature during runtime, to be compared with a reference signature generated at link-time and stored at a given memory location.

### 3.6 Embedded SRAM

All STM32F40xxx products embed:

*   Up to 192 Kbytes of system SRAM including 64 Kbytes of CCM (core coupled memory) data RAM
    RAM memory is accessed (read/write) at CPU clock speed with 0 wait states.
*   4 Kbytes of backup SRAM
    This area is accessible only from the CPU. Its content is protected against possible unwanted write accesses, and is retained in Standby or VBAT mode.

### 3.7 Multi-AHB bus matrix

The 32-bit multi-AHB bus matrix interconnects all the masters (CPU, DMAs, Ethernet, USB HS) and the slaves (flash memory, RAM, FSMC, AHB, and APB peripherals) and ensures a seamless and efficient operation even when several high-speed peripherals work simultaneously.

22/206
DS8626 Rev 12
ST

--- Page 23 ---

STM32F405xx, STM32F407xx
Functional overview

Figure 6. Multi-AHB matrix

64-Kbyte CCM data RAM
ARM Cortex-M4
GP DMA1
GP DMA2
MAC Ethernet
USB OTG HS
I-bus
D-bus
S-bus
DMA_P1
DMA_MEM1
DMA_MEM2
DMA_P2
ETHERNET_M
USB_HS_M
Bus matrix-S
ICODE
DCODE
ACCEL
Flash memory
SRAM1 112 Kbyte
SRAM2 16 Kbyte
AHB1 peripherals
AHB2 peripherals
FSMC Static MemCtl
APB1
APB2
ai18490e

### 3.8 DMA controller (DMA)

The devices feature two general-purpose dual-port DMAs (DMA1 and DMA2) with 8 streams each. They are able to manage memory-to-memory, peripheral-to-memory and memory-to-peripheral transfers. They feature dedicated FIFOs for APB/AHB peripherals, support burst transfer and are designed to provide the maximum peripheral bandwidth (AHB/APB).

The two DMA controllers support circular buffer management, so that no specific code is needed when the controller reaches the end of the buffer. The two DMA controllers also have a double buffering feature, which automates the use and switching of two memory buffers without requiring any special code.

Each stream is connected to dedicated hardware DMA requests, with support for software trigger on each stream. Configuration is made by software and transfer sizes between source and destination are independent.

ST
DS8626 Rev 12
23/206

--- Page 24 ---

Functional overview
STM32F405xx, STM32F407xx

The DMA can be used with the main peripherals:
* SPI and I²S
* I²C
* USART
* General-purpose, basic and advanced-control timers TIMx
* DAC
* SDIO
* Camera interface (DCMI)
* ADC.

### 3.9 Flexible static memory controller (FSMC)

The FSMC is embedded in the STM32F405xx and STM32F407xx family. It has four Chip Select outputs supporting the following modes: PCCard/CompactFlash, SRAM, PSRAM, NOR flash and NAND flash.

Functionality overview:
* Write FIFO
* Maximum FSMC_CLK frequency for synchronous accesses is 60 MHz.

#### 3.9.1 LCD parallel interface

The FSMC can be configured to interface seamlessly with most graphic LCD controllers. It supports the Intel 8080 and Motorola 6800 modes, and is flexible enough to adapt to specific LCD interfaces. This LCD parallel interface capability makes it easy to build cost-effective graphic applications using LCD modules with embedded controllers or high performance solutions using external controllers with dedicated acceleration.

### 3.10 Nested vectored interrupt controller (NVIC)

The STM32F405xx and STM32F407xx embed a nested vectored interrupt controller able to manage 16 priority levels, and handle up to 82 maskable interrupt channels plus the 16 interrupt lines of the Cortex®-M4 with FPU core.

* Closely coupled NVIC gives low-latency interrupt processing
* Interrupt entry vector table address passed directly to the core
* Allows early processing of interrupts
* Processing of late arriving, higher-priority interrupts
* Support tail chaining
* Processor state automatically saved
* Interrupt entry restored on interrupt exit with no instruction overhead

This hardware block provides flexible interrupt management features with minimum interrupt latency.

24/206
DS8626 Rev 12

--- Page 25 ---

STM32F405xx, STM32F407xx
Functional overview

### 3.11 External interrupt/event controller (EXTI)

The external interrupt/event controller consists of 23 edge-detector lines used to generate interrupt/event requests. Each line can be independently configured to select the trigger event (rising edge, falling edge, both) and can be masked independently. A pending register maintains the status of the interrupt requests. The EXTI can detect an external line with a pulse width shorter than the Internal APB2 clock period. Up to 140 GPIOs can be connected to the 16 external interrupt lines.

### 3.12 Clocks and startup

On reset the 16 MHz internal RC oscillator is selected as the default CPU clock. The 16 MHz internal RC oscillator is factory-trimmed to offer 1% accuracy over the full temperature range. The application can then select as system clock either the RC oscillator or an external 4-26 MHz clock source. This clock can be monitored for failure. If a failure is detected, the system automatically switches back to the internal RC oscillator and a software interrupt is generated (if enabled). This clock source is input to a PLL thus allowing to increase the frequency up to 168 MHz. Similarly, full interrupt management of the PLL clock entry is available when necessary (for example if an indirectly used external oscillator fails).

Several prescalers allow the configuration of the three AHB buses, the high-speed APB (APB2) and the low-speed APB (APB1) domains. The maximum frequency of the three AHB buses is 168 MHz while the maximum frequency of the high-speed APB domains is 84 MHz. The maximum allowed frequency of the low-speed APB domain is 42 MHz.

The devices embed a dedicated PLL (PLL2S) which allows to achieve audio class performance. In this case, the I²S master clock can generate all standard sampling frequencies from 8 kHz to 192 kHz.

### 3.13 Boot modes

At startup, boot pins are used to select one out of three boot options:

*   Boot from user flash
*   Boot from system memory
*   Boot from embedded SRAM

The boot loader is located in system memory. It is used to reprogram the flash memory by using USART1 (PA9/PA10), USART3 (PC10/PC11 or PB10/PB11), CAN2 (PB5/PB13), USB OTG FS in Device mode (PA11/PA12) through DFU (device firmware upgrade).

### 3.14 Power supply schemes

*   VDD = 1.8 to 3.6 V: external power supply for I/Os and the internal regulator (when enabled), provided externally through VDD pins.
*   VSSA, VDDA = 1.8 to 3.6 V: external analog power supplies for ADC, DAC, Reset blocks, RCs and PLL. VDDA and VSSA must be connected to VDD and VSS, respectively.
*   VBAT = 1.65 to 3.6 V: power supply for RTC, external clock 32 kHz oscillator and backup registers (through power switch) when VDD is not present.

ST
DS8626 Rev 12
25/206

--- Page 26 ---

Functional overview
STM32F405xx, STM32F407xx

Refer to Figure 21: Power supply scheme for more details.

Note:
$V_{DD}/V_{DDA}$ minimum value of 1.7 V is obtained when the device operates in reduced temperature range, and with the use of an external power supply supervisor (refer to Section 3.15.2: Internal reset OFF).
Refer to Table 2 in order to identify the packages supporting this option.

