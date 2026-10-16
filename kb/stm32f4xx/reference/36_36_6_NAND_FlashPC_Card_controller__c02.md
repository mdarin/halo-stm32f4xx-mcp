> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller


### 36.6.4 NAND Flash operations

The command latch enable (CLE) and address latch enable (ALE) signals of the NAND Flash device are driven by some address signals of the FSMC controller. This means that to send a command or an address to the NAND Flash memory, the CPU has to perform a write to a certain address in its memory space.

A typical page read operation from the NAND Flash device is as follows:

1. Program and enable the corresponding memory bank by configuring the FSMC_PCRx and FSMC_PMEMx (and for some devices, FSMC_PATTx, see Section 36.6.5) registers according to the characteristics of the NAND Flash (PWID bits for the databus width of the NAND Flash, PTYP = 1, PWAITEN = 0 or 1 as needed, see Common memory space timing register 2..4 (FSMC_PMEM2..4) for timing configuration).
2. The CPU performs a byte write in the common memory space, with data byte equal to one Flash command byte (for example 0x00 for Samsung NAND Flash devices). The CLE input of the NAND Flash is active during the write strobe (low pulse on NWE), thus the written byte is interpreted as a command by the NAND Flash. Once the command is latched by the NAND Flash device, it does not need to be written for the following page read operations.
3. The CPU can send the start address (STARTAD) for a read operation by writing the required bytes (for example four bytes or three for smaller capacity devices), STARTAD[7:0], STARTAD[15:8], STARTAD[23:16] and finally STARTAD[25:24] for 64 Mb x 8 bit NAND Flash) in the common memory or attribute space. The ALE input of the NAND Flash device is active during the write strobe (low pulse on NWE), thus the

1588/1749
RM0090 Rev 18

--- Page 29 ---

RM0090
Flexible static memory controller (FSMC)

written bytes are interpreted as the start address for read operations. Using the attribute memory space makes it possible to use a different timing configuration of the FSMC, which can be used to implement the prewait functionality needed by some NAND Flash memories (see details in Section 36.6.5).

4. The controller waits for the NAND Flash to be ready (R/NB signal high) to become active, before starting a new access (to same or another memory bank). While waiting, the controller maintains the NCE signal active (low).
5. The CPU can then perform byte read operations in the common memory space to read the NAND Flash page (data field + Spare field) byte by byte.
6. The next NAND Flash page can be read without any CPU command or address write operation, in three different ways:
   - by simply performing the operation described in step 5
   - a new random address can be accessed by restarting the operation at step 3
   - a new command can be sent to the NAND Flash device by restarting at step 2

> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 36.6.5 NAND Flash prewait functionality

### 36.6.5 NAND Flash prewait functionality

Some NAND Flash devices require that, after writing the last part of the address, the controller wait for the R/NB signal to go low as shown in Figure 455.

**Figure 455. Access to non ‘CE don’t care’ NAND-Flash**

1. CPU wrote byte 0x00 at address 0x7001 0000.
2. CPU wrote byte A7-A0 at address 0x7002 0000.
3. CPU wrote byte A15-A8 at address 0x7002 0000.
4. CPU wrote byte A23-A16 at address 0x7002 0000.
5. CPU wrote byte A25-A24 at address 0x7802 0000: FSMC performs a write access using FSMC_PATT2 timing definition, where ATTHOLD ≥ 7 (providing that (7+1) × HCLK = 112 ns > tWB max). This guarantees that NCE remains low until R/NB goes low and high again (only requested for NAND Flash memories where NCE is not don’t care).

ST
RM0090 Rev 18
1589/1749

--- Page 30 ---

Flexible static memory controller (FSMC)
RM0090

When this functionality is needed, it can be guaranteed by programming the MEMHOLD value to meet the tWB timing. However CPU read accesses to the NAND Flash memory has a hold delay of (MEMHOLD + 2) x HCLK cycles, while CPU write accesses have a hold delay of (MEMHOLD) x HCLK cycles.

To overcome this timing constraint, the attribute memory space can be used by programming its timing register with an ATTHOLD value that meets the tWB timing, and leaving the MEMHOLD value at its minimum. Then, the CPU must use the common memory space for all NAND Flash read and write accesses, except when writing the last address byte to the NAND Flash device, where the CPU must write to the attribute memory space.

