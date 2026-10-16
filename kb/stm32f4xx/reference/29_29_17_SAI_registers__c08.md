> [RM0090 ch.29 §29.17] SAI registers

### 29.17.7 SAI xClear flag register (SAI_xCLRFR) where X is A or B**

**29.17.7 SAI xClear flag register (SAI_xCLRFR) where X is A or B**

Address offset: block A: 0x01C
Address offset: block B: 0x03C
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |  |  |  |  |  |  |
| Reserved | CLFSDET | CCNRDY | Reserved | CWCKCFG | CMUTEDET | COVRUDR |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  | rw |  |  | rw |


Bits 31:7 Reserved, always read as 0.

**Bit 6 CLFSDET**: Clear late frame synchronization detection flag. This bit is write only.
Writing 1 in this bit clears the flag LFSDET in the SAI_xSR register.
It is not used in AC'97.
Reading this bit always returns the value 0.

**Bit 5 CAFSDET**: Clear anticipated frame synchronization detection flag. This bit is write only.
Writing 1 in this bit clears the flag AFSDET in the SAI_xSR register.
It is not used in AC'97.
Reading this bit always returns the value 0.

**Bit 4 CCNRDY**: Clear codec not ready flag. This bit is write only.
Writing 1 in this bit clears the flag CNRDY in the SAI_xSR register.
This bit is used only when the AC'97 audio protocol is selected in the SAI_xCR1 register.
Reading this bit always returns the value 0.

**Bit 3** Reserved, always read as 0.

**Bit 2 CWCKCFG**: Clear wrong clock configuration flag. This bit is write only.
Writing 1 in this bit clears the flag WCKCFG in the SAI_xSR register.
This bit is used only when the audio block is set as master (MODE[1] = 0 in the SAI_ACR1 register) and bit NODIV = 0 in the SAI_xCR1 register.
Reading this bit always returns the value 0.

**Bit 1 CMUTEDET**: Mute detection flag. This bit is write only.
Writing 1 in this bit clears the flag MUTEDET in the SAI_xSR register.
Reading this bit always returns the value 0.

**Bit 0 COVRUDR**: Clear overrun / underrun. This bit is write only.
Writing 1 in this bit clears the flag OVRUDR in the SAI_xSR register.
Reading this bit always returns the value 0.

962/1749
RM0090 Rev 18

--- Page 183 ---

RM0090
Serial audio interface (SAI)

> [RM0090 ch.29 §29.17] SAI registers

### 29.17.8 SAI xData register (SAI_xDR) where x is A or B

### 29.17.8 SAI xData register (SAI_xDR) where x is A or B

Address offset: block A: 0x020
Address offset: block B: 0x040
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |
|---|
| DATA[31:16] |
| rw |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |
| DATA[15:0] |
| rw |


Bits 31:0 **DATA[31:0]**: Data
A write into this register has the effect of loading the FIFO if the FIFO is not full.
A read from this register has to effect of draining-up the FIFO if the FIFO is not empty.

