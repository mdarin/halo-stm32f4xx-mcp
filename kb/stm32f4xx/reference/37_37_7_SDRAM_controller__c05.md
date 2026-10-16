> [RM0090 ch.37 §37.7] SDRAM controller


### 37.7.5 SDRAM controller registers

**SDRAM Control registers 1,2 (FMC_SDCR1,2)**

Address offset: 0x140+ 4* (x − 1), x = 1,2
Reset value: 0x0000 02D0

This register contains the control parameters for each SDRAM memory bank

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|
| Reserved | RPIPE[1:0] | RBURST. | SDCLK[1:0] | WP | CAS[1:0] | NB | MWID[1:0] | NR[1:0] | NC[1:0] |
| rw |  |  |  |  |  |  |  |  |  |


Bits 31:15 Reserved, must be kept at reset value

**Bits 14:13 RPIPE[1:0]: Read pipe**
These bits define the delay, in HCLK clock cycles, for reading data after CAS latency.
00: No HCLK clock cycle delay
01: One HCLK clock cycle delay
10: Two HCLK clock cycle delay
11: reserved, do not use
*Note: The corresponding bits in the FMC_SDCR2 register are read only.*

**Bit 12 RBURST: Burst read**
This bit enables burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO.
0: single read requests are not managed as bursts
1: single read requests are always managed as bursts
*Note: The corresponding bit in the FMC_SDCR2 register is don't care.*

**Bits 11:10 SDCLK[1:0]: SDRAM clock configuration**
These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized.
00: SDCLK clock disabled
01: Reserved
10: SDCLK period = 2 x HCLK periods
11: SDCLK period = 3 x HCLK periods
*Note: The corresponding bits in the FMC_SDCR2 register are read only.*

**Bit 9 WP: Write protection**
This bit enables write mode access to the SDRAM bank.
0: Write accesses allowed
1: Write accesses ignored

**Bits 8:7 CAS[1:0]: CAS Latency**
This bits sets the SDRAM CAS latency in number of memory clock cycles
00: reserved, do not use.
01: 1 cycle
10: 2 cycles
11: 3 cycles

1674/1749
RM0090 Rev 18
ST

--- Page 115 ---

**RM0090**
**Flexible memory controller (FMC)**

Bit 6 **NB**: Number of internal banks
This bit sets the number of internal banks.
0: Two internal Banks
1: Four internal Banks

Bits 5:4 **MWID[1:0]**: Memory data bus width.
These bits define the memory device width.
00: 8 bits
01: 16 bits
10: 32 bits
11: reserved, do not use.

Bits 3:2 **NR[1:0]**: Number of row address bits
These bits define the number of bits of a row address.
00: 11 bit
01: 12 bits
10: 13 bits
11: reserved, do not use.

Bits 1:0 **NC[1:0]**: Number of column address bits
These bits define the number of bits of a column address.
00: 8 bits
01: 9 bits
10: 10 bits
11: 11 bits.

*Note:* Before modifying the *RBURST* or *RPIPE* settings or disabling the *SDCLK* clock, the user must first send a *PALL* command to make sure ongoing operations are complete.

**SDRAM Timing registers 1,2 (FMC_SDTR1,2)**

Address offset: 0x148 + 4 * (x – 1), x = 1,2
Reset value: 0x0FFF FFFF
This register contains the timing parameters of each SDRAM bank

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TRCD[3:0] | TRP[3:0] | TWR[3:0] | TRC[3:0] | TRAS[3:0] | TXSR[3:0] | TMRD[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:28 Reserved, must be kept at reset value
Bits 27:24 **TRCD[3:0]**: Row to column delay
These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles.
0000: 1 cycle.
0001: 2 cycles
....
1111: 16 cycles

STI
RM0090 Rev 18
1675/1749

--- Page 116 ---

**Flexible memory controller (FMC)**
RM0090

**Bits 23:20 TRP[3:0]: Row precharge delay**
These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device.
0000: 1 cycle
0001: 2 cycles
....
1111: 16 cycles
*Note: The corresponding bits in the FMC_SDTR2 register are don't care.*

**Bits 19:16 TWR[3:0]: Recovery delay**
These bits define the delay between a Write and a Precharge command in number of memory clock cycles.
0000: 1 cycle
0001: 2 cycles
....
1111: 16 cycles
*Note: TWR must be programmed to match the write recovery time (tWR) defined in the SDRAM datasheet, and to guarantee that:*
*TWR ≥ TRAS - TRCD and TWR ≥ TRC - TRCD - TRP*
*Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR >= 2 cycles. TWR must be programmed to 0x1.*
*If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device.*

**Bits 15:12 TRC[3:0]: Row cycle delay**
These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device.
0000: 1 cycle
0001: 2 cycles
....
1111: 16 cycles
*Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet.*
*Note: The corresponding bits in the FMC_SDTR2 register are don't care.*

**Bits 11:8 TRAS[3:0]: Self refresh time**
These bits define the minimum Self-refresh period in number of memory clock cycles.
0000: 1 cycle
0001: 2 cycles
....
1111: 16 cycles

**Bits 7:4 TXSR[3:0]: Exit Self-refresh delay**
These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles.
0000: 1 cycle
0001: 2 cycles
....
1111: 16 cycles

1676/1749
RM0090 Rev 18

--- Page 117 ---

RM0090
Flexible memory controller (FMC)

Bits 3:0 **TMRD[3:0]**: Load Mode Register to Active
These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles.
0000: 1 cycle
0001: 2 cycles
....
1111: 16 cycles

**Note:**
*If two SDRAM devices are connected, all the accesses performed simultaneously to both devices by the Command Mode register (Load Mode Register and Self-refresh commands) are issued using the timing parameters configured for Bank 1 (TMRD, TRAS and TXSR timings) in the FMC_SDTR1 register.*

*The TRP and TRC timings are only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP and TRC timings must be programmed with the timings of the slowest device.*

**SDRAM Command Mode register (FMC_SDCMR)**

Address offset: 0x150
Reset value: 0x0000 0000

This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks.

| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | MRD[11:0] | NRFS[3:0] | CTB1 | CTB2 | MODE[2:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  | r/w | w |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:22 Reserved, must be kept at reset value

Bits 21:9 **MRD[11:0]**: Mode Register definition
This 13-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command.

Bits 8:5 **NRFS[3:0]**: Number of Auto-refresh
These bits define the number of consecutive Auto-refresh commands issued when MODE = '011'.
0000: 1 Auto-refresh cycle
0001: 2 Auto-refresh cycles
....
1110: 15 Auto-refresh cycles
1111: Reserved

Bit 4 **CTB1**: Command Target Bank 1
This bit indicates whether the command will be issued to SDRAM Bank 1 or not.
0: Command not issued to SDRAM Bank 1
1: Command issued to SDRAM Bank 1

ST
RM0090 Rev 18
1677/1749

--- Page 118 ---

**Flexible memory controller (FMC)**
RM0090

**Bit 3 CTB2:** Command Target Bank 2
This bit indicates whether the command will be issued to SDRAM Bank 2 or not.
0: Command not issued to SDRAM Bank 2
1: Command issued to SDRAM Bank 2

**Bits 2:0 MODE[2:0]:** Command mode
These bits define the command issued to the SDRAM device.
000: Normal Mode
001: Clock Configuration Enable
010: PALL (“All Bank Precharge”) command
011: Auto-refresh command
100: Load Mode Register
101: Self-refresh command
110: Power-down command
111: Reserved

*Note: When a command is issued, at least one Command Target Bank bit (CBT1 or CBT2) must be set. If both banks are used, the commands must be issued to the two banks at the same time by setting the CBT1 and CBT2 bits.*

**SDRAM Refresh Timer register (FMC_SDRTR)**

Address offset: 0x154
Reset value: 0x0000 0000

This register sets the refresh rate in number of SDCLK clock cycles between the refresh cycles by configuring the Refresh Timer Count value.

Refresh rate = (SDRAM refresh rate × SDRAM clock frequency) – 20

SDRAM refresh rate = SDRAM refresh period/ Number of rows

**Example**

SDRAM refresh rate = 64 ms/ (8196rows) = 7.81µs
where 64 ms is the SDRAM refresh period.

7.81µs × 60MHz = 468.6

The refresh rate must be increased by 20 SDRAM clock cycles (as in the above example) to obtain a safe margin if an internal refresh request occurs when a read request has been accepted. It corresponds to a COUNT value of ‘0000111000000’ (448).

This 13-bit field is loaded into a timer which is decremented using the SDRAM clock. This timer generates a refresh pulse when zero is reached. The COUNT value must be set at least to 41 SDRAM clock cycles.

As soon as the FMC_SDRTR register is programmed, the timer starts counting. If the value programmed in the register is ‘0’, no refresh is carried out. This register must not be reprogrammed after the initialization procedure to avoid modifying the refresh rate.

Each time a refresh pulse is generated, this 13-bit COUNT field is reloaded into the counter.

1678/1749
RM0090 Rev 18

--- Page 119 ---

**RM0090**
**Flexible memory controller (FMC)**

If a memory access is in progress, the Auto-refresh request is delayed. However, if the memory access and Auto-refresh requests are generated simultaneously, the Auto-refresh takes precedence. If the memory access occurs during a refresh operation, the request is buffered to be processed when the refresh is complete.

This register common to SDRAM bank 1 and bank 2.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 | 14 | 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | REIE | COUNT[12:0] | CRE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r/w | w |


**Bits 31: 15** Reserved, must be kept at reset value

**Bit 14 REIE**: RES Interrupt Enable
0: Interrupt is disabled
1: An Interrupt is generated if RE = 1

**Bits 13:1 COUNT[12:0]**: Refresh Timer Count
This 13-bit field defines the refresh rate of the SDRAM device. It is expressed in number of memory clock cycles. It must be set at least to 41 SDRAM clock cycles (0x29).
COUNT = (SDRAM refresh rate x SDRAM clock frequency) - 20
SDRAM refresh rate = SDRAM refresh period / Number of rows

**Bit 0 CRE**: Clear Refresh error flag
This bit is used to clear the Refresh Error Flag (RE) in the Status Register.
0: no effect
1: Refresh Error flag is cleared

**Note:** *The programmed COUNT value must not be equal to the sum of the following timings: TWR+TRP+TRC+TRCD+4 memory clock cycles .*

SDRAM Status register (FMC_SDSR)
Address offset: 0x158
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | BUSY | MODES2[1:0] | MODES1[1:0] | RE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | r |


**Bits 31:5** Reserved, must be kept at reset value

**Bit 5 BUSY**: Busy status
This bit defines the status of the SDRAM controller after a Command Mode request
0: SDRAM Controller is ready to accept a new request
1: SDRAM Controller is not ready to accept a new request

**Bits 4:3 MODES2[1:0]**: Status Mode for Bank 2
This bit defines the Status Mode of SDRAM Bank 2.
00: Normal Mode
01: Self-refresh mode
10: Power-down mode

ST
RM0090 Rev 18
1679/1749

--- Page 120 ---

Flexible memory controller (FMC)

RM0090

Bits 2:1 **MODES1[1:0]**: Status Mode for Bank 1
This bit defines the Status Mode of SDRAM Bank 1.
00: Normal Mode
01: Self-refresh mode
10: Power-down mode

Bit 0 **RE**: Refresh error flag
0: No refresh error has been detected
1: A refresh error has been detected
An interrupt is generated if REIE = 1 and RE = 1

