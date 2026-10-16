> [RM0090 ch.29 §29.17] SAI registers


### 29.17.1 SAI xConfiguration register 1 (SAI_xCR1) where x is A or B

Address offset: Block A: 0x004
Address offset: Block B: 0x024
Reset value: 0x0000 0040

| 31 30 29 28 27 26 25 24 | 23 22 21 20 | 19 | 18 | 17 | 16 |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | MCKDIV[3:0] | NODIV | Res. | DMAEN | SAIxEN |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |  | rw |
| 15 14 13 12 11 10 9 8 | 7 6 5 | 4 | 3 | 2 1 | 0 |  |  |  |  |  |
| Reserved | OutDri v | MONO | SYNCEN[1:0] | CKSTR | LSBFIR ST | DS[2:0] | Res. | PRTCFG[1:0] | MODE[1:0] |  |
|  |  |  |  |  |  |  |  | rw |  |  |


Bits 31:24 Reserved, always read as 0.

Bit 23:20 **MCKDIV[3:0]**: Master clock divider. These bits are set and cleared by software.
0000: Divides by 1 the master clock input.
Otherwise, The Master clock frequency is calculated accordingly to the following formula:
MCLK_x = SAI_CK_x / (MCKDIV[3:0] * 2)
These bits have no meaning when the audio block is slave.
They have to be configured when the audio block is disabled.

Bit 19 **NODIV**: No divider. This bit is set and cleared by software.
0: Master Clock divider is enabled
1: No divider used in the clock generator (in this case Master Clock Divider bit has no effect)

Bit 18 Reserved, always read as 0.

Bit 17 **DMAEN**: DMA enable. This bit is set and cleared by software.
0: DMA is disabled
1: DMA is enabled
*Note: In receiver mode, the bits MODE must be configured before setting bit DMAEN to avoid a DMA request since the audio block is transmitter after reset (default setting)*

Bit 16 **SAIxEN**: Audio block enable where x is A or B. This bit is set by software. It is cleared by hardware, after disabling it by software (writing the bit low), the audio is completely disabled (waiting for the end of the current frame).
0: Audio block is disabled
1: Audio block is enabled: this bit can be set only if it is at 0 during the write operation (means the SAI is completely disabled before being re-enabled).
This bit allows to control the state of the audio block. If it is disabled somewhere in an audio frame, the on-going transfer will be completed and the cell will be totally disabled at the end of this audio frame transfer.
*Note: When SAIx block is configured as master mode, clock must be present on the input of the SAI before setting SAIxEN bit.*

Bits 15:14 Reserved, always read as 0.

Bit 13 **OUTDRIV**: Output drive. This bit is set and cleared by software.
0: Audio block output driven when SAIEN is set
1: Audio block output driven immediately after the setting of this bit.
*Note: This bit has to be set before enabling the audio block but after the audio block configuration.*

950/1749
RM0090 Rev 18

--- Page 171 ---

RM0090
Serial audio interface (SAI)

**Bit 12 MONO**: Mono mode. This bit is set and cleared by software.
0: Stereo mode
1: Mono mode.
This bit has a meaning only when the number of slots is equal to 2.
When the Mono mode is selected, the data of the slot 0 data is duplicated on the slot 1 when the audio block is a transmitter. In reception mode, the slot1 is discarded and only the data received from the slot 0 will be stored.
Refer to Section 29.12.2 for more details.

**Bits 11:10 SYNCEN[1:0]**: Synchronization enable. This bit is set and cleared by software.
00: audio block is asynchronous.
01: audio block is synchronous with the other internal audio block. In this case audio block should be configured in Slave mode
10: Reserved.
11: Not used
These bits have to be configured when the audio block is disabled.

**Bit 9 CKSTR**: Clock strobing edge. This bit is set and cleared by software.
0: data strobing edge is falling edge of SCK
1: data strobing edge is rising edge of SCK
This bit has to be configured when the audio block is disabled.

**Bit 8 LSBFIRST**: Least significant bit first. This bit is set and cleared by software.
0: data is transferred with the MSB of the data first
1: data is transferred with the LSB of the data first
This bit has to be configured when the audio block is disabled.
This bit has no meaning in AC'97 audio protocol since in AC'97 data is transferred with the MSB of the data first.

**Bits 7:5 DS[2:0]**: Data size. These bits are set and cleared by software.
000: Not used
001: Not used
010: 8-bit
011: 10-bit
100: 16-bit
101: 20-bit
110: 24-bit
111: 32-bit
When the companding mode is selected (bit COMP[1:0]), these DS[1:0] are ignored since the data size is fixed to 8-bit mode by the algorithm itself.
These bits must be configured when the audio block is disabled.
*Note: When AC'97 mode is selected the data sizes that can be used are: 16-bit or 20-bit only, else SAI behavior is not guaranteed.*

STI
RM0090 Rev 18
951/1749

--- Page 172 ---

**Serial audio interface (SAI)**
RM0090

Bit 4 Reserved, always read as 0.

Bits 3:2 **PRTCFG[1:0]**: Protocol configuration. These bits are set and cleared by software.
00: Free protocol
01: Not used
10: AC’97 protocol
11: Not used
Free protocol selection allows to use the powerful configuration of the audio block to address a specific audio protocol (like I2S, LSB/MSB justified, TDM, PCM/DSP...) setting most of the configuration register bits as well as frame configuration register.
These bits have to be configured when the audio block is disabled.

Bits 1:0 **MODE[1:0]**: Audio block mode. These bits are set and cleared by software.
00: Master transmitter
01: Master receiver
10: Slave transmitter
11: Slave receiver
These bits have to be configured when the audio block is disabled.
*Note: In Master transmitter mode the audio block will start to generate the FS and clocks*

952/1749
RM0090 Rev 18

--- Page 173 ---

RM0090
Serial audio interface (SAI)

