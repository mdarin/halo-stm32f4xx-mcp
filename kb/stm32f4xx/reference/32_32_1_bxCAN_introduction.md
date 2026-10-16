# 32 Controller area network (bxCAN)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 32.1 bxCAN introduction

The **Basic Extended CAN** peripheral, named **bxCAN**, interfaces the CAN network. It supports the CAN protocols version 2.0A and B. It has been designed to manage a high number of incoming messages efficiently with a minimum CPU load. It also meets the priority requirements for transmit messages.

For safety-critical applications, the CAN controller provides all hardware functions for supporting the CAN Time Triggered Communication option.

## 32.2 bxCAN main features

*   Supports CAN protocol version 2.0 A, B Active
*   Bit rates up to 1 Mbit/s
*   Supports the Time Triggered Communication option

**Transmission**

*   Three transmit mailboxes
*   Configurable transmit priority
*   Time Stamp on SOF transmission

**Reception**

*   Two receive FIFOs with three stages
*   Scalable filter banks:
    *   28 filter banks shared between CAN1 and CAN2
*   Identifier list feature
*   Configurable FIFO overrun
*   Time Stamp on SOF reception

**Time-triggered communication option**

*   Disable automatic retransmission mode
*   16-bit free running timer
*   Time Stamp sent in last two data bytes

**Management**

*   Maskable interrupts
*   Software-efficient mailbox mapping at a unique address space

1076/1749
RM0090 Rev 18
ST

--- Page 102 ---

RM0090
Controller area network (bxCAN)

**Dual CAN**

*   CAN1: Master bxCAN for managing the communication between a Slave bxCAN and the 512-byte SRAM memory
*   CAN2: Slave bxCAN, with no direct access to the SRAM memory.
*   The two bxCAN cells share the 512-byte SRAM memory (see Figure 335)

