> [RM0090 ch.37 §37.6] NAND Flash/PC Card controller


### 37.6.3 Timing diagrams for NAND Flash memory and PC Card

Each PC Card/CompactFlash and NAND Flash memory bank is managed through a set of registers:

* Control register: FMC_PCRx
* Interrupt status register: FMC_SRx
* ECC register: FMC_ECCRx
* Timing register for Common memory space: FMC_PMEMx
* Timing register for Attribute memory space: FMC_PATTx
* Timing register for I/O space: FMC_PIOx

Each timing configuration register contains three parameters used to define number of HCLK cycles for the three phases of any PC Card/CompactFlash or NAND Flash access, plus one parameter that defines the timing for starting driving the data bus when a write access is performed. *Figure 476* shows the timing parameter definitions for common memory accesses, knowing that Attribute and I/O (only for PC Card) memory space access timings are similar.

1650/1749
RM0090 Rev 18

--- Page 91 ---

RM0090
Flexible memory controller (FMC)

Figure 476. NAND Flash/PC Card controller waveforms for common memory access

HCLK
A[25:0]
NCE
NREG,
NIOW,
NIOR
NWE,
NOE
write_data
read_data

High
MEMxSET
+1
MEMxWAIT + 1
MEMxHOLD
(1)
MEMxHiZ
Valid

MS33733V2

1. NOE remains high (inactive) during write accesses. NWE remains high (inactive) during read accesses.
2. For write accesses, the hold phase delay is (MEMHOLD) x HCLK cycles, while it is (MEMHOLD + 2) x HCLK cycles for read accesses.

37.6.4 NAND Flash operations

The command latch enable (CLE) and address latch enable (ALE) signals of the NAND Flash memory device are driven by address signals from the FMC controller. This means that to send a command or an address to the NAND Flash memory, the CPU has to perform a write to a specific address in its memory space.

A typical page read operation from the NAND Flash device requires the following steps:

3. Program and enable the corresponding memory bank by configuring the FMC_PCRx and FMC_PMEMx (and for some devices, FMC_PATTx, see Section 37.6.5: NAND Flash prewait functionality) registers according to the characteristics of the NAND Flash memory (PWID bits for the data bus width of the NAND Flash, PTYP = 1, PWAITEN = 0 or 1 as needed, see section Section 37.4.2: NAND Flash memory/PC Card address mapping for timing configuration).

4. The CPU performs a byte write to the common memory space, with data byte equal to one Flash command byte (for example 0x00 for Samsung NAND Flash devices). The LE input of the NAND Flash memory is active during the write strobe (low pulse on NWE), thus the written byte is interpreted as a command by the NAND Flash memory. Once the command is latched by the memory device, it does not need to be written again for the following page read operations.

5. The CPU can send the start address (STARTAD) for a read operation by writing four byte (or three for smaller capacity devices), STARTAD[7:0], STARTAD[16:9], STARTAD[24:17] and finally STARTAD[25] (for 64 Mb x 8 bit NAND Flash memories) in the common memory or attribute space. The ALE input of the NAND Flash device is active during the write strobe (low pulse on NWE), thus the written byte are interpreted as the start address for read operations. Using the attribute memory space makes it possible to use a different timing configuration of the FMC, which can be used to

ST
RM0090 Rev 18
1651/1749

--- Page 92 ---

Flexible memory controller (FMC)
RM0090

implement the prewait functionality needed by some NAND Flash memories (see details in Section 37.6.5: NAND Flash prewait functionality).

6. The controller waits for the NAND Flash memory to be ready (R/NB signal high), before starting a new access to the same or another memory bank. While waiting, the controller holds the NCE signal active (low).

7. The CPU can then perform byte read operations from the common memory space to read the NAND Flash page (data field + Spare field) byte by byte.

8. The next NAND Flash page can be read without any CPU command or address write operation. This can be done in three different ways:
   - by simply performing the operation described in step 5
   - a new random address can be accessed by restarting the operation at step 3
   - a new command can be sent to the NAND Flash device by restarting at step 2

> [RM0090 ch.37 §37.6] NAND Flash/PC Card controller

### 37.6.5 NAND Flash prewait functionality

### 37.6.5 NAND Flash prewait functionality

Some NAND Flash devices require that, after writing the last part of the address, the controller waits for the R/NB signal to go low. (see Figure 457).

**Figure 477. Access to non ‘CE don’t care’ NAND-Flash**

1. CPU wrote byte 0x00 at address 0x7001 0000.
2. CPU wrote byte A7~A0 at address 0x7002 0000.
3. CPU wrote byte A16~A9 at address 0x7002 0000.
4. CPU wrote byte A24~A17 at address 0x7002 0000.
5. CPU wrote byte A25 at address 0x7802 0000: FMC performs a write access using FMC_PATT2 timing definition, where ATT HOLD ≥ 7 (providing that (7+1) × HCLK = 112 ns > tWB max). This guarantees that NCE remains low until R/NB goes low and high again (only requested for NAND Flash memories where NCE is not don’t care).

1652/1749
RM0090 Rev 18

--- Page 93 ---

RM0090
Flexible memory controller (FMC)

When this functionality is required, it can be ensured by programming the MEMHOLD value to meet the $t_{WB}$ timing. However CPU read accesses to the NAND Flash memory has a hold delay of (MEMHOLD + 2) x HCLK cycles, while CPU write accesses have a hold delay of (MEMHOLD) x HCLK cycles.

To cope with this timing constraint, the attribute memory space can be used by programming its timing register with an ATTHOLD value that meets the $t_{WB}$ timing, and by keeping the MEMHOLD value at its minimum value. The CPU must then use the common memory space for all NAND Flash read and write accesses, except when writing the last address byte to the NAND Flash device, where the CPU must write to the attribute memory space.

