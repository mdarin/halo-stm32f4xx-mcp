> [RM0090 ch.29 §29.17] SAI registers

### 29.17.5 SAI xInterrupt mask register (SAI_xIM) where x is A or B**

**29.17.5 SAI xInterrupt mask register (SAI_xIM) where x is A or B**

Address offset: blockA: 0x014
Address offset: block B: 0x034
Reset value: 0x0000 0000

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |
| Reserved | LFSDETIE | AFSDETIE | CNRDYIE | FREQIE | WCKCFGIE | MUTEDTIE | OVRUDRIE |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  | rw |


Bits 31:7 Reserved, always read as 0.

**Bit 6 LFSDETIE**: Late frame synchronization detection interrupt enable. This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt will be generated if the LFSDET bit is set in the SAI_ASR register. This bit has no meaning in AC'97 mode. It has no meaning also if the audio block is master.

**Bit 5 AFSDETIE**: Anticipated frame synchronization detection interrupt enable. This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt will be generated if the AFSDET bit in the SAI_ASR register is set. This bit has no meaning in AC'97 mode. It has no meaning also if the audio block is master.

**Bit 4 CNRDYIE**: Codec not ready interrupt enable (ac'97). This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When the interrupt is enabled, the audio block will detect in the slot 0 (tag0) of the AC'97 frame if the codec connected on this line is ready or not. If not, the flag CNRDY in the SAI_ASR register will be set and an interruption will be generated.
This bit has a meaning only if the AC97 mode is selected (bit PRTCFG[1:0]) and the audio block is a receiver.

**Bit 3 FREQIE**: FIFO request interrupt enable. This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt will be generated if the FREQ bit in the SAI_ASR register is set.
In receiver mode, the bit MODE must be configured before setting bit FREQIE to avoid a parasitic interruption since the audio block is a transmitter (default setting).

958/1749
RM0090 Rev 18
ST

--- Page 179 ---

RM0090
Serial audio interface (SAI)

**Bit 2 WCKCFGIE**: Wrong clock configuration interrupt enable. This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
This bit is considered only if the audio block is configured as master (MODE[1] = 0 in the SAI_ACR1 register) and bit NODIV = 0 in the SAI_xCR1 register.
It generates an interrupt if the flag WCKCFG in the SAI_ASR register is set.
*Note: This bit is used only in TDM mode and has no meaning for other modes.*

**Bit 1 MUTEDETIE**: Mute detection interrupt enable. This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt will be generated if the MUTEDET bit in the SAI_ASR register is set.
This bit has a meaning only if the audio block is configured in receiver mode.

**Bit 0 OVRUDRIE**: Overrun/underrun interrupt enable. This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt will be generated if the OVRUDR bit in the SAI_ASR register is set.

ST
RM0090 Rev 18
959/1749

--- Page 180 ---

**Serial audio interface (SAI)**
RM0090

