### 38.7 JTAG debug port

A standard JTAG state machine is implemented with a 4-bit instruction register (IR) and five data registers (for full details, refer to the Cortex®-M4 with FPU r0p1 Technical Reference Manual (TRM), for references, see Section 38.2).

Table 300. JTAG debug port data registers

| IR(3:0) | Data register | Details |
|---|---|---|
| 1111 | BYPASS [1 bit] | - |
| 1110 | IDCODE [32 bits] | ID CODE 0x4BA00477 (Arm® Cortex®-M4 with FPU r0p1 ID Code) |
| 1010 | DPACC [35 bits] | Debug port access register This initiates a debug port and allows access to a debug port register. – When transferring data IN: Bits 34:3 = DATA[31:0] = 32-bit data to transfer for a write request Bits 2:1 = A[3:2] = 2-bit address of a debug port register. Bit 0 = RnW = Read request (1) or write request (0). – When transferring data OUT: Bits 34:3 = DATA[31:0] = 32-bit data which is read following a read request Bits 2:0 = ACK[2:0] = 3-bit Acknowledge: 010 = OK/FAULT 001 = WAIT OTHER = reserved Refer to Table 301 for a description of the A[3:2] bits |


RM0090 Rev 18
1691/1749

--- Page 132 ---

Debug support (DBG)
RM0090

Table 300. JTAG debug port data registers (continued)

| IR(3:0) | Data register | Details |
|---|---|---|
| 1011 | APACC [35 bits] | Access port access registerInitiates an access port and allows access to an access port register.– When transferring data IN:Bits 34:3 = DATA[31:0] = 32-bit data to shift in for a write requestBits 2:1 = A[3:2] = 2-bit address (sub-address AP registers).Bit 0 = RnW= Read request (1) or write request (0).– When transferring data OUT:Bits 34:3 = DATA[31:0] = 32-bit data which is read following a read requestBits 2:0 = ACK[2:0] = 3-bit Acknowledge:010 = OK/FAULT001 = WAITOTHER = reservedThere are many AP Registers (see AHB-AP) addressed as the combination of:– The shifted value A[3:2]– The current value of the DP SELECT register |
| Abort register– Bits 31:1 = Reserved– Bit 0 = DAPABORT: write 1 to generate a DAP abort. |  |  |


Table 301. 32-bit debug port registers addressed through the shifted value A[3:2]

| Address | A[3:2] value | Description |
|---|---|---|
| 0x0 | 00 | Reserved, must be kept at reset value. |
| 0x4 | 01 | DP CTRL/STAT register. Used to:– Request a system or debug power-up– Configure the transfer operation for AP accesses– Control the pushed compare and pushed verify operations.– Read some status flags (overrun, power-up acknowledges) |
| 0x8 | 10 | DP SELECT register: Used to select the current access port and the active 4-words register window.– Bits 31:24: APSEL: select the current AP– Bits 23:8: reserved– Bits 7:4: APBANKSEL: select the active 4-words register window on the current AP– Bits 3:0: reserved |
| 0xC | 11 | DP RDBUFF register: Used to allow the debugger to get the final result after a sequence of operations (without requesting new JTAG-DP operation) |


1692/1749
RM0090 Rev 18

--- Page 133 ---

RM0090
Debug support (DBG)

