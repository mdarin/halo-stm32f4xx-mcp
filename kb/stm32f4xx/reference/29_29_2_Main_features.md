# 29 Serial audio interface (SAI)

This section applies to the STM32F42xxx and STM32F43xxx family.

## 29.1 Introduction

The SAI interface (serial audio interface) offers a wide set of audio protocols due to its flexibility and wide range of configurations. Many stereo or mono audio applications may be targeted. I2S standards, LSB or MSB-justified, PCM/DSP, TDM, and AC'97 protocols may be addressed for example.

To bring this level of flexibility and configurability, the SAI contains two audio sub-blocks that are fully independent of each other. Each audio sub-block is connected to up to 4 pins (SD, SCK, FS, MCLK). Some of these pins can be shared if the two sub-blocks are declared as synchronous to leave some free to be used as general purpose I/Os. The MCLK pin can be output, or not, depending on the application, the decoder requirement and whether the audio block is configured as the master.

The SAI can work in master or slave configuration. The audio sub-blocks can be either receiver or transmitter and can work synchronously or not (with respect to the other one).

926/1749
RM0090 Rev 18
ST

--- Page 147 ---

RM0090
Serial audio interface (SAI)

## 29.2 Main features

* Two independent audio sub-blocks which can be transmitters or receivers with their respective FIFO.
* 8-word integrated FIFOs for each audio sub-block.
* Synchronous or asynchronous mode between the audio sub-blocks.
* Master or slave configuration independent for both audio sub-blocks.
* Clock generator for each audio block to target independent audio frequency sampling when both audio sub-blocks are configured in master mode.
* Data size configurable: 8-, 10-, 16-, 20-, 24-, 32-bit.
* Peripheral with large configurability and flexibility allowing to target as example the following audio protocol: I2S, LSB or MSB-justified, PCM/DSP, TDM, AC'97
* Up to 16 slots available with configurable size and with the possibility to select which ones are active in the audio frame.
* Number of bits by frame may be configurable.
* Frame synchronization active level configurable (offset, bit length, level).
* First active bit position in the slot is configurable.
* LSB first or MSB first for data transfer.
* Mute mode.
* Stereo/Mono audio frame capability.
* Communication clock strobing edge configurable (SCK).
* Error flags with associated interrupts if enabled respectively.
  - Overrun and underrun detection,
  - Anticipated frame synchronization signal detection in slave mode,
  - Late frame synchronization signal detection in slave mode,
  - Codec not ready for the AC'97 mode in reception.
* Interruption sources when enabled:
  - Errors,
  - FIFO requests.
* DMA interface with 2 dedicated channels to handle access to the dedicated integrated FIFO of each SAI audio sub-block.

RM0090 Rev 18
927/1749

--- Page 148 ---

Serial audio interface (SAI)
RM0090

