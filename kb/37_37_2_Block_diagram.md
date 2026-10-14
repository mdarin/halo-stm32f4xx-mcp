### 37.2 Block diagram

The FMC consists of five main blocks:

*   The AHB interface (including the FMC configuration registers)
*   The NOR Flash/PSRAM/SRAM controller
*   The NAND Flash/PC Card controller
*   The SDRAM controller
*   The external device interface

The block diagram is shown in Figure 456.

ST
RM0090 Rev 18
1603/1749

--- Page 44 ---

Flexible memory controller (FMC)
RM0090

Figure 456. FMC block diagram

FMC interrupts to NVIC
From clock controller
HCLK
NOR/PSRAM memory controller
Configuration registers
NAND/PC Card memory controller
SDRAM controller
FMC_NL(or NADV)
FMC_CLK
NOR/PSRAM signals
FMC_NBL[3:0]
SRAM/PSRAM/SDRAM Shared signals
FMC_A[25:0]
FMC_D[31:0]
Shared signals
FMC_NE[4:1]
FMC_NOE
FMC_NWE
FMC_NWAIT
NOR/PSRAM/SRAM Shared signals
FMC_NCE[3:2]
FMC_INT[3:2]
NAND signals
FMC_INTR
FMC_NCE4_1
FMC_NCE4_2
FMC_NIORD
FMC_NIOWR
PC Card signals
FMC_NREG
FMC_CD
FMC_SDCLK
FMC_SDNWE
FMC_SDCKE[1:0]
FMC_SDNE[1:0]
FMC_NRAS
FMC_NCAS
SDRAM signals
MS30443V5

37.3 AHB interface

The AHB slave interface allows internal CPUs and other bus master peripherals to access the external memories.

AHB transactions are translated into the external device protocol. In particular, if the selected external memory is 16- or 8-bit wide, 32-bit wide transactions on the AHB are split into consecutive 16- or 8-bit accesses. The FMC Chip Select (FMC_NEx) does not toggle between consecutive accesses except when performing accesses in mode D with the extended mode enabled.

1604/1749
RM0090 Rev 18

--- Page 45 ---

RM0090
Flexible memory controller (FMC)

The FMC generates an AHB error in the following conditions:

*   When reading or writing to an FMC bank (Bank 1 to 4) which is not enabled.
*   When reading or writing to the NOR Flash bank while the FACCEN bit is reset in the FMC_BCRx register.
*   When reading or writing to the PC Card banks while the FMC_CD input pin (Card Presence Detection) is low.
*   When writing to a write protected SDRAM bank (WP bit set in the SDRAM_SDCRx register).
*   When the SDRAM address range is violated (access to reserved address range)

The effect of an AHB error depends on the AHB master which has attempted the R/W access:

*   If the access has been attempted by the Cortex®-M4 with FPU CPU, a hard fault interrupt is generated.
*   If the access has been performed by a DMA controller, a DMA transfer error is generated and the corresponding DMA channel is automatically disabled.

The AHB clock (HCLK) is the reference clock for the FMC.

### 37.3.1 Supported memories and transactions

**General transaction rules**

The requested AHB transaction data size can be 8-, 16- or 32-bit wide whereas the accessed external device has a fixed data width. This may lead to inconsistent transfers.

Therefore, some simple transaction rules must be followed:

*   AHB transaction size and memory data size are equal
    There is no issue in this case.
*   AHB transaction size is greater than the memory size:
    In this case, the FMC splits the AHB transaction into smaller consecutive memory accesses to meet the external data width. The FMC Chip Select (FMC_NEx) does not toggle between the consecutive accesses.
*   AHB transaction size is smaller than the memory size:
    The transfer may or not be consistent depending on the type of external device:
    *   Accesses to devices that have the byte select feature (SRAM, ROM, PSRAM, SDRAM)
        In this case, the FMC allows read/write transactions and accesses the right data through its byte lanes BL[3:0].
        byte to be written are addressed by NBL[3:0].
        All memory byte are read (NBL[3:0] are driven low during read transaction) and the useless ones are discarded.
    *   Accesses to devices that do not have the byte select feature (16-bit NOR and NAND Flash memories)
        This situation occurs when a byte access is requested to a 16-bit wide Flash memory. Since the device cannot be accessed in byte mode (only 16-bit words can be read/written from/to the Flash memory), Write transactions and Read transactions are allowed (the controller reads the entire 16-bit memory word and uses only the required byte).

ST
RM0090 Rev 18
1605/1749

--- Page 46 ---

Flexible memory controller (FMC) RM0090

**Configuration registers**

The FMC can be configured through a set of registers. Refer to Section 37.5.6, for a detailed description of the NOR Flash/PSRAM controller registers. Refer to Section 37.6.8, for a detailed description of the NAND Flash/PC Card registers and to Section 37.7.5 for a detailed description of the SDRAM controller registers.

