## 29.7 Frame synchronization

The FS signal acts as the Frame synchronization signal in the audio frame (start of frame). The shape of this signal is completely configurable in order to target the different audio protocols with their own specificities concerning this Frame synchronization behavior. This configurability is done using register SAI_xFRCR. Figure 284 gives a view of this flexibility.

**Figure 284. Audio frame**

Fs length = < 256 bits
Fs active length = < 128 bits
Fs offset
Fs
sck
sd
Slot 0 Slot 1 Slot 2 Slot 3 ... Slot n

MS30037V1

930/1749
RM0090 Rev 18

--- Page 151 ---

RM0090
Serial audio interface (SAI)

In AC'97 mode (bit PRTCFG[1:0] = 10 in the SAI_xCR1 register), the frame synchronization shape is forced to be configured to target these protocols. The SAI_xFRCR register value is ignored.

Each audio block is independent and so each requires a specific configuration.

### 29.7.1 Frame length

*   Master mode: The audio frame length can be configured up to 256 bit clock, setting bit FRL[7:0] in the SAI_xFRCR register. If the frame length is greater than the number of declared slots for the frame, the remaining bits to transmit will be extended to 0 or the SD line will be released to HI-z depending the state of bit TRIS in the SAI_xCR2 register (refer to Section 29.12.4). In reception mode, the remaining bit is ignored.
*   Slave mode: The audio frame length is mainly used in order to specify to the slave the number of bit clocks per audio frame sent by the external master. It is used mainly to detect from the master, any anticipated or late occurrence of the Frame synchronization signal during an on-going audio frame. An error will be generated in such case. For more details please refer to the Section 29.13.

The number of bits in the frame is equal to FRL[7:0] + 1.

The minimum number of bits to transfer in an audio frame is 8. This is the case when the data size is 8-bit and only one slot is defined in NBSLOT[3:0] in the SAI_xSLOTR register (NBSLOT[3:0] = 0000 for slot 0).

In master mode:

*   If bit NODIV in the SAI_xCR1 register is cleared, the frame length should be aligned to a number equal to a power of 2, from 8 to 256. This is to ensure that an audio frame contains an integer number of MCLK pulses per bit clock, which ensures correct operation of the external DAC/ADC inside the decoders. If the value set in FRL[7:0] does not respect this rule, flag WCKCFG is set when the audio block is enabled and an interrupt is generated if bit WCKCFGIE is set in the SAI_xIM register. The SAI is automatically disabled.
*   If bit NODIV in the SAI_xCR1 register is set, the FRL[7:0] bit can take any of the values without constraint since the input clock of the audio block should be equal to the bit clock. There is no MCLK_x clock which can be output. MCLK_x output pad is automatically disabled.

In slave mode, there are no constraints for the FRL[7:0] bit configuration in the SAI_xFRCR register.

### 29.7.2 Frame synchronization polarity

Bit FSPOL in the SAI_xFRCR register sets the active polarity of the FS pin from which a frame is started. The start of frame is edge sensitive.

In slave mode, the audio block waits for a valid frame to start to transmit or to receive. Start of frame is synchronized to this signal. It is effective only if the start of frame is not detected during an on-going communication and assimilated to an anticipated start of frame (refer to Section 29.13).

In master mode, the frame synchronization is sent continuously each time an audio frame is complete until the SAIxEN bit in the SAI_xCR1 register is cleared. If no data is present in the FIFO at the end of the previous audio frame, an underrun condition will be managed as

ST
RM0090 Rev 18
931/1749

--- Page 152 ---

**Serial audio interface (SAI)** RM0090

described in Section 29.13), but there will be no interruption in the audio communication flow.

### 29.7.3 Frame synchronization active level length

Bit FSALL[6:0] in the SAI_xFRCR register configures the length of the active level of the Frame synchronization signal. The length can be set from 1 to 128 bit clock SCK.

The active length may be half of the frame length in I2S, LSB or MSB-justified modes for instance, or one-bit wide for PCM/DSP or TDM mode, or even 16-bit length in AC'97.

### 29.7.4 Frame synchronization offset

Depending on the audio protocol targeted in the application, the Frame synchronization signal can be asserted when transmitting the last bit or the first bit of the audio frame (as for instance, respectively, in I2S standard protocol and in MSB-justified protocol). Bit FSOFF in the SAI_xFRCR register makes the choice.

### 29.7.5 FS signal role

The FS signal may have a different meaning depending on the FS function. Bit FSDEF in the SAI_xFRCR register selects which meaning it will have. It may be either:

*   0: a start of frame, like for instance the PCM/DSP, TDM, AC'97, audio protocols,
*   1: a start of frame and a channel side identification within the audio frame like for the I2S, the MSB or LSB-justified protocols.

When the FS signal is considered as a start of frame and channel side identification within the frame, the number of declared slots must be considered to be half of the number for the left channel and half of the number for the right channel. If the number of bit clock on half audio frame is greater than the number of slots dedicated to a channel side, if TRIS = 0, 0 is sent for transmission for the remaining bit clock in the SAI_xCR2 register, otherwise if TRIS = 1, the SD line is released to HI-Z. In reception, the remaining bit clock are not considered until the channel side changes.

932/1749 RM0090 Rev 18 ST

--- Page 153 ---

RM0090
Serial audio interface (SAI)

**Figure 285. FS role is start of frame + channel side identification (FSDEF = TRIS = 1)**

Number of slots not aligned with the audio frame
Audio frame
Half of frame
FS
sck
slot
Slot 0 ON
Slot 1 OFF
Slot 2 ON
Slot 3 ON
Slot 4 OFF
Slot 5 ON

Number of slots aligned with the audio frame
Audio frame
Half of frame
FS
sck
slot
Slot 0
Slot 1
Slot 2
Slot 3
Slot 4
Slot 5

MS30038V1

1. The frame length should be even.
If bit FSDEF in SAI_xFRCR is kept clear, so FS signal is equivalent to a start of frame, and if the number of slots defined in bit NBSLOT[3:0] in SAI_xSLOTR multiplied by the number of bits by slot configured in bit SLOTSZ[1:0] in SAI_xSLOTR is less than the frame size (bit FRL[7:0] in the SAI_xFRCR register), then,
* if TRIS = 0 in the SAI_xCR2 register, the remaining bit after the last slot will be forced to 0 until the end of frame in case of transmission,
* if TRIS = 1, the line will be released to HI-Z during the transfer of these remaining bits. In reception mode, these bits are discarded.

**Figure 286. FS role is start of frame (FSDEF = 0)**

Audio frame
sck
slot
Slot 0
Slot 1
Slot 2
Slot n

Data = 0 after slot n if TRIS = 0
SD output released (HI-Z) after slot n if TRIS = 1

MS30039V1

