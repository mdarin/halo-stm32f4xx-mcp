> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller


### 36.6 NAND Flash/PC Card controller

The FSMC generates the appropriate signal timings to drive the following types of device:
* NAND Flash
  - 8-bit
  - 16-bit
* 16-bit PC Card compatible devices

The NAND/PC Card controller can control three external banks. Bank 2 and bank 3 support NAND Flash devices. Bank 4 supports PC Card devices.

Each bank is configured by means of dedicated registers (Section 36.6.8). The programmable memory parameters include access timings (shown in Table 246) and ECC configuration.

1584/1749 RM0090 Rev 18 ST

--- Page 25 ---

RM0090
Flexible static memory controller (FSMC)

**Table 246. Programmable NAND/PC Card access parameters**

| Parameter | Function | Access mode | Unit | Min. | Max. |
|---|---|---|---|---|---|
| Memory setup time | Number of clock cycles (HCLK) to set up the address before the command assertion | Read/Write | AHB clock cycle (HCLK) | 1 | 255 |
| Memory wait | Minimum duration (HCLK clock cycles) of the command assertion | Read/Write | AHB clock cycle (HCLK) | 2 | 256 |
| Memory hold | Number of clock cycles (HCLK) to hold the address (and the data in case of a write access) after the command de-assertion | Read/Write | AHB clock cycle (HCLK) | 1 | 254 |
| Memory databus high-Z | Number of clock cycles (HCLK) during which the databus is kept in high-Z state after the start of a write access | Write | AHB clock cycle (HCLK) | 0 | 255 |


> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 36.6.1 External memory interface signals

### 36.6.1 External memory interface signals

The following tables list the signals that are typically used to interface NAND Flash and PC Card.

Note: Prefix “N”. specifies the associated signal as active low.

> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 8-bit NAND Flash

#### 8-bit NAND Flash

**Table 247. 8-bit NAND Flash**

| FSMC signal name | I/O | Function |
|---|---|---|
| A[17] | O | NAND Flash address latch enable (ALE) signal |
| A[16] | O | NAND Flash command latch enable (CLE) signal |
| D[7:0] | I/O | 8-bit multiplexed, bidirectional address/data bus |
| NCE[x] | O | Chip select, x = 2, 3 |
| NOE(= NRE) | O | Output enable (memory signal name: read enable, NRE) |
| NWE | O | Write enable |
| NWAIT/INT[3:2] | I | NAND Flash ready/busy input signal to the FSMC |


There is no theoretical capacity limitation as the FSMC can manage as many address cycles as needed.

ST
RM0090 Rev 18
1585/1749

--- Page 26 ---

Flexible static memory controller (FSMC)
RM0090

> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 16-bit NAND Flash

## 16-bit NAND Flash

Table 248. 16-bit NAND Flash

| FSMC signal name | I/O | Function |
|---|---|---|
| A[17] | O | NAND Flash address latch enable (ALE) signal |
| A[16] | O | NAND Flash command latch enable (CLE) signal |
| D[15:0] | I/O | 16-bit multiplexed, bidirectional address/data bus |
| NCE[x] | O | Chip select, x = 2, 3 |
| NOE(= NRE) | O | Output enable (memory signal name: read enable, NRE) |
| NWE | O | Write enable |
| NWAIT/INT[3:2] | I | NAND Flash ready/busy input signal to the FSMC |


*There is no theoretical capacity limitation as the FSMC can manage as many address cycles as needed.*

> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 16-bit PC Card

## 16-bit PC Card

Table 249. 16-bit PC Card

| FSMC signal name | I/O | Function |
|---|---|---|
| A[10:0] | O | Address bus |
| NIORD | O | Output enable for I/O space |
| NIOWR | O | Write enable for I/O space |
| NREG | O | Register signal indicating if access is in Common or Attribute space |
| D[15:0] | I/O | Bidirectional databus |
| NCE4_1 | O | Chip select 1 |
| NCE4_2 | O | Chip select 2 (indicates if access is 16-bit or 8-bit) |
| NOE | O | Output enable in Common and in Attribute space |
| NWE | O | Write enable in Common and in Attribute space |
| NWAIT | I | PC Card wait input signal to the FSMC (memory signal name IORDY) |
| INTR | I | PC Card interrupt to the FSMC (only for PC Cards that can generate an interrupt) |
| CD | I | PC Card presence detection. Active high. If an access is performed to the PC Card banks while CD is low, an AHB error is generated. Refer to Section 36.3: AHB interface |


1586/1749
RM0090 Rev 18
ST

--- Page 27 ---

RM0090
Flexible static memory controller (FSMC)

> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 36.6.2 NAND Flash / PC Card supported memories and transactions

### 36.6.2 NAND Flash / PC Card supported memories and transactions

Table 250 below shows the supported devices, access modes and transactions.
Transactions not allowed (or not supported) by the NAND Flash / PC Card controller appear in gray.

**Table 250. Supported memories and transactions**

| Device | Mode | R/W | AHB data size | Memory data size | Allowed/not allowed | Comments |
|---|---|---|---|---|---|---|
| NAND 8-bit | Asynchronous | R | 8 | Y |  |  |
| Asynchronous | W | 8 | Y |  |  |  |
| Asynchronous | R | 16 | 8 | Y | Split into 2 FSMC accesses |  |
| Asynchronous | W | 16 | 8 | Y | Split into 2 FSMC accesses |  |
| Asynchronous | R | 32 | 8 | Y | Split into 4 FSMC accesses |  |
| Asynchronous | W | 32 | 8 | Y | Split into 4 FSMC accesses |  |
| NAND 16-bit | Asynchronous | R | 8 | 16 | Y |  |
| Asynchronous | W | 8 | 16 | N |  |  |
| Asynchronous | R | 16 | Y |  |  |  |
| Asynchronous | W | 16 | Y |  |  |  |
| Asynchronous | R | 32 | 16 | Y | Split into 2 FSMC accesses |  |
| Asynchronous | W | 32 | 16 | Y | Split into 2 FSMC accesses |  |


> [RM0090 ch.36 §36.6] NAND Flash/PC Card controller

### 36.6.3 Timing diagrams for NAND and PC Card

### 36.6.3 Timing diagrams for NAND and PC Card

Each PC Card/CompactFlash and NAND Flash memory bank is managed through a set of registers:

* Control register: FSMC_PCRx
* Interrupt status register: FSMC_SRx
* ECC register: FSMC_ECCRx
* Timing register for Common memory space: FSMC_PMEMx
* Timing register for Attribute memory space: FSMC_PATTx
* Timing register for I/O space: FSMC_PIOx

Each timing configuration register contains three parameters used to define number of HCLK cycles for the three phases of any PC Card/CompactFlash or NAND Flash access, plus one parameter that defines the timing for starting driving the databus in the case of a write. Figure 454 shows the timing parameter definitions for common memory accesses, knowing that Attribute and I/O (only for PC Card) memory space access timings are similar.

ST
RM0090 Rev 18
1587/1749

--- Page 28 ---

Flexible static memory controller (FSMC)
RM0090

Figure 454. NAND/PC Card controller timing for common memory access

1. NOE remains high (inactive) during write access. NWE remains high (inactive) during read access.
2. For write accesses, the hold phase delay is (MEMHOLD) x HCLK cycles, while it is (MEMHOLD + 2) x HCLK cycles for read accesses.

