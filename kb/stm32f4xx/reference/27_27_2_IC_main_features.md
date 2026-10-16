# 27 Inter-integrated circuit (I2C) interface

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 27.1 I²C introduction

I²C (inter-integrated circuit) bus Interface serves as an interface between the microcontroller and the serial I²C bus. It provides multimaster capability, and controls all I²C bus-specific sequencing, protocol, arbitration and timing. It supports the standard mode (Sm, up to 100 kHz) and Fm mode (Fm, up to 400 kHz).

It may be used for a variety of purposes, including CRC generation and verification, SMBus (system management bus) and PMBus (power management bus).

Depending on specific device implementation DMA capability can be available for reduced CPU overload.

## 27.2 I²C main features

*   Parallel-bus/I²C protocol converter
*   Multimaster capability: the same interface can act as Master or Slave
*   I²C Master features:
    *   Clock generation
    *   Start and Stop generation
*   I²C Slave features:
    *   Programmable I²C Address detection
    *   Dual Addressing Capability to acknowledge 2 slave addresses
    *   Stop bit detection
*   Generation and detection of 7-bit/10-bit addressing and General Call
*   Supports different communication speeds:
    *   Standard Speed (up to 100 kHz)
    *   Fast Speed (up to 400 kHz)
*   Analog noise filter
*   Programmable digital noise filter for STM32F42xxx and STM32F43xxx
*   Status flags:
    *   Transmitter/Receiver mode flag
    *   End-of-Byte transmission flag
    *   I²C busy flag
*   Error flags:
    *   Arbitration lost condition for master mode
    *   Acknowledgment failure after address/ data transmission
    *   Detection of misplaced start or stop condition
    *   Overrun/Underrun if clock stretching is disabled
*   2 Interrupt vectors:

ST
RM0090 Rev 18
839/1749

--- Page 60 ---

**Inter-integrated circuit (I²C) interface** RM0090

- 1 Interrupt for successful address/ data communication
- 1 Interrupt for error condition
- Optional clock stretching
- 1-byte buffer with DMA capability
- Configurable PEC (packet error checking) generation or verification:
  - PEC value can be transmitted as last byte in Tx mode
  - PEC error checking for last received byte
- SMBus 2.0 Compatibility:
  - 25 ms clock low timeout delay
  - 10 ms master cumulative clock low extend time
  - 25 ms slave cumulative clock low extend time
  - Hardware PEC generation/verification with ACK control
  - Address Resolution Protocol (ARP) supported
- PMBus Compatibility

*Note:* Some of the above features may not be available in certain products. The user should refer to the product data sheet, to identify the specific features supported by the I²C interface implementation.

