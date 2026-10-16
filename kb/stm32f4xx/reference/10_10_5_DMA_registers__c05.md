> [RM0090 ch.10 §10.5] DMA registers


### 10.5.10 DMA stream x FIFO control register (DMA_SxFCR) (x = 0..7)

Address offset: 0x24 + 0x24 × stream number
Reset value: 0x0000 0021

| Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 |
| Reserved | FEIE | Reserved | FS[2:0] | DMDIS | FTH[1:0] |  |  |  |  |  |  |  |  |  |  |
| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|  |  |  |  |  |  |  |  | rw |  | r | rw |  |  |  |  |


Bits 31:8 Reserved, must be kept at reset value.
Bit 7 **FEIE**: FIFO error interrupt enable
This bit is set and cleared by software.
0: FE interrupt disabled
1: FE interrupt enabled
Bit 6 Reserved, must be kept at reset value.

STI
RM0090 Rev 18
333/1749

--- Page 139 ---

**DMA controller (DMA)**

RM0090

**Bits 5:3 FS[2:0]: FIFO status**

These bits are read-only.
000: 0 < fifo_level < 1/4
001: 1/4 ≤ fifo_level < 1/2
010: 1/2 ≤ fifo_level < 3/4
011: 3/4 ≤ fifo_level < full
100: FIFO is empty
101: FIFO is full
others: no meaning
These bits are not relevant in the direct mode (DMDIS bit is zero).

**Bit 2 DMDIS: Direct mode disable**

This bit is set and cleared by software. It can be set by hardware.
0: Direct mode enabled
1: Direct mode disabled
This bit is protected and can be written only if EN is '0'.
This bit is set by hardware if the memory-to-memory mode is selected (DIR bit in DMA_SxCR are “10”) and the EN bit in the DMA_SxCR register is ‘1’ because the direct mode is not allowed in the memory-to-memory configuration.

**Bits 1:0 FTH[1:0]: FIFO threshold selection**

These bits are set and cleared by software.
00: 1/4 full FIFO
01: 1/2 full FIFO
10: 3/4 full FIFO
11: full FIFO
These bits are not used in the direct mode when the DMIS value is zero.
These bits are protected and can be written only if EN is '0'.

334/1749 RM0090 Rev 18

--- Page 140 ---

RM0090
DMA controller (DMA)

10.5.11
DMA register map

Table 51 summarizes the DMA registers.

Table 51. DMA register map and reset values

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x0000 | DMA_LISR | Reserved | TCIF3 | HTIF3 | TEIF3 | DMEIF3 | FEIF3 | TCIF2 | HTIF2 | TEIF2 | DMEIF2 | FEIF2 | Reserved | TCIF1 | HTIF1 | TEIF1 | DMEIF1 | FEIF1 | TCIF0 | HTIF0 | TEIF0 | DMEIF0 | FEIF0 | Reserved | 0 |  |  |  |  |  |  |  |  |
| Reset value | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0004 | DMA_HISR | Reserved | TCIF7 | HTIF7 | TEIF7 | DMEIF7 | FEIF7 | TCIF6 | HTIF6 | TEIF6 | DMEIF6 | FEIF6 | Reserved | TCIF5 | HTIF5 | TEIF5 | DMEIF5 | FEIF5 | TCIF4 | HTIF4 | TEIF4 | DMEIF4 | FEIF4 | Reserved | 0 |  |  |  |  |  |  |  |  |
| Reset value | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0008 | DMA_LIFCR | Reserved | CTCIF3 | CHTIF3 | CTEIF3 | CDMEIF3 | CFEIF3 | CTCIF2 | CHTIF2 | CTEIF2 | CDMEIF2 | CFEIF2 | Reserved | CTCIF1 | CHTIF1 | CTEIF1 | CDMEIF1 | CFEIF1 | CTCIF0 | CHTIF0 | CTEIF0 | CDMEIF0 | CFEIF0 | Reserved | 0 |  |  |  |  |  |  |  |  |
| Reset value | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x000C | DMA_HIFCR | Reserved | CTCIF7 | CHTIF7 | CTEIF7 | CDMEIF7 | CFEIF7 | CTCIF6 | CHTIF6 | CTEIF6 | CDMEIF6 | CFEIF6 | Reserved | CTCIF5 | CHTIF5 | CTEIF5 | CDMEIF5 | CFEIF5 | CTCIF4 | CHTIF4 | CTEIF4 | CDMEIF4 | CFEIF4 | Reserved | 0 |  |  |  |  |  |  |  |  |
| Reset value | 0 | Reserved | 0 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0010 | DMA_S0CR | Reserved | CHSEL[2:0] | MBURST[1:0] | PBURST[1:0] | Reserved | CT | DBM | PL[1:0] | PINCOS | MSIZE[1:0] | PSIZE[1:0] | MINC | PINC | CIRC | DIR[1:0] | PFCTRL | TCIE | HTIE | TEIE | DMEIE | EN |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | Offset | Register | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0030 | DMA_S1PAR | PA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0034 | DMA_S1M0AR | MOA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0038 | DMA_S1M1AR | M1A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x003C | DMA_S1FCR | 0 | FEIE | 0 | Reserved | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0040 | DMA_S2CR | Reserved | CHSEL [2:0] | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0044 | DMA_S2NDTR | Reset value | NDT[15:J] | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0048 | DMA_S2PAR | PA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x004C | DMA_S2M0AR | MOA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0050 | DMA_S2M1AR | M1A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0054 | DMA_S2FCR | 0 | FEIE | 0 | Reserved | 0 | 1 | 0 | 1 | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0058 | DMA_S3CR | Reserved | CHSEL[2:0] | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x005C | DMA_S3NDTR | Reset value | NDT[15:J] | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0060 | DMA_S3PAR | PA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0064 | DMA_S3M0AR | MOA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 142 ---

RM0090
DMA controller (DMA)
Table 51. DMA register map and reset values (continued)
337/1749
RM0090 Rev 18

| Offset | Register | M1A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0068 | DMA_S3M1AR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x006C | DMA_S3CR | FS[2:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 1 0 0 | 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DWDS FTH [1:0] | FEIE | 0 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0074 | DMA_S4NTR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0078 | DMA_S4PAR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x007C | DMA_S4M0AR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0080 | DMA_S4M1AR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0084 | DMA_S4CR | FS[2:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 1 0 0 | 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DWDS FTH [1:0] | FEIE | 0 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0088 | DMA_S5CR | CHSEL[2:0] | MBURST[1:0] | PBURST[1:0] | Reserved | CT | DBM | PL[1:0] | PINCOS | MSIZE[1:0] | PSIZE[1:0] | PINC | MINC | CIRC | DIR[1:0] | PFCRTL | TCIE | HTIE | TEIE | DMEIE | EN |
|  |  | 0 | 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x008C | DMA_S5NTR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0090 | DMA_S5PAR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0094 | DMA_S5M0AR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0098 | DMA_S5M1AR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x009C | DMA_S5CR | FS[2:0] | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | 1 0 0 | 0 | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | DWDS FTH [1:0] | FEIE | 0 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00A0 | DMA_S6CR | CHSEL[2:0] | MBURST[1:0] | PBURST[1:0] | Reserved | CT | DBM | PL[1:0] | PINCOS | MSIZE[1:0] | PSIZE[1:0] | PINC | MINC | CIRC | DIR[1:0] | PFCRTL | TCIE | HTIE | TEIE | DMEIE | EN |
|  |  | 0 | 1 | 0 | 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00A4 | DMA_S6NTR | 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00A8 | DMA_S6PAR | Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


--- Page 143 ---

DMA controller (DMA)
RM0090

Table 51. DMA register map and reset values (continued)

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00AC | DMA_S6M0AR | M0A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00B0 | DMA_S6M1AR | M1A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00B4 | DMA_S6FCR | Reserved | FEIE | Reserved | FS[2:0] | DMDIS | FTH [1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  | 1 | 0 | 1 |  |  |  |  |
| 0x00B8 | DMA_S7CR | Reserved | CHSEL[2:0] | MBURST[1:0] | PBURST[1:0] | Reserved | CT | DBM | PL[1:0] | PINCOS | MSIZE[1:0] | PSIZE[1:0] | MINC | PINC | CIRC | DIR[1:0] | PFCTRL | TCIE | HTIE | TEIE | DMEIE | EN |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 | - | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00BC | DMA_S7NDTR | Reserved | NDT[15:] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  |  |  |  |  |  |  |  |
| 0x00C0 | DMA_S7PAR | PA[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00C4 | DMA_S7M0AR | M0A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00C8 | DMA_S7M1AR | M1A[31:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x00CC | DMA_S7FCR | Reserved | FEIE | Reserved | FS[2:0] | DMDIS | FTH [1:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 0 |  | 1 | 0 | 1 |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses.

338/1749
RM0090 Rev 18

--- Page 144 ---

RM0090
Chrom-Art Accelerator™ controller (DMA2D)

