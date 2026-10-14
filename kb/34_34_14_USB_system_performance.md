### 34.14 USB system performance

Best USB and system performance is achieved owing to the large RAM buffers, the highly configurable FIFO sizes, the quick 32-bit FIFO access through AHB push/pop registers and, especially, the advanced FIFO control mechanism. Indeed, this mechanism allows the OTG_FS to fill in the available RAM space at best regardless of the current USB sequence. With these features:

* The application gains good margins to calibrate its intervention in order to optimize the CPU bandwidth usage:
    * It can accumulate large amounts of transmission data in advance compared to when they are effectively sent over the USB
    * It benefits of a large time margin to download data from the single receive FIFO
* The USB Core is able to maintain its full operating rate, that is to provide maximum full-speed bandwidth with a great margin of autonomy versus application intervention:
    * It has a large reserve of transmission data at its disposal to autonomously manage the sending of data over the USB

1262/1749 RM0090 Rev 18 ST

--- Page 93 ---

RM0090
USB on-the-go full-speed (OTG_FS)

- It has a lot of empty space available in the receive buffer to autonomously fill it in with the data coming from the USB

As the OTG_FS core is able to fill in the 1.25 Kbyte RAM buffer very efficiently, and as 1.25 Kbyte of transmit/receive data is more than enough to cover a full speed frame, the USB system is able to withstand the maximum full-speed data rate for up to one USB frame (1 ms) without any CPU intervention.

