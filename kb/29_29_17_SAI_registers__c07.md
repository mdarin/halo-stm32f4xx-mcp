> [RM0090 ch.29 §29.17] SAI registers


## 29.17.6 SAI xStatus register (SAI_xSR) where x is A or B

Address offset: block A: 0x018
Address offset: block B: 0x038
Reset value: 0x0000 0008

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | FLTH |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  | r |
| 15 14 13 12 11 10 9 8 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |
| Reserved | LFSDET | AFSDET | CNRDY | FREQ | WCKCFG | MUTDET | OVRUDR |  |  |  |  |  |  |
|  |  |  |  |  |  | r |  |  |  |  |  |  |  |


Bits 31:19 Reserved, always read as 0.

Bits 18:16 **FLTH**: FIFO level threshold. This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting depends on SAI block configuration (transmitter or receiver mode).

If SAI block is configured as transmitter:
000: FIFO_empty
001: FIFO <= ¼ but not empty
010: ¼ < FIFO <= ½
011: ½ < FIFO <= ¾
100: ¾ < FIFO but not full
101: FIFO full

If SAI block is configured as receiver:
000: FIFO_empty
001: FIFO < ¼ but not empty
010: ¼ <= FIFO < ½
011: ½ <= FIFO < ¾
100: ¾ <= FIFO but not full
101: FIFO full

Bits 15:7 Reserved, always read as 0.

Bit 6 **LFSDET**: Late frame synchronization detection. This bit is read only.
0: No error.
1: Frame synchronization signal is not present at the right time.
This flag can be set only if the audio block is configured in Slave mode.
It is not used in AC'97 mode.
It may generate an interrupt if bit LFSDETIE in the SAI_xIM register is set.
This flag is cleared when the software sets bit CLFSDET in the SAI_xCLRFR register

Bit 5 **AFSDET**: Anticipated frame synchronization detection. This bit is read only.
0: No error.
1: Frame synchronization signal is detected earlier than expected.
This flag can be set only if the audio block is configured in Slave mode.
It is not used in AC'97.
It may generate an interrupt if bit AFSDETIE in the SAI_xIM register is set.
This flag is cleared when the software sets bit CAFSDET in the SAI_xCLRFR register

960/1749
RM0090 Rev 18

--- Page 181 ---

RM0090
Serial audio interface (SAI)

Bit 4 **CNRDY**: Codec not ready. This bit is read only.
0: The external AC'97 codec is ready
1: The external AC'97 codec is not ready
This bit is used only when the AC'97 audio protocol is selected in the SAI_xCR1 register and is configured in receiver mode.
It may generate an interrupt if bit CNRDYIE in the SAI_xIM register is set.
This flag is cleared when the software sets bit CCNRDY in the SAI_xCLRFR register

Bit 3 **FREQ**: FIFO request. This bit is read only.
0: No FIFO request.
1: FIFO request to read or to write the SAI_xDR.
The request depends on the audio block configuration.
If configured in transmission, the FIFO request concerns a write request operation in the SAI_xDR.
If configured in reception, the FIFO request concerns a read request operation from the SAI_xDR.
This flag can generate an interrupt if bit FREQIE in the SAI_xIM register is set.

Bit 2 **WCKCFG**: Wrong clock configuration flag. This bit is read only.
0: The clock configuration is correct
1: The clock configuration does not respect the rule concerning the frame length specification defined in Section 29.7 (configuration of FRL[7:0] bit in the SAI_x FRCR register)
This bit is used only when the audio block is master (MODE[1] = 0 in the SAI_xCR1 register) and when NODIV = 0 in the SAI_xCR1 register.
It may generate an interrupt if bit WCKCFGIE in the SAI_xIM register is set.
This flag is cleared when the software sets bit CWCKCFG in the SAI_xCLRFR register

Bit 1 **MUTEDET**: Mute detection. This bit is read only.
0: No MUTE detection on the SD input line
1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
This flag is set if consecutive 0 values are received in each slot of an audio frame and for a consecutive number of audio frames (set in the MUTE CNT bit in the SAI_xCR2 register).
It may generate an interrupt if bit MUTEDETIE in the SAI_xIM register is set.
This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register.

Bit 0 **OVRUDR**: Overrun / underrun. This bit is read only.
0: No overrun/underrun error.
1: Overrun/underrun error detection.
The overrun condition can occur only when the audio block is configured in reception.
The underrun condition can occur only when the audio block is configured in transmission.
It may generate an interrupt if bit OVRUDRIE in the SAI_xIM register is set.
This flag is cleared when the software set bit COVRUDR bit in the SAI_xCLRFR register.

ST
RM0090 Rev 18
961/1749

--- Page 182 ---

**Serial audio interface (SAI)**
RM0090

