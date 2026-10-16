## 3 Embedded Flash memory interface

**3.1 Introduction**

The Flash memory interface manages CPU AHB I-Code and D-Code accesses to the Flash memory. It implements the erase and program Flash memory operations and the read and

write protection mechanisms.

The Flash memory interface accelerates code execution with a system of instruction

prefetch and cache lines.

**3.2 Main features**

• Flash memory read operations

• Flash memory program/erase operations

• Read / write protections

• Prefetch on I-Code

• 64 cache lines of 128 bits on I-Code

• 8 cache lines of 128 bits on D-Code Figure 3

shows the Flash memory interface connection inside the system architecture.

**Figure 3. Flash memory interface connection inside system architecture (STM32F405xx/07xx and STM32F415xx/17xx)**

AHB Flash

32-bit memory

Cortex-M4 with FPU

instruction Flash interface bus

I-Code bus I-Code

bus 128 bits

Flash memory AHB

Cortex D-Code

core

S bus

32-bit FLITF registers

D-code bus

data bus

AHB AHB periph1 32-bit

CCM data

RAM

system bus SRAM and

DMA1

External memories

DMA2

AHB periph2

USB HS

Ethernet

Access to instruction in Flash memory Access to data and literal pool in Flash memory FLITF register access

MS30468V3

RM0090 Rev 18 73/1749

112

--- Page 74 ---

**Embedded Flash memory interface**

**RM0090**

**Figure 4. Flash memory interface connection inside system architecture (STM32F42xxx and STM32F43xxx)**

AHB Flash 32-bit memory instruction Flash interface bus bus 128 bits

Cortex-M4 with FPU

I-Code bus I-Code

Flash memory AHB

Cortex D-Code

core

S bus

32-bit FLITF registers

D-code bus

data bus

AHB AHB periph1 32-bit

CCM data

RAM

system bus SRAM and

DMA1

external memories

DMA2

AHB periph2

DMA2D

LCD-TFT

USB HS

Ethernet

Access to instruction in Flash memory Access to data and literal pool in Flash memory FLITF register access

MS30466V3

