> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.28 SD/SDIO MMC card host interface (SDIO) characteristics**

**6.3.28 SD/SDIO MMC card host interface (SDIO) characteristics**

Unless otherwise specified, the parameters given in Table 88 are derived from tests performed under ambient temperature, f<sub>PCLKx</sub> frequency and V<sub>DD</sub> supply voltage conditions summarized in Table 14 with the following configuration:

* Output speed is set to OSPEEDRy[1:0] = 10
* Capacitive load C = 30 pF
* Measurement points are done at CMOS levels: 0.5V<sub>DD</sub>

Refer to Section 6.3.16: I/O port characteristics for more details on the input/output characteristics.

Figure 71. SDIO high-speed mode

ai14887

ST
DS8626 Rev 12
163/206

--- Page 164 ---

Electrical characteristics
STM32F405xx, STM32F407xx

Figure 72. SD default mode

CK
D, CMD
(output)
tOVD
tOHD
ai14888

Table 88. Dynamic characteristics: SD/MMC characteristics(1)
| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| fPP | Clock frequency in data transfer mode | - | 0 | - | 48 | MHz |
|  | SDIO_CK/fPCLK2 frequency ratio | - | 8/3 | - |  |  |
| tW(CKL) | Clock low time | fPP = 48 MHz | 8.5 | 9 | - | ns |
| tW(CKH) | Clock high time | fPP = 48 MHz | 8.3 | 10 | - |  |
| CMD, D inputs (referenced to CK) in MMC and SD HS mode |  |  |  |  |  |  |
| tISU | Input setup time HS | fPP = 48 MHz | 3 | - | ns |  |
| tIH | Input hold time HS | fPP = 48 MHz | 0 | - |  |  |
| CMD, D outputs (referenced to CK) in MMC and SD HS mode |  |  |  |  |  |  |
| tOV | Output valid time HS | fPP = 48 MHz | - | 4.5 | 6 | ns |
| tOH | Output hold time HS | fPP = 48 MHz | 1 | - |  |  |
| CMD, D inputs (referenced to CK) in SD default mode |  |  |  |  |  |  |
| tISUD | Input setup time SD | fPP = 24 MHz | 1.5 | - | ns |  |
| tIHD | Input hold time SD | fPP = 24 MHz | 0.5 | - |  |  |
| CMD, D outputs (referenced to CK) in SD default mode |  |  |  |  |  |  |
| tOVD | Output valid default time SD | fPP = 24 MHz | - | 4.5 | 7 | ns |
| tOHD | Output hold default time SD | fPP = 24 MHz | 0.5 | - |  |  |


1. Evaluated by characterization - not tested in production.

6.3.29 RTC characteristics

Table 89. RTC characteristics
| Symbol | Parameter | Conditions | Min | Max |
|---|---|---|---|---|
| - | fPCLK1/RTCCLK frequency ratio | Any read/write operation from/to an RTC register | 4 | - |


164/206
DS8626 Rev 12
ST

--- Page 165 ---

STM32F405xx, STM32F407xx
Package information

