> [RM0090 ch.28 §28.4] I²S functional description

### 28.4.4 Clock generator**

**28.4.4 Clock generator**

The I²S bitrate determines the dataflow on the I²S data line and the I²S clock signal frequency.

I²S bitrate = number of bits per channel × number of channels × sampling audio frequency

For a 16-bit audio, left and right channel, the I²S bitrate is calculated as follows:
I²S bitrate = 16 × 2 × Fₛ

It will be: I²S bitrate = 32 x 2 x Fₛ if the packet length is 32-bit wide.

ST
RM0090 Rev 18
907/1749

--- Page 128 ---

**Serial peripheral interface (SPI)**
RM0090

**Figure 281. Audio sampling frequency definition**

16-or 32-bit left channel
16-or 32-bit right channel
32- or 64-bits
Fs
sampling point
sampling point
Fs : audio sampling frequency
MS30108V1

When the master mode is configured, a specific action needs to be taken to properly program the linear divider in order to communicate with the desired audio frequency.

**Figure 282. I²S clock generator architecture**

I²SxCLK
8-bit linear divider + reshaping stage
Divider by 4
Div2
MCK
CK
MCKOE
MCKOE_ODD
I²SDIV[7:0]
I²SMOD
CHLEN
MS30109V1

1. Where x could be 2 or 3.

*Figure 281* presents the communication clock architecture. To achieve high-quality audio performance, the I2SxCLK clock source can be either the PLLI2S output (through R division factor) or an external clock (mapped to I2S_CKIN pin).

The audio sampling frequency can be 192 kHz, 96 kHz, or 48 kHz. In order to reach the desired frequency, the linear divider needs to be programmed according to the formulas below:

When the master clock is generated (MCKOE in the SPI_I2SPR register is set):
Fs = I2SxCLK / [(16*2)*((2*I2SDIV)+ODD)*8)] when the channel frame is 16-bit wide
Fs = I2SxCLK / [(32*2)*((2*I2SDIV)+ODD)*4)] when the channel frame is 32-bit wide

When the master clock is disabled (MCKOE bit cleared):
Fs = I2SxCLK / [(16*2)*((2*I2SDIV)+ODD))] when the channel frame is 16-bit wide
Fs = I2SxCLK / [(32*2)*((2*I2SDIV)+ODD))] when the channel frame is 32-bit wide

*Table 127* provides example precision values for different clock configurations.

**Note:**
*Other configurations are possible that allow optimum clock precision.*

908/1749
RM0090 Rev 18

--- Page 129 ---

RM0090
Serial peripheral interface (SPI)

Table 127. Audio frequency precision (for PLLM VCO = 1 MHz or 2 MHz)(1)

| Master clock | Target fS (Hz) | Data format | PLLI2SN | PLLI2SR | I2SDIV | I2SODD | Real fS (Hz) | Error |
|---|---|---|---|---|---|---|---|---|
| 8000 | 16-bit | 192 | 2 | 187 | 1 | 8000 | 0.0000% |  |
| 32-bit | 192 | 3 | 62 | 1 | 8000 | 0.0000% |  |  |
| Disabled | 16000 | 16-bit | 192 | 3 | 62 | 1 | 16000 | 0.0000% |
| 32-bit | 256 | 2 | 62 | 1 | 16000 | 0.0000% |  |  |
| 32000 | 16-bit | 256 | 2 | 62 | 1 | 32000 | 0.0000% |  |
| 32-bit | 256 | 5 | 12 | 1 | 32000 | 0.0000% |  |  |
| 48000 | 16-bit | 192 | 5 | 12 | 1 | 48000 | 0.0000% |  |
| 32-bit | 384 | 5 | 12 | 1 | 48000 | 0.0000% |  |  |
| 96000 | 16-bit | 384 | 5 | 12 | 1 | 96000 | 0.0000% |  |
| 32-bit | 424 | 3 | 11 | 1 | 96014.49219 | 0.0151% |  |  |
| 22050 | 16-bit | 290 | 3 | 68 | 1 | 22049.87695 | 0.0006% |  |
| 32-bit | 302 | 2 | 53 | 1 | 22050.23438 | 0.0011% |  |  |
| 44100 | 16-bit | 302 | 2 | 53 | 1 | 44100.46875 | 0.0011% |  |
| 32-bit | 429 | 4 | 19 | 0 | 44099.50781 | 0.0011% |  |  |
| 192000 | 16-bit | 424 | 3 | 11 | 1 | 192028.9844 | 0.0151% |  |
| 32-bit | 258 | 3 | 1 | 191964.2813 | 0.0186% |  |  |  |
| 8000 | don't care | 256 | 5 | 12 | 1 | 8000 | 0.0000% |  |
| 16000 | don't care | 213 | 2 | 13 | 0 | 16000.60059 | 0.0038% |  |
| 32000 | don't care | 213 | 2 | 6 | 1 | 32001.20117 | 0.0038% |  |
| 48000 | don't care | 258 | 3 | 1 | 47991.07031 | 0.0186% |  |  |
| 96000 | don't care | 344 | 2 | 3 | 1 | 95982.14063 | 0.0186% |  |
| 22050 | don't care | 429 | 4 | 9 | 1 | 22049.75391 | 0.0011% |  |
| 44100 | don't care | 271 | 2 | 6 | 0 | 44108.07422 | 0.0183% |  |


1. This table gives only example values for different clock configurations. Other configurations allowing optimum clock precision are possible.

