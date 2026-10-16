> [RM0090 ch.29 §29.17] SAI registers


## 29.17.3 SAI xFrame configuration register (SAI_XFRCR) where x is A or B

Address offset: Block A: 0x00C
Address offset: Block B: 0x02C
Reset value: 0x0000 0007

*Note:* This register has no meaning in AC'97 audio protocol

| Reserved | FSOFF | FSPOL | FSDEF |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  |  |  |  |  |  |  |  |  |  | rw | r |
| 31 30 29 28 27 26 25 24 23 22 21 20 19 | 18 | 17 | 16 |  |  |  |  |  |  |  |  |  |  |  |
| Res. | FSALL[6:0] | FRL[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 31:19 Reserved, always read as 0.

**Bit 18 FSOFF:** Frame synchronization offset. This bit is set and cleared by software.
0: FS is asserted on the first bit of the slot 0.
1: FS is asserted one bit before the first bit of the slot 0.
This bit has no meaning and is not used in AC'97 audio block configuration.
This bit must be configured when the audio block is disabled.

**Bit 17 FSPOL:** Frame synchronization polarity. This bit is set and cleared by software
0: FS is active low (falling edge)
1: FS is active high (rising edge)
This bit is used to configure the level of the start of frame on the FS signal.
This bit has no meaning and is not used in AC'97 audio block configuration.
This bit must be configured when the audio block is disabled.

**Bit 16 FSDEF:** Frame synchronization definition. This bit is set and cleared by software.
0: FS signal is a start frame signal
1: FS signal is a start of frame signal + channel side identification
When the bit is set, the number of slots defined in the SAI_ASLOTR register has to be even. It means that there will be half of this number of slots dedicated for the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...)
This bit has no meaning and is not used in AC'97 audio block configuration.
This bit must be configured when the audio block is disabled.

ST
RM0090 Rev 18
955/1749

--- Page 176 ---

**Serial audio interface (SAI)**

RM0090

Bit 15 Reserved, always read as 0.

**Bits 14:8 FSALL[6:0]**: Frame synchronization active level length. These bits are set and cleared by software
The value set in these bits specifies the length in number of bit clock (SCK) + 1 (FSALL[6:0] + 1) of
the active level of the FS signal in the audio frame
These bits have no meaning and are not used in AC’97 audio block configuration.
These bits must be configured when the audio block is disabled.

**Bits 7:0 FRL[7:0]**: Frame length. These bits are set and cleared by software.
They define the length of the audio frame. More precisely, these bits define the number of SCK
clocks for each audio frame.
The number of bits in the frame is equal to FRL[7:0] + 1.
The minimum number of bits to transfer in an audio frame has to be equal to 8 or else the audio
block will have unexpected behavior. This is the case when the data size is 8-bit and only one slot 0
is defined in NBSLOT[4:0] in the SAI_ASLTR register (NBSLOT[3:0] = 0000).
In master mode, if the master clock MCLK_x pin is declared as an output, the frame length should
be aligned to a number equal to a power of 2, from 8 to 256 in order to keep in an audio frame, an
integer number of MCLK pulses by bit clock for correct operation for external DAC/ADC inside the
decoders.
The Frame length should be even.
These bits have no meaning and are not used in AC’97 audio block configuration.

956/1749 RM0090 Rev 18

--- Page 177 ---

RM0090
Serial audio interface (SAI)

