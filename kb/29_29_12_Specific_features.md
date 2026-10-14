## 29.12 Specific features

The SAI has some specific functions which can be useful depending on the audio protocol selected. These functions are accessible through specific bits in the SAI_xCR2 register.

### 29.12.1 Mute mode

Mute mode may be used when the audio block is a transmitter or receiver.

**Transmitter**

In transmitter mode, Mute mode can be selected at anytime. Mute mode is active for entire audio frames. The bit MUTE in the SAI_xCR2 register requests Mute mode when it is set during an on-going frame.

The mute mode bit is strobed only at the end of the frame. If set at this time, the mute mode is active at the beginning of the new audio frame, for a complete frame, until the next end of frame, it then strobes the bit to determine if the next frame will still be a mute frame.

If the number of slots set in bit NBSLOT[3:0] in the SAI_xSLOTR register is lower than or equal to two, it is possible to specify if the value sent during the Mute mode is 0 or if it is the last value of each slot. The selection is done via bit MUTEVAL in the SAI_xCR2 register.

If the number of slots set in bit NBSLOT[3:0] in the SAI_xSLOTR register is greater than two, MUTEVAL bit in the SAI_xCR2 has no meaning as 0 values are sent on each bit on each slot.

During Mute mode, the FIFO pointers are still incremented, meaning that data which was present in the FIFO and for which the Mute mode is requested is discarded.

**Receiver**

In receiver mode, it is possible to detect a Mute mode sent from the external transmitter when all the declared and valid slots of the audio frame receive 0 for a given consecutive number of audio frames (bit MUTE CNT[5:0] in the SAI_xCR2 register).

When the number of MUTE frames is detected, flag MUTED ET in the SAI_xSR register is set and an interrupt can be generated if bit MUTED ETIE is set in the SAI_xCR2.

The mute frame counter is cleared when the audio block is disabled or when a valid slot receives at least one data in an audio frame. The interrupt is generated just once, when the counter reaches the specified value in bit MUTE CNT[5:0]. Then the interrupt event is re-armed when the counter is cleared.

### 29.12.2 MONO/STEREO function

In transmission mode, it is possible to address the Mono mode without any data pre-processing in memory when the number of slot is equal to 2 (NBSLOT[3:0] = 0001 in the SAI_xSLOTR). In such a case, the access to and from the FIFO will be reduced by two since in transmission, the data for slot 0 is duplicated into data slot 1.

To select the Mono feature, set bit MONO in the SAI_xCR1 register.

In reception mode, bit MONO can be set and has a meaning only if the number of slots is equal to 2 like for the transmission mode. When it is set, only the data of slot 0 will be stored in the FIFO. The data belonging to slot 1 will be discarded since in this case, it is supposed to be the same as the previous slot. If the data flux in reception is a real stereo audio flow

940/1749
RM0090 Rev 18
ST

--- Page 161 ---

RM0090
Serial audio interface (SAI)

with a distinct and different left and right data, bit MONO has no meaning. The conversion from the output stereo file to the equivalent mono file is done by software.

Note:
To enable Mono mode, NBSLOT and SLOTEN must equal two and MONO bit set to 1.

**29.12.3 Companding mode**

Telecommunication applications may require to process the data to transmit or to receive with a data companding algorithm.

Depending on the COMP[1:0] bit in the SAI_xCR2 register (used only when TDM mode is selected), the software may choose to process or not the data before sending it on SD serial output line (compression) or to expand the data after the reception on SD serial input line (expansion) as illustrated in Figure 291. The two companding modes supported are the μ-Law and the A-Law log which are a part of the CCITT G.711 recommendation.

The companding standard employed in the United States and Japan is the μ-Law and allows 14 bits of dynamic range (COMP[1:0] = 10 in the SAI_xCR2 register).

The European companding standard is A-Law and allows 13 bits of dynamic range (COMP[1:0] = 11 in the SAI_xCR2 register).

Companding standard (μ-Law or A-Law) can be computed based on 1’s complement or 2’s complement representation depending on the CPL bit setting in the SAI_xCR2 register.

The μ-Law and A-Law formats encode data into 8-bit code elements with MSB alignment. Companded data is always 8 bits wide. For this reason, bit DS[2:0] in the SAI_xCR1 register will be forced to 010 when the SAI audio block is enabled (bit SAIxEN = 1 in the SAI_xCR1 register) and when the COMP[1:0] bit selects one of these two companding modes.

If no companding processing is required, COMP[1:0] bit in the SAI_xCR2 register should be kept cleared.

Figure 291. Data companding hardware in an audio block in the SAI

Receiver mode (bit MODE[0] = 1 in SAI_xCR1)
COMP[1]
FIFO
expand
32-bit shift register
SD

Transmitter mode (bit MODE[0] = 0 in SAI_xCR1)
FIFO
compress
32-bit shift register
SD
COMP[1]

MS19244V1

Note:
Not applicable when AC’97 selected.

ST
RM0090 Rev 18
941/1749

--- Page 162 ---

**Serial audio interface (SAI)** RM0090

Expansion or compression mode is automatically selected by the SAI configuration.

*   If the SAI audio block is configured to be a transmitter, and if the COMP[1] bit is set in the SAI_xCR2 register, the compression mode will be applied.
*   If the SAI audio block is declared as a receiver, the expansion algorithm will be applied.

### 29.12.4 Output data line management on an inactive slot

In transmitter mode, it is possible to choose the behavior of the SD line in output when an inactive slot is sent on the data line (via bit TRIS in the SAI_xCR2 register when the SAI is disabled).

*   Either the SAI forces 0 on the SD output line when an inactive slot is transmitted, or
*   The line is released in HI-z state at the end of the last bit of data transferred, to release the line for other transmitters connected to this node.

It is important to note that the two transmitters do not attempt to drive the same SD output pin simultaneously, which could result in a short circuit. In order to ensure a gap between transmissions, if the data is lower than 32-bit, the data can be extended to 32-bit by setting bit SLOTSZ[1:0] = 10 in the SAI_xSLOTR register. Then, the SD output pin will be tristated at the end of the LSB of the active slot (during the padding to 0 phase to extend the data to 32-bit) if the following slot is declared inactive.

In addition, if the number of slots multiplied by the slot size is lower than the frame length, the SD output line will be tristated when the padding to 0 is done to complete the audio frame.

*Figure 292* illustrates these behaviors.

942/1749 RM0090 Rev 18

--- Page 163 ---

RM0090
Serial audio interface (SAI)

**Figure 292. Tristate strategy on SD output line on an inactive slot**

Bit TRIS = 1 in the SAI_xCR1 and frame length = number of slots

Audio frame

sck

Slot size = data size

slot
Slot 0 ON
Slot 1 OFF
Slot 2 OFF
Slot 3 ON
.. ON
.. ON
Slot n ON

SD (output)
Data 0
Data 1
Data m

Slot size > data size

slot
Slot 0 ON
Slot 1 OFF
Slot 2 OFF
Slot 3 ON
.. ON
.. ON
Slot n ON

SD (output)
Data 0
Data 1
Data m

slot
Slot 0 ON
Slot 1 OFF
Slot 2 OFF
Slot 3 ON
.. ON
.. ON
Slot n ON

SD (output)
Data 0
Data m

Bit TRIS = 1 in the SAI_xCR1 and frame length > number of slots

Audio frame

sck

Slot size = data size

slot
Slot 0 ON
Slot 1 OFF
Slot 2 OFF
.. ON
Slot n ON

SD (output)
Data 0
Data m

Slot size > data size

slot
Slot 0 ON
Slot 1 OFF
Slot 2 OFF
.. ON
Slot n ON

SD (output)
Data 0
Data m

slot
Slot 0 ON
Slot 1 OFF
Slot 2 OFF
.. ON
.. ON

SD (output)
Data m

MS192345V1

When the selected audio protocol uses the FS signal as a start of frame and a channel side identification (bit FSDEF = 1 in the SAI_xFRCR register), the tristate mode is managed according to Figure 293 (where bit TRIS in the SAI_xCR1 register = 1, and FSDEF=1, and half frame length > number of slots/2, and NBSLOT=6).

ST
RM0090 Rev 18
943/1749

--- Page 164 ---

Serial audio interface (SAI)
RM0090

Figure 293. Tristate on output data line in a protocol like I2S

sck
slot
Slot size = data size
Slot 0 ON Slot 1 OFF Slot 2 ON Slot 3 ON Slot 4 OFF Slot 5 ON
SD (output)
Data 0 Data 1 Data 2 Data 3
Slot size > data size
slot
Slot 0 ON Slot 1 OFF Slot 2 ON Slot 3 ON Slot 4 OFF Slot 5 ON
SD (output)
Data 0 Data 1 Data 2 Data 3
slot
Slot 0 ON Slot 1 OFF Slot 2 ON Slot 3 ON Slot 4 OFF Slot 5 ON
SD (output)
Data 0 Data 1 Data m

MS192346V1

If the TRIS bit in the SAI_xCR2 register is cleared, all the High impedance states on the SD output line on Figure 292 and Figure 293 are replaced by a drive with a value of 0.

29.13 Error flags

The SAI embeds some error flags:
- FIFO overrun/underrun,
- Anticipated frame synchronization detection,
- Late frame synchronization detection,
- Codec not ready (AC'97 exclusively),
- Wrong clock configuration in master mode.

29.13.1 FIFO overrun/underrun (OVRUDR)

The FIFO Overrun/Underrun bit is called OVRUDR in the SAI_xSR register.

The overrun or underrun errors occupy the same bit since an audio block can be either receiver or transmitter and each audio block in an SAI has its own SAI_xSR register.

Overrun

When the audio block is configured as receiver, an overrun condition may appear if data is received in an audio frame when the FIFO is full and is not able to store the received data. In this case, the received data is lost, the flag OVRUDR in the SAI_xSR register is set and an interrupt is generated if bit OVRUDRIE is set in the SAI_xIM register. The slot number from which the overrun occurs, is stored internally. No more data will be stored into the FIFO until it becomes free to store new data. When the FIFO has at least one data free, the SAI audio block receiver will store new data (from new audio frame) from the slot number which

944/1749
RM0090 Rev 18
ST

--- Page 165 ---

RM0090
Serial audio interface (SAI)

was stored internally when the overrun condition was detected, and this, to avoid data slot de-alignment in the destination memory (refer to Figure 294).

The OVRUDR flag is cleared when bit COVRUDR is set in the SAI_xCLRFR register.

**Figure 294. Overrun detection error**

Example: FIFO overrun on Slot 1
Audio frame Audio frame
sck
data Slot 0 ON Slot 1 ON Slot 1 ON Slot 0 ON Slot 1 ON ON Slot n ON
FIFO full
Received data discarded Data stored again in FIFO
OVRUDR
COVRUDR = 1
MS192348V2

**Underrun**

An underrun may occur when the audio block in the SAI is a transmitter and the FIFO is empty when data needs to be transmitted (the audio block configuration (Master or Slave) is not relevant). If an underrun is detected, the software must resynchronize data and slot. Proceed as follows:

1. Disable the SAI peripheral by resetting the SAIEN bit of the SAI_xCR1 register. Check that the SAI has been disabled by reading back the SAIEN bit (SAIEN should be equal to 0).
2. Flush the Tx FIFO through the FFLUS bit of the SAI_xCR2 register.
3. Re-assigned to the correct data to be transferred on the first active slot of the new frame.
4. Re-enabling the SAI peripheral (SAIEN bit set to 1).

The underrun event sets the OVRUDR flag in the SAI_xSR register and an interrupt is generated if the OVRUDRIE bit is set in the SAI_xIM register. To clear this flag, set the COVRUDR bit in the SAI_xCLRFR register.

ST
RM0090 Rev 18
945/1749

--- Page 166 ---

Serial audio interface (SAI)
RM0090

Figure 295. FIFO underrun event

Example: FIFO underrun on Slot 1
Audio frame
Audio frame
sck
data
SD (output)
Slot size = data size
Slot 0 ON
MUTE
MUTE
MUTE
Slot 1 ON
...
ON
Slot 0 ON
FIFO empty
OVRUND
OVRUND=1
MS192347V2

**29.13.2 Anticipated frame synchronisation detection (AFSDET)**

This flag AFSDET is used only in Slave mode. In master mode, it is never asserted. It informs about the detection of a frame synchronisation (FS) earlier than expected since the frame length, the frame polarity, the frame offset are defined and known.

Early detection sets flag AFSDET in the SAI_xSR register.

This detection has no effect on the current audio frame which is not sensitive to the anticipated FS. This means that “parasitic” events on signal FS are flagged without any perturbation of the current audio frame.

If bit AFSDETIE is set in the SAI_xIM register, an interrupt is generated. To clear the flag AFSDET, bit CAFSDDET in the SAI_xCLRFR register has to be set.

To resynchronize with the master after Anticipated frame detection error, four steps should be respected:

1. SAI block should be disabled by resetting SAIEN bit in SAI_xCR1 register, to be sure that the SAI is disabled SAIEN bit is should be equal to 0 (reading back this bit).
2. FIFO should be flushed via FFLUS bit in SAI_xCR2 register.
3. Re-enabling the SAI peripheral (SAIEN bit set to 1) then the SAI.
4. SAI block will wait for the assertion on FS to restart the synchronization with master.

*Note:* This flag is not asserted in AC’97 since the SAI audio block acts as a link controller and generates the FS signal even when declared as slave.

**29.13.3 Late frame synchronization detection**

Flag LFSDET in the SAI_xSR register can be set only when the SAI audio block is defined as slave. The frame length, the frame polarity and the frame offset configuration are known in register SAI_xFRCR.

If the external master does not send the FS signal at the expecting time (generating the signal too late), the flag LFSDET in the SAI_xSR register will be set and an interrupt is generated if bit LFSDETIE in the SAI_xIM register is set.

The flag is cleared when bit CLFSDET is set in the SAI_xCLRFR register.

946/1749
RM0090 Rev 18
STI

--- Page 167 ---

RM0090
Serial audio interface (SAI)

The late frame synchronisation detection flag is set when the error is detected, SAI needs to be resynchronized with the master (the four steps described above should be respected).

This detection and flag assertion can detect glitches on the SCK clock in a noisy environment, detected by the state machine of the audio block. It could incorrectly shift the SAI audio block state machine from one state in the current audio frame, thus corrupting the frame.

There is no corruption if the external master is not managing the audio data frame transfer in a continuous mode, which should not be the case for most application purposes. In this case, flag LFSDET will be set.

Note: This flag is not asserted in AC'97 mode since the SAI audio block acts as a link controller and generates the FS signal even when declared as slave.

### 29.13.4 Codec not ready (CNRDY AC'97)

The flag CNRDY in the SAI_xSR register is relevant only if the SAI audio block is configured to work in AC'97 mode (bit PRTCFG[1:0] = 10 in the SAI_xCR1 register). If bit CNRDYIE is set in the SAI_xIM register, an interrupt will be generated when the flag CNRDY is set.

It is asserted when the codec is not ready to communicate during the reception of the TAG 0 (slot0) of the AC'97 audio frame. In this case, there will be no data automatically stored into the FIFO since the codec is not ready, until the TAG 0 indicates that the codec is ready. All the active slots defined in the SAI_xSLOTR register will be captured when the codec is ready.

To clear the flag, bit CCNRDY in the SAI_xCLRF register has to be set.

### 29.13.5 Wrong clock configuration in master mode (with NODIV = 0)

When the audio block is master (MODE[1] = 0 in the SAI_xCR1 register) and if bit NODIV in the SAI_xCR1 is clear, the flag WCKCFG will be set if bit FRL[7:0] in the SAI_xFRCR is not set with a proper value when the SAIxEN bit in the SAI_xCR1 register is set, in order to respect this following rule:

(FRL[7,0]) + 1 = 2^n

where n is in the range from 3 to 8.

If bit WCKCFGIE is set, an interrupt is generated when flag WCKCFG is set in the SAI_xSR register. To clear the flag, set bit CWCKCFG bit in the SAI_xCLRF register.

When bit WCKCFG is set, the audio block is automatically disabled, clearing bit SAIxEN in the SAI_xCR1 register via hardware.

The above formula is intended to guarantee that the number of MCLK pulses by bit clock is an even integer in the audio frame with a 50% duty cycle bit clock generation to guarantee the good quality of the audio sounds or acquisitions.

ST
RM0090 Rev 18
947/1749

--- Page 168 ---

Serial audio interface (SAI)
RM0090

## 29.14 Interrupt sources

The SAI has 7 possible interrupt sources as illustrated by Table .

Table 131. Interrupt sources

| Interrupt source | Interrupt group | Audio block mode | Interrupt enable | Interrupt clear |
|---|---|---|---|---|
| FREQ | Master or Slave Receiver or transmitter | FREQIE in SAI_xIM register | Depend on: - FIFO threshold setting (FLTH bits in SAI_CR2) - Communication direction transmitter or receiver for more details please refer to Internal FIFOs section |  |
| OVRUDR | ERROR | Master or Slave Receiver or transmitter | OVRUDRIE in SAI_xIM register | COVRUDR = 1 in SAI_xCLRFR register |
| AFSDET | ERROR | Slave (Not used in AC'97 mode) | AFSDETIE in SAI_xIM register | CAFSDET = 1 in SAI_xCLRFR register |
| LFSDET | ERROR | Slave (Not used in AC'97 mode) | LFSDETIE in SAI_xIM register | CLFSDET = 1 in SAI_xCLRFR register |
| CNRDY | ERROR | Slave (Only in AC'97 mode) | CNRDYIE in SAI_xIM register | CCNRDY = 1 in SAI_xCLRFR register |
| MUTEDET | MUTE | Master or slave Receiver mode only | MUTEDETIE in SAI_xIM register | CMUTEDET = 1 in SAI_xCLRFR register |
| WCKCFG | ERROR | Master with NODIV = 0 in the SAI_xCR1 register | WCKCFGIE in SAI_xIM register | CWCKCFG = 1 in SAI_xCLRFR register |


Below are the SAI configuration steps to follow when an interrupt occurs:
1. Disable SAI interrupt.
2. Configure SAI.
3. Configure SAI interrupt source.
4. Enable SAI.

## 29.15 Disabling the SAI

The audio block in the SAI can be disabled at any moment by clearing bit SAIxEN in the SAI_xCR1 register. All the frames that have already started will be automatically completed before the total extinction of the SAI. Bit SAIxEN in the SAI_xCR1 register will stay high until the SAI is completely switched-off at the end of the current audio frame transfer.

948/1749
RM0090 Rev 18
STI

--- Page 169 ---

RM0090
Serial audio interface (SAI)

If there is an audio block in the SAI synchronous with the other one, the one which is the master must be disabled first.

## 29.16 SAI DMA interface

In order to free the CPU and to optimize the bus bandwidth, each SAI audio block has an independent DMA interface in order to read or to write into the SAI_xDR register (to hit the internal FIFO). There is one DMA channel per audio block following basic DMA request/acknowledge protocol.

To configure the audio block to transfer through the DMA interface, set bit DMAEN in the SAI_xCR1 register. The DMA request is managed directly by the FIFO controller depend of FIFO threshold level (for more details please refer to Internal FIFOs section). DMA direction is linked to the SAI audio block configuration:

*   If the audio block is a transmitter, the audio block's FIFO controller outputs a DMA request to load the FIFO with data written in the SAI_xDR register.
*   If the audio block is a receiver, the DMA request will concern read operations from the SAI_xDR register.

Below are the SAI configuration steps followed when DMA is used:

1.  Configure SAI and FIFO Threshold level (in order to specify when the DMA request to be launched)
2.  Configure SAI DMA channel
3.  Enable DMA
4.  Enable SAI

*Note:* Before configuring the SAI block, the SAI DMA channel must be disabled.

ST
RM0090 Rev 18
949/1749

--- Page 170 ---

**Serial audio interface (SAI)**
RM0090

