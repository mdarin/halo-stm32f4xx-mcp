> [RM0090 ch.27 §27.3] I²C functional description


### 27.3.1 Mode selection

The interface can operate in one of the four following modes:

- Slave transmitter
- Slave receiver
- Master transmitter
- Master receiver

By default, it operates in slave mode. The interface automatically switches from slave to master, after it generates a START condition and from master to slave, if an arbitration loss or a Stop generation occurs, allowing multimaster capability.

**Communication flow**

In Master mode, the I²C interface initiates a data transfer and generates the clock signal. A serial data transfer always begins with a start condition and ends with a stop condition. Both start and stop conditions are generated in master mode by software.

In Slave mode, the interface is capable of recognizing its own addresses (7 or 10-bit), and the General Call address. The General Call address detection may be enabled or disabled by software.

840/1749 RM0090 Rev 18 STI

--- Page 61 ---

**RM0090**
**Inter-integrated circuit (I2C) interface**

Data and addresses are transferred as 8-bit bytes, MSB first. The first byte(s) following the start condition contain the address (one in 7-bit mode, two in 10-bit mode). The address is always transmitted in Master mode.

A 9th clock pulse follows the 8 clock cycles of a byte transfer, during which the receiver must send an acknowledge bit to the transmitter. Refer to *Figure 238*.

**Figure 238. I²C bus protocol**

MS19854V1

Acknowledge may be enabled or disabled by software. The I²C interface addresses (dual addressing 7-bit/ 10-bit and/or general call address) can be selected by software.

The block diagram of the I²C interface is shown in *Figure 239*.

ST
RM0090 Rev 18
841/1749

--- Page 62 ---

Inter-integrated circuit (I²C) interface
RM0090

Figure 239. I²C block diagram for STM32F40x/41x

Data register
SDA
Noise filter
Data control
Data shift register
Comparator
PEC calculation
Own address register
Dual address register
PEC register
SCL
Noise filter
Clock control
Clock control Register (CCR)
Control registers (CR1&CR2)
Status registers (SR1&SR2)
Control logic
SMBA
Interrupts
DMA requests & ACK
MS30035V1

842/1749
RM0090 Rev 18

--- Page 63 ---

RM0090
Inter-integrated circuit (I2C) interface

Figure 240. I²C block diagram for STM32F42x/43x

Data register
SDA
Noise filter
Data control
Data shift register
Comparator
PEC calculation
Own address register
Dual address register
PEC register
SCL
Noise filter
Clock control
Clock control Register (CCR)
Control registers (CR1&CR2)
Status registers (SR1&SR2)
Control logic
SMBA
Interrupts
DMA requests & ACK
MS30035V1

1. SMBA is an optional signal in SMBus mode. This signal is not applicable if SMBus is disabled.

27.3.2
**I²C slave mode**

By default the I²C interface operates in Slave mode. To switch from default Slave mode to Master mode a Start condition generation is needed.

The peripheral input clock must be programmed in the I2C_CR2 register in order to generate correct timings. The peripheral input clock frequency must be at least:
* 2 MHz in Sm mode
* 4 MHz in Fm mode

As soon as a start condition is detected, the address is received from the SDA line and sent to the shift register. Then it is compared with the address of the interface (OAR1) and with OAR2 (if ENDUAL=1) or the General Call address (if ENGC = 1).

Note:
In 10-bit addressing mode, the comparison includes the header sequence (11110xx0), where xx denotes the two most significant bits of the address.

ST
RM0090 Rev 18
843/1749

--- Page 64 ---

**Inter-integrated circuit (I2C) interface** RM0090

**Header or address not matched**: the interface ignores it and waits for another Start condition.

**Header matched** (10-bit mode only): the interface generates an acknowledge pulse if the ACK bit is set and waits for the 8-bit slave address.

**Address matched**: the interface generates in sequence:
* An acknowledge pulse if the ACK bit is set
* The ADDR bit is set by hardware and an interrupt is generated if the ITEVFEN bit is set.
* If ENDUAL=1, the software has to read the DUALF bit to check which slave address has been acknowledged.

In 10-bit mode, after receiving the address sequence the slave is always in Receiver mode. It will enter Transmitter mode on receiving a repeated Start condition followed by the header sequence with matching address bits and the least significant bit set (11110xx1).

The TRA bit indicates whether the slave is in Receiver or Transmitter mode.

**Slave transmitter**

Following the address reception and after clearing ADDR, the slave sends bytes from the DR register to the SDA line via the internal shift register.

The slave stretches SCL low until ADDR is cleared and DR filled with the data to be sent (see Figure 241 Transfer sequencing EV1 EV3).

When the acknowledge pulse is received:
* The TxE bit is set by hardware with an interrupt if the ITEVFEN and the ITBUFEN bits are set.

If TxE is set and some data were not written in the I2C_DR register before the end of the next data transmission, the BTF bit is set and the interface waits until BTF is cleared by a read to I2C_SR1 followed by a write to the I2C_DR register, stretching SCL low.

844/1749 RM0090 Rev 18 ST

--- Page 65 ---

RM0090
Inter-integrated circuit (I2C) interface

Figure 241. Transfer sequence diagram for slave transmitter

7-bit slave transmitter
S Address A Data1 A Data2 A DataN NA P
EV1 EV3-1 EV3 EV3 EV3 EV3-2

10-bit slave transmitter
S Header A Address A
Sr Header A Data1 A DataN NA P
EV1 EV1 EV3_1 EV3 EV3 EV3-2

Legend: S= Start, Sr = Repeated Start, P= Stop, A= Acknowledge, NA= Non-acknowledge, EVx= Event (with interrupt if ITEVFEN=1)

EV1: ADDR=1, cleared by reading SR1 followed by reading SR2
EV3-1: TxE=1, shift register empty, data register empty, write Data1 in DR.
EV3: TxE=1, shift register not empty, data register empty, cleared by writing DR
EV3-2: AF=1; AF is cleared by writing '0' in AF bit of SR1 register.

ai18209

1. The EV1 and EV3_1 events stretch SCL low until the end of the corresponding software sequence.
2. The EV3 event stretches SCL low if the software sequence is not completed before the end of the next byte transmission.

Slave receiver

Following the address reception and after clearing ADDR, the slave receives bytes from the SDA line into the DR register via the internal shift register. After each byte the interface generates in sequence:

* An acknowledge pulse if the ACK bit is set
* The RxNE bit is set by hardware and an interrupt is generated if the ITEVFEN and ITBUFEN bit is set.

If RxNE is set and the data in the DR register is not read before the end of the next data reception, the BTF bit is set and the interface waits until BTF is cleared by a read from the I2C_DR register, stretching SCL low (see Figure 242 Transfer sequencing).

ST
RM0090 Rev 18
845/1749

--- Page 66 ---

Inter-integrated circuit (I2C) interface
RM0090

Figure 242. Transfer sequence diagram for slave receiver

7-bit slave receiver
S Address A Data1 A Data2 A DataN A P
EV1 EV2 EV2 EV2 EV4

10-bit slave receiver
S Header A Address A Data1 A DataN A P
EV1 EV2 EV2 EV4

Legend: S= Start, Sr = Repeated Start, P= Stop, A= Acknowledge,
EVx= Event (with interrupt if ITEVFEN=1)

EV1: ADDR=1, cleared by reading SR1 followed by reading SR2
EV2: RxNE=1 cleared by reading DR register.
EV4: STOPF=1, cleared by reading SR1 register followed by writing to the CR1 register

ai18208

1. The EV1 event stretches SCL low until the end of the corresponding software sequence.
2. The EV2 event stretches SCL low if the software sequence is not completed before the end of the next byte reception.
3. After checking the SR1 register content, the user should perform the complete clearing sequence for each flag found set.
Thus, for ADDR and STOPF flags, the following sequence is required inside the I2C interrupt routine:
READ SR1
if (ADDR == 1) {READ SR1; READ SR2}
if (STOPF == 1) {READ SR1; WRITE CR1}
The purpose is to make sure that both ADDR and STOPF flags are cleared if both are found set.

**Closing slave communication**

After the last data byte is transferred a Stop Condition is generated by the master. The interface detects this condition and sets:
* The STOPF bit and generates an interrupt if the ITEVFEN bit is set.

The STOPF bit is cleared by a read of the SR1 register followed by a write to the CR1 register (see EV4 in Figure 242).

> [RM0090 ch.27 §27.3] I²C functional description

### 27.3.3 I2C master mode**

**27.3.3 I2C master mode**

In Master mode, the I2C interface initiates a data transfer and generates the clock signal. A serial data transfer always begins with a Start condition and ends with a Stop condition. Master mode is selected as soon as the Start condition is generated on the bus with a START bit.

The following is the required sequence in master mode.
* Program the peripheral input clock in I2C_CR2 Register in order to generate correct timings
* Configure the clock control registers
* Configure the rise time register
* Program the I2C_CR1 register to enable the peripheral
* Set the START bit in the I2C_CR1 register to generate a Start condition

The peripheral input clock frequency must be at least:
* 2 MHz in Sm mode
* 4 MHz in Fm mode

846/1749
RM0090 Rev 18
ST

--- Page 67 ---

RM0090
Inter-integrated circuit (I2C) interface

> [RM0090 ch.27 §27.3] I²C functional description

### SCL master clock generation

## SCL master clock generation

The CCR bits are used to generate the high and low level of the SCL clock, starting from the generation of the rising and falling edge (respectively). As a slave may stretch the SCL line, the peripheral checks the SCL input from the bus at the end of the time programmed in TRISE bits after rising edge generation.

*   If the SCL line is low, it means that a slave is stretching the bus, and the high level counter stops until the SCL line is detected high. This allows to guarantee the minimum HIGH period of the SCL clock parameter.
*   If the SCL line is high, the high level counter keeps on counting.

Indeed, the feedback loop from the SCL rising edge generation by the peripheral to the SCL rising edge detection by the peripheral takes time even if no slave stretches the clock. This loopback duration is linked to the SCL rising time (impacting SCL VIH input detection), plus delay due to the noise filter present on the SCL input path, plus delay due to internal SCL input synchronization with APB clock. The maximum time used by the feedback loop is programmed in the TRISE bits, so that the SCL frequency remains stable whatever the SCL rising time.

> [RM0090 ch.27 §27.3] I²C functional description

### Start condition

## Start condition

Setting the START bit causes the interface to generate a Start condition and to switch to Master mode (MSL bit set) when the BUSY bit is cleared.

**Note:** In master mode, setting the START bit causes the interface to generate a ReStart condition at the end of the current byte transfer.

Once the Start condition is sent:

*   The SB bit is set by hardware and an interrupt is generated if the ITEVFEN bit is set.

Then the master waits for a read of the SR1 register followed by a write in the DR register with the Slave address (see Figure 243 and Figure 244 Transfer sequencing EV5).

