**3.5 Read interface**

**3.5.1 Relation between CPU clock frequency and Flash memory read time**

To correctly read data from Flash memory, the number of wait states (LATENCY) must be correctly programmed in the Flash access control register (FLASH_ACR) according to the frequency of the CPU clock (HCLK) and the supply voltage of the device.

The prefetch buffer must be disabled when the supply voltage is below 2.1 V. The correspondence between wait states and CPU clock frequency is given in Table 10 and Table 11 .

Note: On STM32F405xx/07xx and STM32F415xx/17xx devices:

- when VOS = '0', the maximum value of f HCLK = 144 MHz.

- when VOS = '1', the maximum value of f HCLK = 168 MHz.

On STM32F42xxx and STM32F43xxx devices:

- when VOS[1:0] = '0x01', the maximum value of f HCLK is 120 MHz.

- when VOS[1:0] = '0x10', the maximum value of f HCLK is 144 MHz. It can be extended to 168 MHz by activating the over-drive mode.

- when VOS[1:0] = '0x11, the maximum value of f HCLK is 168 MHz. It can be extended to 180 MHz by activating the over-drive mode.

- The over-drive mode is not available when V DD ranges from 1.8 to 2.1 V.

Refer to Section 5.1.4: Voltage regulator for STM32F42xxx and STM32F43xxx for details on how to activate the over-drive mode.

**Table 10. Number of wait states according to CPU clock (HCLK) frequency**

**(STM32F405xx/07xx and STM32F415xx/17xx)**

| Wait states (WS) (LATENCY) | HCLK (MHz) |  |  |  |
|---|---|---|---|---|
| Voltage range 2.7 V - 3.6 V | Voltage range 2.4 V - 2.7 V | Voltage range 2.1 V - 2.4 V | Voltage range 1.8 V - 2.1 V Prefetch OFF |  |
| 0 WS (1 CPU cycle) | 0 < HCLK ≤ 30 | 0 < HCLK ≤ 24 | 0 < HCLK ≤ 22 | 0 < HCLK ≤ 20 |
| 1 WS (2 CPU cycles) | 30 < HCLK ≤ 60 | 24 < HCLK ≤ 48 | 22 < HCLK ≤ 44 | 20 <HCLK ≤ 40 |
| 2 WS (3 CPU cycles) | 60 < HCLK ≤ 90 | 48 < HCLK ≤ 72 | 44 < HCLK ≤ 66 | 40 < HCLK ≤ 60 |
| 3 WS (4 CPU cycles) | 90 < HCLK ≤ 120 | 72 < HCLK ≤ 96 | 66 < HCLK ≤ 88 | 60 < HCLK ≤ 80 |
| 4 WS (5 CPU cycles) | 120 < HCLK ≤ 150 | 96 < HCLK ≤ 120 | 88 < HCLK ≤ 110 | 80 < HCLK ≤ 100 |
| 5 WS (6 CPU cycles) | 150 < HCLK ≤ 168 | 120 < HCLK ≤ 144 | 110 < HCLK ≤ 132 | 100 < HCLK ≤ 120 |
| 6 WS (7 CPU cycles) |  | 144 < HCLK ≤ 168 | 132 < HCLK ≤ 154 | 120 < HCLK ≤ 140 |
| 7 WS (8 CPU cycles) |  |  | 154 < HCLK ≤ 168 | 140 < HCLK ≤ 160 |




--- Page 81 ---

**RM0090 Embedded Flash memory interface**

**Table 11. Number of wait states according to CPU clock (HCLK) frequency**

**(STM32F42xxx and STM32F43xxx)**

| Wait states (WS) (LATENCY) | HCLK (MHz) |  |  |  |
|---|---|---|---|---|
| Voltage range 2.7 V - 3.6 V | Voltage range 2.4 V - 2.7 V | Voltage range 2.1 V - 2.4 V | Voltage range 1.8 V - 2.1 V Prefetch OFF |  |
| 0 WS (1 CPU cycle) | 0 <HCLK ≤ 30 | 0 <HCLK ≤ 24 | 0 <HCLK ≤ 22 | 0 < HCLK ≤ 20 |
| 1 WS (2 CPU cycles) | 30 <HCLK ≤ 60 | 24 < HCLK ≤ 48 | 22 <HCLK ≤ 44 | 20 <HCLK ≤ 40 |
| 2 WS (3 CPU cycles) | 60 <HCLK ≤ 90 | 48 < HCLK ≤ 72 | 44 < HCLK ≤ 66 | 40 < HCLK ≤ 60 |
| 3 WS (4 CPU cycles) | 90 <HCLK ≤ 120 | 72 < HCLK ≤ 96 | 66 <HCLK ≤ 88 | 60 < HCLK ≤ 80 |
| 4 WS (5 CPU cycles) | 120 <HCLK ≤ 150 | 96 < HCLK ≤ 120 | 88 < HCLK ≤ 110 | 80 < HCLK ≤ 100 |
| 5 WS (6 CPU cycles) | 150 <HCLK ≤ 180 | 120 <HCLK ≤ 144 | 110 < HCLK ≤ 132 | 100 < HCLK ≤ 120 |
| 6 WS (7 CPU cycles) |  | 144 <HCLK ≤ 168 | 132 < HCLK ≤ 154 | 120 < HCLK ≤ 140 |
| 7 WS (8 CPU cycles) |  | 168 <HCLK ≤ 180 | 154 <HCLK ≤ 176 | 140 < HCLK ≤ 160 |
| 8 WS (9 CPU cycles) |  |  | 176 <HCLK ≤ 180 | 160 < HCLK ≤ 168 |


After reset, the CPU clock frequency is 16 MHz and 0 wait state (WS) is configured in the FLASH_ACR register.

It is highly recommended to use the following software sequences to tune the number of wait states needed to access the Flash memory with the CPU frequency.

**Increasing the CPU frequency**

1. Program the new number of wait states to the LATENCY bits in the FLASH_ACR

register

2. Check that the new number of wait states is taken into account to access the Flash

memory by reading the FLASH_ACR register

3. Modify the CPU clock source by writing the SW bits in the RCC_CFGR register

4. If needed, modify the CPU clock prescaler by writing the HPRE bits in RCC_CFGR

5. Check that the new CPU clock source or/and the new CPU clock prescaler value is/are

taken into account by reading the clock source status (SWS bits) or/and the AHB prescaler value (HPRE bits), respectively, in the RCC_CFGR register.

**Decreasing the CPU frequency**

1. Modify the CPU clock source by writing the SW bits in the RCC_CFGR register

2. If needed, modify the CPU clock prescaler by writing the HPRE bits in RCC_CFGR

3. Check that the new CPU clock source or/and the new CPU clock prescaler value is/are

taken into account by reading the clock source status (SWS bits) or/and the AHB prescaler value (HPRE bits), respectively, in the RCC_CFGR register

4. Program the new number of wait states to the LATENCY bits in FLASH_ACR

5. Check that the new number of wait states is used to access the Flash memory by

reading the FLASH_ACR register

RM0090 Rev 18 81/1749

112

--- Page 82 ---

**Embedded Flash memory interface RM0090**

Note: A change in CPU clock configuration or wait state (WS) configuration may not be effective straight away. To make sure that the current CPU clock frequency is the one you have configured, you can check the AHB prescaler factor and clock source status values. To make sure that the number of WS you have programmed is effective, you can read the FLASH_ACR register.

**3.5.2 Adaptive real-time memory accelerator (ART Accelerator™)**

The proprietary Adaptive real-time (ART) memory accelerator is optimized for STM32 industry-standard Arm ® Cortex ® -M4 with FPU processors. It balances the inherent performance advantage of the Arm ® Cortex ® -M4 with FPU over Flash memory technologies, which normally requires the processor to wait for the Flash memory at higher operating frequencies.

To release the processor full performance, the accelerator implements an instruction prefetch queue and branch cache which increases program execution speed from the 128- bit Flash memory. Based on CoreMark benchmark, the performance achieved thanks to the ART accelerator is equivalent to 0 wait state program execution from Flash memory at a CPU frequency up to 180 MHz.

**Instruction prefetch**

Each Flash memory read operation provides 128 bits from either four instructions of 32 bits or 8 instructions of 16 bits according to the program launched. So, in case of sequential code, at least four CPU cycles are needed to execute the previous read instruction line. Prefetch on the I-Code bus can be used to read the next sequential instruction line from the Flash memory while the current instruction line is being requested by the CPU. Prefetch is enabled by setting the PRFTEN bit in the FLASH_ACR register. This feature is useful if at least one wait state is needed to access the Flash memory.

Figure 5 shows the execution of sequential 32-bit instructions with and without prefetch when 3 WSs are needed to access the Flash memory.



--- Page 83 ---

|  |  |  |  |  | Embedded Flash memory interface |
|---|---|---|---|---|---|
|  |  |  | Figure 5 | . Seque |  |
| @ F D E WAIT 1 1 1 1 Without prefetch @ F D E 2 2 2 2 @ F D E 3 3 3 3 @ F D E 4 4 4 4 @ F D E WAIT 5 5 5 5 @ F D E 6 6 6 6 @ F D 7 7 7 @ F 8 8 ins 1 ins 2 ins 3 ins 4 ins 5 ins 6 ins 7 ins 8 fetch fetch fetch fetch fetch fetch fetch fetch Read ins 1, 2, 3, 4 Gives ins 1, 2, 3, 4 Read ins 5, 6, 7, 8 Gives ins 5, 6, 7, 8 @ Wait data F D E 1 1 1 1 With prefetch @ F D E 2 2 2 2 @ F D E 3 3 3 3 @ F D E 4 4 4 4 @ F D E 5 5 5 5 @ F D E 6 6 6 @ F D Cortex-M4 pipeline 7 7 7 @ F @ F D E 8 8 AHB protocol ins 1 ins 2 ins 3 ins 4 ins 5 ins 6 ins 7ins 8 fetch fetch fetch fetch fetch fetch fetchfetch @ : address requested F: Fetch stage D: Decode stage Read ins 1, 2, 3, 4 Gives ins 1, 2, 3, 4 Gives ins 5, 6, 7, 8 E: Execute stage Read ins 5, 6, 7, 8 Read ins 9, 10, ... MS31831V1 |  |  |  |  |  |


**RM0090**

| @ 1 | WAIT | F 1 | D 1 | E 1 |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| @ 2 | F 2 | D 2 | E 2 |  |  |  |  |  |  |
| @ 3 | F 3 | D 3 | E 3 |  |  |  |  |  |  |
| @ 4 | F 4 | D 4 | E 4 |  |  |  |  |  |  |
| @ 5 | WAIT | F 5 | D 5 | E 5 |  |  |  |  |  |
| @ 6 | F 6 | D 6 | E 6 |  |  |  |  |  |  |
|  |  |  |  |  |  | @ 7 | F 7 | D 7 |  |
| @ 8 | F 8 |  |  |  |  |  |  |  |  |
|  |  |  | ins 2 fetch | ins 4 in etch fe | s 6 ins tch fetc |  |  |  |  |
| Read ins 1, 2, 3, 4 | Gives ins 1, 2, 3, 4 | Read ins 5, 6, 7, 8 | Gives ins 5, 6, 7, 8 |  |  |  |  |  |  |


| @ 1 | Wait data | F 1 | D 1 | E 1 |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| @ 2 | F 2 | D 2 | E 2 |  |  |  |  |  |
| @ 3 | F 3 | D 3 | E 3 |  |  |  |  |  |
| @ 4 | F 4 | D 4 | E 4 |  |  |  |  |  |
|  |  | @ 5 | F 5 | D 5 | E 5 | E 6 |  |  |
| @ | F 6 | D 6 | E 6 |  |  |  |  |  |
|  |  |  |  |  |  | @ 7 | F 7 | D 7 |
| @ | F 8 |  |  |  |  |  |  |  |






When the code is not sequential (branch), the instruction may not be present in the currently used instruction line or in the prefetched instruction line. In this case (miss), the penalty in terms of number of cycles is at least equal to the number of wait states.



112

--- Page 84 ---

**Embedded Flash memory interface RM0090**

**Instruction cache memory**

To limit the time lost due to jumps, it is possible to retain 64 lines of 128 bits in an instruction cache memory. This feature can be enabled by setting the instruction cache enable (ICEN) bit in the FLASH_ACR register. Each time a miss occurs (requested data not present in the currently used instruction line, in the prefetched instruction line or in the instruction cache memory), the line read is copied into the instruction cache memory. If some data contained in the instruction cache memory are requested by the CPU, they are provided without inserting any delay. Once all the instruction cache memory lines have been filled, the LRU (least recently used) policy is used to determine the line to replace in the instruction memory cache. This feature is particularly useful in case of code containing loops.

**Data management**

Literal pools are fetched from Flash memory through the D-Code bus during the execution stage of the CPU pipeline. The CPU pipeline is consequently stalled until the requested literal pool is provided. To limit the time lost due to literal pools, accesses through the AHB databus D-Code have priority over accesses through the AHB instruction bus I-Code. If some literal pools are frequently used, the data cache memory can be enabled by setting the data cache enable (DCEN) bit in the FLASH_ACR register. This feature works like the instruction cache memory, but the retained data size is limited to 8 rows of 128 bits. Note: Data in user configuration sector are not cacheable.

