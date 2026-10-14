## 29.8 Slot configuration

The slot is the basic element in the audio frame. The number of slots in the audio frame is equal to the configured setting of bit NBSLOT[3:0] in the SAI_xSLOTR register +1. The maximum number of slots per audio frame is fixed at 16.

For AC'97 protocol (when bit PRTCFG[1:0] = 10), the number of slots is automatically set to target the protocol specification, and the value of NBSLOT[3:0] is ignored.

ST
RM0090 Rev 18
933/1749

--- Page 154 ---

**Serial audio interface (SAI)**

RM0090

Each slot can be defined as a valid slot, or not, by setting bit SLOTEN[15:0] in the SAI_xSLOTR register. In an audio frame, during the transfer of a non-valid slot, 0 value will be forced on the data line or the SD data line will be released to HI-z (refer to Section 29.12.4) if the audio block is transmitter, or the received value from the end of this slot will be ignored. Consequently, there will be no FIFO access and so no request to read or write the FIFO linked to this inactive slot status.

The slot size is also configurable as shown in the Figure 287. The size of the slots is selected by setting bit SLOTSZ[1:0] in the SAI_xSLOTR register. The size is applied identically for each slot in an audio frame.

**Figure 287. Slot size configuration with FBOFF = 0 in SAI_xSLOTR**

Audio block is transmitter
Slot size = data size
slotx data size
data size
slotx data size 00..00
16-bit
slotx data size 00..00
32-bit

Audio block is receiver
Slot size = data size
slotx data size
data size
slotx data size X, X
16-bit
slotx data size XX..XX
32-bit

X: don't care
MS30033V1

It is possible to choose the position of the first data bit to transfer within the slots, this offset is configured by bit FBOFF[5:0] in the SAI_xSLOTR register. 0 values will be injected in transmitter mode from the beginning of the slot until this offset position is reached. In reception, the bit in the offset phase is ignored. This feature targets the LSB justified protocol (if the offset is equal to the slot size minus the data size).

**Figure 288. First bit offset**

Audio block is transmitter
Slot size = data size
slotx data size
data size
FBOFF
00 data size 00
16-bit
FBOFF = SLOT SZ -DS
slotx 00..00 data size
32-bit

Audio block is receiver
Slot size = data size
slotx data size
data size
FBOFF
XX data size XX
16-bit
FBOFF = SLOT SZ -DS
slotx XX .. XX data size
32-bit

X: don't care
MS30034V1

934/1749
RM0090 Rev 18

--- Page 155 ---

RM0090
Serial audio interface (SAI)

It is mandatory to respect the following conditions in order to avoid bad SAI behavior:
FBOFF ≤(SLOTSZ - DS),
DS ≤SLOTSZ,
NBSLOT x SLOTSZ ≤FRL (frame length),

The number of slots should be even when bit FSDEF in the SAI_xFRCR register is set.
In AC'97 (bit PRTCFG[1:0] = 10), the slot size is automatically set as defined in
Section 29.11.

