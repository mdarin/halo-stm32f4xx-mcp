> [RM0090 ch.27 §27.3] I²C functional description


## Slave address transmission

Then the slave address is sent to the SDA line via the internal shift register.

*   In 10-bit addressing mode, sending the header sequence causes the following event:
    *   The ADD10 bit is set by hardware and an interrupt is generated if the ITEVFEN bit is set.

    Then the master waits for a read of the SR1 register followed by a write in the DR register with the second address byte (see Figure 243 and Figure 244 Transfer sequencing).
    *   The ADDR bit is set by hardware and an interrupt is generated if the ITEVFEN bit is set.

    Then the master waits for a read of the SR1 register followed by a read of the SR2 register (see Figure 243 and Figure 244 Transfer sequencing).
*   In 7-bit addressing mode, one address byte is sent.

As soon as the address byte is sent,

*   The ADDR bit is set by hardware and an interrupt is generated if the ITEVFEN bit is set.

Then the master waits for a read of the SR1 register followed by a read of the SR2 register (see Figure 243 and Figure 244 Transfer sequencing).

ST
RM0090 Rev 18
847/1749

--- Page 68 ---

**Inter-integrated circuit (I2C) interface** RM0090

The master can decide to enter Transmitter or Receiver mode depending on the LSB of the slave address sent.

*   In 7-bit addressing mode,
    *   To enter Transmitter mode, a master sends the slave address with LSB reset.
    *   To enter Receiver mode, a master sends the slave address with LSB set.
*   In 10-bit addressing mode,
    *   To enter Transmitter mode, a master sends the header (11110xx0) and then the slave address, (where xx denotes the two most significant bits of the address).
    *   To enter Receiver mode, a master sends the header (11110xx0) and then the slave address. Then it should send a repeated Start condition followed by the header (11110xx1), (where xx denotes the two most significant bits of the address).

The TRA bit indicates whether the master is in Receiver or Transmitter mode.

**Master transmitter**

Following the address transmission and after clearing ADDR, the master sends bytes from the DR register to the SDA line via the internal shift register.

The master waits until the first data byte is written into I2C_DR (see Figure 243 Transfer sequencing EV8_1).

When the acknowledge pulse is received, the TxE bit is set by hardware and an interrupt is generated if the ITEVFEN and ITBUFEN bits are set.

If TxE is set and a data byte was not written in the DR register before the end of the last data transmission, BTF is set and the interface waits until BTF is cleared by a write to I2C_DR, stretching SCL low.

**Closing the communication**

After the last byte is written to the DR register, the STOP bit is set by software to generate a Stop condition (see Figure 243 Transfer sequencing EV8_2). The interface automatically goes back to slave mode (MSL bit cleared).

*Note:* Stop condition should be programmed during EV8_2 event, when either TxE or BTF is set.

848/1749 RM0090 Rev 18 STI

--- Page 69 ---

RM0090
Inter-integrated circuit (I2C) interface

Figure 243. Transfer sequence diagram for master transmitter

7-bit master transmitter
S Address A Data1 A Data2 A DataN A P
EV5 EV6 EV8_1 EV8 EV8 EV8_2

10-bit master transmitter
S Header A Address A Data1 A DataN A P
EV5 EV9 EV6 EV8_1 EV8 EV8_2

Legend: S= Start, Sr = Repeated Start, P= Stop, A= Acknowledge,
EVx= Event (with interrupt if ITEVFEN = 1)

EV5: SB=1, cleared by reading SR1 register followed by writing DR register with Address.
EV6: ADDR=1, cleared by reading SR1 register followed by reading SR2.
EV8_1: TxE=1, shift register empty, data register empty, write Data1 in DR.
EV8: TxE=1, shift register not empty, data register empty, cleared by writing DR register
EV8_2: TxE=1, BTF = 1, Program Stop request. TxE and BTF are cleared by hardware by the Stop condition
EV9: ADD10=1, cleared by reading SR1 register followed by writing DR register.

ai18210

1. The EV5, EV6, EV9, EV8_1 and EV8_2 events stretch SCL low until the end of the corresponding software sequence.
2. The EV8 event stretches SCL low if the software sequence is not complete before the end of the next byte transmission.

**Master receiver**

Following the address transmission and after clearing ADDR, the I²C interface enters Master Receiver mode. In this mode the interface receives bytes from the SDA line into the DR register via the internal shift register. After each byte the interface generates in sequence:

1. An acknowledge pulse if the ACK bit is set
2. The RxNE bit is set and an interrupt is generated if the ITEVFEN and ITBUFEN bits are set (see Figure 244 Transfer sequencing EV7).

If the RxNE bit is set and the data in the DR register is not read before the end of the last data reception, the BTF bit is set by hardware and the interface waits until BTF is cleared by a read in the DR register, stretching SCL low.

**Closing the communication**

The master sends a NACK for the last byte received from the slave. After receiving this NACK, the slave releases the control of the SCL and SDA lines. Then the master can send a Stop/Restart condition.

1. To generate the nonacknowledge pulse after the last received data byte, the ACK bit must be cleared just after reading the second last data byte (after second last RxNE event).
2. In order to generate the Stop/Restart condition, software must set the STOP/START bit after reading the second last data byte (after the second last RxNE event).
3. In case a single byte has to be received, the Acknowledge disable is made during EV6 (before ADDR flag is cleared) and the STOP condition generation is made after EV6.

After the Stop condition generation, the interface goes automatically back to slave mode (MSL bit cleared).

ST
RM0090 Rev 18
849/1749

--- Page 70 ---

Inter-integrated circuit (I2C) interface
RM0090

Figure 244. Transfer sequence diagram for master receiver

7-bit master receiver
S Address A Data1 A(1) Data2 A DataN NA P
EV5 EV6 EV7 EV7 EV7_1 EV7

10-bit master receiver
S Header A Address A
EV5 EV9 EV6

S, Header A Data1 A(1) Data2 A DataN NA P
EV5 EV6 EV7 EV7 EV7_1 EV7

Legend: S= Start, Sr = Repeated Start, P= Stop, A= Acknowledge, NA= Non-acknowledge,
EVx= Event (with interrupt if ITEVFEN=1)
EV5: SB=1, cleared by reading SR1 register followed by writing DR register.
EV6: ADDR=1, cleared by reading SR1 register followed by reading SR2. In 10-bit master receiver mode, this sequence should be followed by writing CR2 with START = 1.
In case of the reception of 1 byte, the Acknowledge disable must be performed during EV6 event, i.e. before clearing ADDR flag.
EV7: RxNE=1 cleared by reading DR register.
EV7_1: RxNE=1 cleared by reading DR register, program ACK=0 and STOP request
EV9: ADD10=1, cleared by reading SR1 register followed by writing DR register.

ai17540d

1. If a single byte is received, it is NA.
2. The EV5, EV6 and EV9 events stretch SCL low until the end of the corresponding software sequence.
3. The EV7 event stretches SCL low if the software sequence is not completed before the end of the next byte reception.
4. The EV7_1 software sequence must be completed before the ACK pulse of the current byte transfer.

The procedures described below are recommended if the EV7-1 software sequence is not completed before the ACK pulse of the current byte transfer.

These procedures must be followed to make sure:
* The ACK bit is set low on time before the end of the last data reception
* The STOP bit is set high after the last data reception without reception of supplementary data.

**For 2-byte reception:**
* Wait until ADDR = 1 (SCL stretched low until the ADDR flag is cleared)
* Set ACK low, set POS high
* Clear ADDR flag
* Wait until BTF = 1 (Data 1 in DR, Data2 in shift register, SCL stretched low until a data 1 is read)
* Set STOP high
* Read data 1 and 2

850/1749
RM0090 Rev 18

--- Page 71 ---

RM0090
Inter-integrated circuit (I2C) interface

For N >2 -byte reception, from N-2 data reception
* Wait until BTF = 1 (data N-2 in DR, data N-1 in shift register, SCL stretched low until data N-2 is read)
* Set ACK low
* Read data N-2
* Wait until BTF = 1 (data N-1 in DR, data N in shift register, SCL stretched low until a data N-1 is read)
* Set STOP high
* Read data N-1 and N

> [RM0090 ch.27 §27.3] I²C functional description

### 27.3.4 Error conditions

### 27.3.4 Error conditions

The following are the error conditions which may cause communication to fail.

> [RM0090 ch.27 §27.3] I²C functional description

### Bus error (BERR)

#### Bus error (BERR)

This error occurs when the I²C interface detects an external Stop or Start condition during an address or a data transfer. In this case:
* the BERR bit is set and an interrupt is generated if the ITERREN bit is set
* in Slave mode: data are discarded and the lines are released by hardware:
    * in case of a misplaced Start, the slave considers it is a restart and waits for an address, or a Stop condition
    * in case of a misplaced Stop, the slave behaves like for a Stop condition and the lines are released by hardware
* In Master mode: the lines are not released and the state of the current transmission is not affected. It is up to the software to abort or not the current transmission

> [RM0090 ch.27 §27.3] I²C functional description

### Acknowledge failure (AF)

#### Acknowledge failure (AF)

This error occurs when the interface detects a nonacknowledge bit. In this case:
* the AF bit is set and an interrupt is generated if the ITERREN bit is set
* a transmitter which receives a NACK must reset the communication:
    * If Slave: lines are released by hardware
    * If Master: a Stop or repeated Start condition must be generated by software

> [RM0090 ch.27 §27.3] I²C functional description

### Arbitration lost (ARLO)

#### Arbitration lost (ARLO)

This error occurs when the I²C interface detects an arbitration lost condition. In this case
* the ARLO bit is set by hardware (and an interrupt is generated if the ITERREN bit is set)
* the I²C Interface goes automatically back to slave mode (the MSL bit is cleared). When the I²C loses the arbitration, it is not able to acknowledge its slave address in the same transfer, but it can acknowledge it after a repeated Start from the winning master.
* lines are released by hardware

ST
RM0090 Rev 18
851/1749

--- Page 72 ---

**Inter-integrated circuit (I²C) interface**
RM0090

**Overrun/underrun error (OVR)**

An overrun error can occur in slave mode when clock stretching is disabled and the I²C interface is receiving data. The interface has received a byte (RxNE=1) and the data in DR has not been read, before the next byte is received by the interface. In this case,
* The last received byte is lost.
* In case of Overrun error, software should clear the RxNE bit and the transmitter should re-transmit the last received byte.

Underrun error can occur in slave mode when clock stretching is disabled and the I²C interface is transmitting data. The interface has not updated the DR with the next byte (TxE=1), before the clock comes for the next byte. In this case,
* The same byte in the DR register will be sent again
* The user should make sure that data received on the receiver side during an underrun error are discarded and that the next bytes are written within the clock low time specified in the I²C bus standard.

For the first byte to be transmitted, the DR must be written after ADDR is cleared and before the first SCL rising edge. If not possible, the receiver must discard the first data.

> [RM0090 ch.27 §27.3] I²C functional description

### 27.3.5 Programmable noise filter**

**27.3.5 Programmable noise filter**

The programmable noise filter is available on STM32F42xxx and STM32F43xxx devices only.

In Fm mode, the I²C standard requires that spikes are suppressed to a length of 50 ns on SDA and SCL lines.

An analog noise filter is implemented in the SDA and SCL I/Os. This filter is enabled by default and can be disabled by setting the ANOFF bit in the I2C_FLTR register.

A digital noise filter can be enabled by configuring the DNF[3:0] bits to a non-zero value. This suppresses the spikes on SDA and SCL inputs with a length of up to DNF[3:0] * T_PCLK1.

Enabling the digital noise filter increases the SDA hold time by (DNF[3:0] +1)* T_PCLK.

To be compliant with the maximum hold time of the I²C-bus specification version 2.1 (Thd:dat), the DNF bits must be programmed using the constraints shown in Table 122, and assuming that the analog filter is disabled.

*Note:* DNF[3:0] must only be configured when the I²C is disabled (PE = 0). If the analog filter is also enabled, the digital filter is added to the analog filter.

**Table 122. Maximum DNF[3:0] value to be compliant with Thd:dat(max)**

| PCLK1 frequency | Maximum DNF value |  |
|---|---|---|
| Sm mode | Fm mode |  |
| 2 <= FPCLK1 <= 5 | 2 | 0 |
| 5 < FPCLK1 <= 10 | 12 | 0 |
| 10 < FPCLK1 <= 20 | 15 | 1 |
| 20 < FPCLK1 <= 30 | 15 | 7 |


852/1749
RM0090 Rev 18
STI

--- Page 73 ---

RM0090
Inter-integrated circuit (I2C) interface

**Table 122. Maximum DNF[3:0] value to be compliant with Thd:dat(max) (continued)**

| PCLK1 frequency | Maximum DNF value |  |
|---|---|---|
| Sm mode | Fm mode |  |
| 30 < FPCLK1 <= 40 | 15 | 13 |
| 40 < FPCLK1 <= 50 | 15 |  |


**Note:** For each frequency range, the constraint is given based on the worst case which is the minimum frequency of the range. Greater DNF values can be used if the system can support maximum hold time violation.

> [RM0090 ch.27 §27.3] I²C functional description

### 27.3.6 SDA/SCL line control**

**27.3.6 SDA/SCL line control**

*   If clock stretching is enabled:
    *   Transmitter mode: If TxE=1 and BTF=1: the interface holds the clock line low before transmission to wait for the microcontroller to write the byte in the Data Register (both buffer and shift register are empty).
    *   Receiver mode: If RxNE=1 and BTF=1: the interface holds the clock line low after reception to wait for the microcontroller to read the byte in the Data Register (both buffer and shift register are full).
*   If clock stretching is disabled in Slave mode:
    *   Overrun Error in case of RxNE=1 and no read of DR has been done before the next byte is received. The last received byte is lost.
    *   Underrun Error in case TxE=1 and no write into DR has been done before the next byte must be transmitted. The same byte will be sent again.
    *   Write Collision not managed.

> [RM0090 ch.27 §27.3] I²C functional description

### 27.3.7 SMBus**

**27.3.7 SMBus**

**Introduction**

The System Management Bus (SMBus) is a two-wire interface through which various devices can communicate with each other and with the rest of the system. It is based on I²C principles of operation. SMBus provides a control bus for system and power management related tasks. A system may use SMBus to pass messages to and from devices instead of toggling individual control lines.

The System Management Bus Specification refers to three types of devices. A *slave* is a device that is receiving or responding to a command. A *master* is a device that issues commands, generates the clocks, and terminates the transfer. A *host* is a specialized master that provides the main interface to the system's CPU. A host must be a master-slave and must support the SMBus host notify protocol. Only one host is allowed in a system.

**Similarities between SMBus and I²C**

*   2-wire bus protocol (1 Clk, 1 Data) + SMBus Alert line optional
*   Master-slave communication, Master provides clock
*   Multi master capability
*   SMBus data format similar to I²C 7-bit addressing format (Figure 238).

ST
RM0090 Rev 18
853/1749

--- Page 74 ---

Inter-integrated circuit (I²C) interface
RM0090

> [RM0090 ch.27 §27.3] I²C functional description

### Differences between SMBus and I²C

## Differences between SMBus and I²C

The following table describes the differences between SMBus and I²C.

Table 123. SMBus vs. I²C

| SMBus | I²C |
|---|---|
| Max. speed 100 kHz | Max. speed 400 kHz |
| Min. clock speed 10 kHz | No minimum clock speed |
| 35 ms clock low timeout | No timeout |
| Logic levels are fixed | Logic levels are VDD dependent |
| Different address types (reserved, dynamic etc.) | 7-bit, 10-bit and general call slave address types |
| Different bus protocols (quick command, process call etc.) | No bus protocols |


> [RM0090 ch.27 §27.3] I²C functional description

### SMBus application usage

### SMBus application usage

With System Management Bus, a device can provide manufacturer information, tell the system what its model/part number is, save its state for a suspend event, report different types of errors, accept control parameters, and return its status. SMBus provides a control bus for system and power management related tasks.

> [RM0090 ch.27 §27.3] I²C functional description

### Device identification

### Device identification

Any device that exists on the System Management Bus as a slave has a unique address called the Slave Address. For the list of reserved slave addresses, refer to the SMBus specification version. 2.0 (http://smbus.org/).

> [RM0090 ch.27 §27.3] I²C functional description

### Bus protocols

### Bus protocols

The SMBus specification supports up to nine bus protocols. For more details of these protocols and SMBus address types, refer to SMBus specification version. 2.0. These protocols should be implemented by the user software.

> [RM0090 ch.27 §27.3] I²C functional description

### Address resolution protocol (ARP)

### Address resolution protocol (ARP)

SMBus slave address conflicts can be resolved by dynamically assigning a new unique address to each slave device. The Address Resolution Protocol (ARP) has the following attributes:

* Address assignment uses the standard SMBus physical layer arbitration mechanism
* Assigned addresses remain constant while device power is applied; address retention through device power loss is also allowed
* No additional SMBus packet overhead is incurred after address assignment. (i.e. subsequent accesses to assigned slave addresses have the same overhead as accesses to fixed address devices.)
* Any SMBus master can enumerate the bus

