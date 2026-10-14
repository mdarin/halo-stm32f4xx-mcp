## 29.3 Functional block diagram

The block diagram of the SAI is shown in Figure 283.

Figure 283. Functional block diagram

APB
Serial Audio Interface (SAI)
APB interface
SAI_XCR1
Clock generator Audio block A
FIFO
FIFO ctrl
Configuration registers and Status register
FSM
32-bit shift register
synchro ctrl out
int_sck
int_FS
FS_A
SCK_A
SD_A
MCLK_A
Audio block A
SAI_CK_A
I/O line Management
SAI_CK_B
Clock generator Audio block B
FIFO
FIFO ctrl
Configuration registers and Status register
FSM
32-bit shift register
SAI_XCR1
APB interface
APB
Audio block B
FS_B
SCK_B
SD_B
MCLK_B
MS30032V1

The SAI is mainly composed of two audio sub-blocks with their own clock generator. Each audio block integrates a 32-bit shift register controlled by their own functional state machine. Data are stored or read from the dedicated FIFO. FIFO may be accessed by the CPU, or by DMA in order to leave the CPU free during the communication. Each audio block is independent. They can be synchronous with each other.

An I/O line controller manages each dedicated pins for a given audio block in the SAI. If the two blocks are synchronized, this controller reduces the number of I/Os used, freeing up an FS pin, an SCK pin and eventually an MCLK pin, making them general purpose I/Os.

The functional state machine can be configured to address a wide range of audio protocols. Some registers are present to set-up the desired protocols (audio frame waveform generator).

The audio block can be a transmitter or receiver, in master or slave mode. The master mode means the bit clock SCK and the frame synchronization signal are generated from the SAI, whereas in slave mode, they come from another external or internal master. There is a particular case for which the FS signal direction is not directly linked to the master or slave

928/1749
RM0090 Rev 18
ST

--- Page 149 ---

RM0090
Serial audio interface (SAI)

mode definition. In AC'97 protocol, it will be an SAI output even if the SAI (link controller) is set-up to consume the SCK clock (and so to be in Slave mode).

