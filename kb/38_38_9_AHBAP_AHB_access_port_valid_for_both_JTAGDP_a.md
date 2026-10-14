### 38.9 AHB-AP (AHB access port) - valid for both JTAG-DP and SW-DP

**Features:**
* System access is independent of the processor status.
* Either SW-DP or JTAG-DP accesses AHB-AP.
* The AHB-AP is an AHB master into the Bus Matrix. Consequently, it can access all the data buses (Dcode Bus, System Bus, internal and external PPB bus) but the ICode bus.
* Bitband transactions are supported.
* AHB-AP transactions bypass the FPB.

The address of the 32-bits AHP-AP resisters are 6-bits wide (up to 64 words or 256 bytes) and consists of:
c) Bits [7:4] = the bits [7:4] APBANKSEL of the DP SELECT register
d) Bits [3:2] = the 2 address bits of A[3:2] of the 35-bit packet request for SW-DP.

The AHB-AP of the Cortex®-M4 with FPU includes 9 x 32-bits registers:

**Table 306. Cortex®-M4 with FPU AHB-AP registers**

| Address offset | Register name | Notes |
|---|---|---|
| 0x00 | AHB-AP Control and Status Word | Configures and controls transfers through the AHB interface (size, hprot, status on current transfer, address increment type |
| 0x04 | AHB-AP Transfer Address | - |
| 0x0C | AHB-AP Data Read/Write | - |
| 0x10 | AHB-AP Banked Data 0 | Directly maps the 4 aligned data words without rewriting the Transfer Address Register. |
| 0x14 | AHB-AP Banked Data 1 |  |
| 0x18 | AHB-AP Banked Data 2 |  |
| 0x1C | AHB-AP Banked Data 3 |  |
| 0xF8 | AHB-AP Debug ROM Address | Base Address of the debug interface |
| 0xFC | AHB-AP ID Register | - |


Refer to the Cortex®-M4 with FPU *r0p1 TRM* for further details.

1696/1749
RM0090 Rev 18

--- Page 137 ---

RM0090
Debug support (DBG)

