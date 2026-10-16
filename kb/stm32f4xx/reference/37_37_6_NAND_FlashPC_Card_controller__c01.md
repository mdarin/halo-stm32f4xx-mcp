> [RM0090 ch.37 §37.6] NAND Flash/PC Card controller


### 37.6 NAND Flash/PC Card controller

The FMC generates the appropriate signal timings to drive the following types of device:
*   8- and 16-bit NAND Flash memories
*   16-bit PC Card compatible devices

The NAND Flash/PC Card controller can control three external banks, Bank 2, 3 and 4:
*   Bank 2 and Bank 3 support NAND Flash devices
*   Bank 4 supports PC Card devices.

Each bank is configured through dedicated registers (Section 37.6.8). The programmable memory parameters include access timings (shown in Table 289) and ECC configuration.

ST
RM0090 Rev 18
1647/1749

--- Page 88 ---

Flexible memory controller (FMC)
RM0090

Table 289. Programmable NAND Flash/PC Card access parameters

| Parameter | Function | Access mode | Unit | Min. | Max. |
|---|---|---|---|---|---|
| Memory setup time | Number of clock cycles (HCLK) required to set up the address before the command assertion | Read/Write | AHB clock cycle (HCLK) | 1 | 256 |
| Memory wait | Minimum duration (in HCLK clock cycles) of the command assertion | Read/Write | AHB clock cycle (HCLK) | 2 | 255 |
| Memory hold | Number of clock cycles (HCLK) during which the address must be held (as well as the data if a write access is performed) after the command de-assertion | Read/Write | AHB clock cycle (HCLK) | 1 | 254 |
| Memory databus high-Z | Number of clock cycles (HCLK) during which the data bus is kept in high-Z state after a write access has started | Write | AHB clock cycle (HCLK) | 1 | 255 |


37.6.1 **External memory interface signals**

The following tables list the signals that are typically used to interface NAND Flash memory and PC Card.

Note: The prefix “N” identifies the signals which are active low.

**8-bit NAND Flash memory**

Table 290. 8-bit NAND Flash

| FMC signal name | I/O | Function |
|---|---|---|
| A[17] | O | NAND Flash address latch enable (ALE) signal |
| A[16] | O | NAND Flash command latch enable (CLE) signal |
| D[7:0] | I/O | 8-bit multiplexed, bidirectional address/data bus |
| NCE[x] | O | Chip Select, x = 2, 3 |
| NOE(= NRE) | O | Output enable (memory signal name: read enable, NRE) |
| NWE | O | Write enable |
| NWAIT/INT[3:2] | I | NAND Flash ready/busy input signal to the FMC |


Theoretically, there is no capacity limitation as the FMC can manage as many address cycles as needed.

1648/1749
RM0090 Rev 18
ST

--- Page 89 ---

RM0090
Flexible memory controller (FMC)

**16-bit NAND Flash memory**

Table 291. 16-bit NAND Flash

| FMC signal name | I/O | Function |
|---|---|---|
| A[17] | O | NAND Flash address latch enable (ALE) signal |
| A[16] | O | NAND Flash command latch enable (CLE) signal |
| D[15:0] | I/O | 16-bit multiplexed, bidirectional address/data bus |
| NCE[x] | O | Chip Select, x = 2, 3 |
| NOE(= NRE) | O | Output enable (memory signal name: read enable, NRE) |
| NWE | O | Write enable |
| NWAIT/INT[3:2] | I | NAND Flash ready/busy input signal to the FMC |


*Theoretically, there is no capacity limitation as the FMC can manage as many address cycles as needed.*

Table 292. 16-bit PC Card

| FMC signal name | I/O | Function |
|---|---|---|
| A[10:0] | O | Address bus |
| NIORD | O | Output enable for I/O space |
| NIOWR | O | Write enable for I/O space |
| NREG | O | Register signal indicating if access is in Common or Attribute space |
| D[15:0] | I/O | Bidirectional databus |
| NCE4_1 | O | Chip Select 1 |
| NCE4_2 | O | Chip Select 2 (indicates if access is 16-bit or 8-bit) |
| NOE | O | Output enable in Common and in Attribute space |
| NWE | O | Write enable in Common and in Attribute space |
| NWAIT | I | PC Card wait input signal to the FMC (memory signal name IORDY) |
| INTR | I | PC Card interrupt to the FMC (only for PC Cards that can generate an interrupt) |
| CD | I | PC Card presence detection. Active high. If an access is performed to the PC Card banks while CD is low, an AHB error is generated. Refer to Section 37.3: AHB interface |


ST
RM0090 Rev 18
1649/1749

--- Page 90 ---

Flexible memory controller (FMC)
RM0090

> [RM0090 ch.37 §37.6] NAND Flash/PC Card controller

### 37.6.2 NAND Flash / PC Card supported memories and transactions

### 37.6.2 NAND Flash / PC Card supported memories and transactions

*Table 293 shows the supported devices, access modes and transactions. Transactions not allowed (or not supported) by the NAND Flash / PC Card controller are shown in gray.*

**Table 293. Supported memories and transactions**

| Device | Mode | R/W | AHB data size | Memory data size | Allowed/not allowed | Comments |
|---|---|---|---|---|---|---|
| NAND 8-bit | Asynchronous | R | 8 | Y | - |  |
| Asynchronous | W | 8 | Y | - |  |  |
| Asynchronous | R | 16 | 8 | Y | Split into 2 FMC accesses |  |
| Asynchronous | W | 16 | 8 | Y | Split into 2 FMC accesses |  |
| Asynchronous | R | 32 | 8 | Y | Split into 4 FMC accesses |  |
| Asynchronous | W | 32 | 8 | Y | Split into 4 FMC accesses |  |
| NAND 16-bit | Asynchronous | R | 8 | 16 | Y | - |
| Asynchronous | W | 8 | 16 | N | - |  |
| Asynchronous | R | 16 | Y | - |  |  |
| Asynchronous | W | 16 | Y | - |  |  |
| Asynchronous | R | 32 | 16 | Y | Split into 2 FMC accesses |  |
| Asynchronous | W | 32 | 16 | Y | Split into 2 FMC accesses |  |


