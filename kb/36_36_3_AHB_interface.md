### 36.3 AHB interface

The AHB slave interface enables internal CPUs and other bus master peripherals to access the external static memories.

AHB transactions are translated into the external device protocol. In particular, if the selected external memory is 16 or 8 bits wide, 32-bit wide transactions on the AHB are split into consecutive 16- or 8-bit accesses. The FSMC Chip Select (FSMC_NEx) does not

ST
RM0090 Rev 18
1545/1749

--- Page 181 ---

Flexible static memory controller (FSMC)
RM0090

toggle between consecutive accesses except when performing accesses in mode D with the extended mode enabled.

The FSMC generates an AHB error in the following conditions:
* When reading or writing to an FSMC bank which is not enabled
* When reading or writing to the NOR Flash bank while the FACCEN bit is reset in the FSMC_BCRx register.
* When reading or writing to the PC Card banks while the input pin FSMC_CD (Card Presence Detection) is low.

The effect of this AHB error depends on the AHB master which has attempted the R/W access:
* If it is the Cortex®-M4 with FPU CPU, a hard fault interrupt is generated
* If is a DMA, a DMA transfer error is generated and the corresponding DMA channel is automatically disabled.

The AHB clock (HCLK) is the reference clock for the FSMC.

### 36.3.1 Supported memories and transactions

**General transaction rules**

The requested AHB transaction data size can be 8-, 16- or 32-bit wide whereas the accessed external device has a fixed data width. This may lead to inconsistent transfers.

Therefore, some simple transaction rules must be followed:
* AHB transaction size and memory data size are equal
  There is no issue in this case.
* AHB transaction size is greater than the memory size
  In this case, the FSMC splits the AHB transaction into smaller consecutive memory accesses in order to meet the external data width.
* AHB transaction size is smaller than the memory size
  Asynchronous transfers may or not be consistent depending on the type of external device.
  - Asynchronous accesses to devices that have the byte select feature (SRAM, ROM, PSRAM).
    a) FSMC allows write transactions accessing the right data through its byte lanes NBL[1:0]
    b) Read transactions are allowed. All memory bytes are read and the useless ones are discarded. The NBL[1:0] are kept low during read transactions.
  - Asynchronous accesses to devices that do not have the byte select feature (NOR and NAND Flash 16-bit).
    This situation occurs when a byte access is requested to a 16-bit wide Flash memory. Clearly, the device cannot be accessed in byte mode (only 16-bit words can be read from/written to the Flash memory) therefore:
    a) Write transactions are not allowed
    b) Read transactions are allowed. All memory bytes are read and the useless ones are discarded. The NBL[1:0] are set to 0 during read transactions.

1546/1749
RM0090 Rev 18
ST

--- Page 182 ---

RM0090
Flexible static memory controller (FSMC)

**Configuration registers**
The FSMC can be configured using a register set. See Section 36.5.6, for a detailed description of the NOR Flash/PSRAM control registers. See Section 36.6.8, for a detailed description of the NAND Flash/PC Card registers.

