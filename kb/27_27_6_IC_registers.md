## 27.6 I²C registers

Refer to Section 1.1 for a list of abbreviations used in register descriptions.
The peripheral registers have to be accessed by half-words (16 bits) or words (32 bits).

### 27.6.1 I²C Control register 1 (I2C_CR1)

Address offset: 0x00
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| SWRST | Res. | ALERT | PEC | POS | ACK | STOP | START | NO STRETCH | ENGC | ENPEC | ENARP | SMB TYPE | Res. | SMBUS | PE |
| rw |  | rw |  | rw |  |  |  |  |  |  |  |  |  |  |  |


**Bit 15 SWRST: Software reset**
When set, the I2C is under reset state. Before resetting this bit, make sure the I2C lines are released and the bus is free.
0: I²C Peripheral not under reset
1: I²C Peripheral under reset state
*Note: This bit can be used to reinitialize the peripheral after an error or a locked state. As an example, if the BUSY bit is set and remains locked due to a glitch on the bus, the SWRST bit can be used to exit from this state.*

**Bit 14** Reserved, must be kept at reset value

**Bit 13 ALERT: SMBus alert**
This bit is set and cleared by software, and cleared by hardware when PE=0.
0: Releases SMB pin high. Alert Response Address Header followed by NACK.
1: Drives SMB pin low. Alert Response Address Header followed by ACK.

**Bit 12 PEC: Packet error checking**
This bit is set and cleared by software, and cleared by hardware when PEC is transferred or by a START or Stop condition or when PE=0.
0: No PEC transfer
1: PEC transfer (in Tx or Rx mode)
*Note: PEC calculation is corrupted by an arbitration loss.*

**Bit 11 POS: Acknowledge/PEC Position (for data reception)**
This bit is set and cleared by software and cleared by hardware when PE=0.
0: ACK bit controls the (N)ACK of the current byte being received in the shift register. The PEC bit indicates that current byte in shift register is a PEC.
1: ACK bit controls the (N)ACK of the next byte which will be received in the shift register. The PEC bit indicates that the next byte in the shift register is a PEC
*Note: The POS bit must be used only in 2-byte reception configuration in master mode. It must be configured before data reception starts, as described in the 2-byte reception procedure recommended in Section : Master receiver on page 849.*

860/1749
RM0090 Rev 18
ST

--- Page 81 ---

RM0090
Inter-integrated circuit (I2C) interface

**Bit 10 ACK**: Acknowledge enable
This bit is set and cleared by software and cleared by hardware when PE=0.
0: No acknowledge returned
1: Acknowledge returned after a byte is received (matched address or data)

**Bit 9 STOP**: Stop generation
The bit is set and cleared by software, cleared by hardware when a Stop condition is detected, set by hardware when a timeout error is detected.
In Master Mode:
0: No Stop generation.
1: Stop generation after the current byte transfer or after the current Start condition is sent.
In Slave mode:
0: No Stop generation.
1: Release the SCL and SDA lines after the current byte transfer.

**Bit 8 START**: Start generation
This bit is set and cleared by software and cleared by hardware when start is sent or PE=0.
In Master Mode:
0: No Start generation
1: Repeated start generation
In Slave mode:
0: No Start generation
1: Start generation when the bus is free

**Bit 7 NOSTRETCH**: Clock stretching disable (Slave mode)
This bit is used to disable clock stretching in slave mode when ADDR or BTF flag is set, until it is reset by software.
0: Clock stretching enabled
1: Clock stretching disabled

**Bit 6 ENGC**: General call enable
0: General call disabled. Address 00h is NACKed.
1: General call enabled. Address 00h is ACKed.

**Bit 5 ENPEC**: PEC enable
0: PEC calculation disabled
1: PEC calculation enabled

**Bit 4 ENARP**: ARP enable
0: ARP disable
1: ARP enable
SMBus Device default address recognized if SMBTYPE=0
SMBus Host address recognized if SMBTYPE=1

**Bit 3 SMBTYPE**: SMBus type
0: SMBus Device
1: SMBus Host

STI
RM0090 Rev 18
861/1749

--- Page 82 ---

Inter-integrated circuit (I2C) interface
RM0090

Bit 2 Reserved, must be kept at reset value
Bit 1 **SMBUS**: SMBus mode
0: I²C mode
1: SMBus mode
Bit 0 **PE**: Peripheral enable
0: Peripheral disable
1: Peripheral enable
*Note: If this bit is reset while a communication is on going, the peripheral is disabled at the end of the current communication, when back to IDLE state. All bit resets due to PE=0 occur at the end of the communication. In master mode, this bit must not be reset before the end of the communication.*

*Note: When the STOP, START or PEC bit is set, the software must not perform any write access to I2C_CR1 before this bit is cleared by hardware. Otherwise there is a risk of setting a second STOP, START or PEC request.*

### 27.6.2 I²C Control register 2 (I2C_CR2)

Address offset: 0x04
Reset value: 0x0000

| 15 14 13 | 12 | 11 | 10 | 9 | 8 | 7 6 | 5 4 3 2 1 0 |
|---|---|---|---|---|---|---|---|
| Reserved | LAST | DMAEN | ITBUFEN | ITEVTEN | ITERREN | Reserved | FREQ[5:0] |
|  |  |  | rw |  |  | rw |  |


Bits 15:13 Reserved, must be kept at reset value
Bit 12 **LAST**: DMA last transfer
0: Next DMA EOT is not the last transfer
1: Next DMA EOT is the last transfer
*Note: This bit is used in master receiver mode to permit the generation of a NACK on the last received data.*
Bit 11 **DMAEN**: DMA requests enable
0: DMA requests disabled
1: DMA request enabled when TxE=1 or RxNE =1
Bit 10 **ITBUFEN**: Buffer interrupt enable
0: TxE = 1 or RxNE = 1 does not generate any interrupt.
1: TxE = 1 or RxNE = 1 generates Event Interrupt (whatever the state of DMAEN)

862/1749
RM0090 Rev 18
ST

--- Page 83 ---

RM0090
Inter-integrated circuit (I2C) interface

Bit 9 **ITEVTEN**: Event interrupt enable
0: Event interrupt disabled
1: Event interrupt enabled
This interrupt is generated when:
- SB = 1 (Master)
- ADDR = 1 (Master/Slave)
- ADD10= 1 (Master)
- STOPF = 1 (Slave)
- BTF = 1 with no TxE or RxNE event
- TxE event to 1 if ITBUFEN = 1
- RxNE event to 1 if ITBUFEN = 1

Bit 8 **ITERREN**: Error interrupt enable
0: Error interrupt disabled
1: Error interrupt enabled
This interrupt is generated when:
- BERR = 1
- ARLO = 1
- AF = 1
- OVR = 1
- PECERR = 1
- TIMEOUT = 1
- SMBALERT = 1

Bits 7:6 Reserved, must be kept at reset value

Bits 5:0 **FREQ[5:0]**: Peripheral clock frequency
The FREQ bits must be configured with the APB clock frequency value (I2C peripheral connected to APB). The FREQ field is used by the peripheral to generate data setup and hold times compliant with the I2C specifications. The minimum allowed frequency is 2 MHz, the maximum frequency is limited by the maximum APB frequency and cannot exceed 50 MHz (peripheral intrinsic maximum limit).
0b000000: Not allowed
0b000001: Not allowed
0b000010: 2 MHz
...
0b110010: 50 MHz
Higher than 0b100100: Not allowed

ST
RM0090 Rev 18
863/1749

--- Page 84 ---

Inter-integrated circuit (I2C) interface
RM0090

### 27.6.3 I²C Own address register 1 (I2C_OAR1)

Address offset: 0x08
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ADD MODE | Reserved | ADD[9:8] | ADD[7:1] | ADD0 |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bit 15 **ADDMODE**: Addressing mode (slave mode)
0: 7-bit slave address (10-bit address not acknowledged)
1: 10-bit slave address (7-bit address not acknowledged)

Bit 14 Should always be kept at 1 by software.

Bits 13:10 Reserved, must be kept at reset value

Bits 9:8 **ADD[9:8]**: Interface address
7-bit addressing mode: don't care
10-bit addressing mode: bits9:8 of address

Bits 7:1 **ADD[7:1]**: Interface address
bits 7:1 of address

Bit 0 **ADD0**: Interface address
7-bit addressing mode: don't care
10-bit addressing mode: bit 0 of address

### 27.6.4 I²C Own address register 2 (I2C_OAR2)

Address offset: 0x0C
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ADD2[7:1] | ENDUAL |  |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Bits 15:8 Reserved, must be kept at reset value

Bits 7:1 **ADD2[7:1]**: Interface address
bits 7:1 of address in dual addressing mode

Bit 0 **ENDUAL**: Dual addressing mode enable
0: Only OAR1 is recognized in 7-bit addressing mode
1: Both OAR1 and OAR2 are recognized in 7-bit addressing mode

864/1749
RM0090 Rev 18
ST

--- Page 85 ---

RM0090
Inter-integrated circuit (I2C) interface

## 27.6.5 I²C Data register (I2C_DR)

Address offset: 0x10
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | DR[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | rw |  |  |  |  |  |  |  |


Bits 15:8 Reserved, must be kept at reset value
Bits 7:0 **DR[7:0]** 8-bit data register
Byte received or to be transmitted to the bus.
– Transmitter mode: Byte transmission starts automatically when a byte is written in the DR register. A continuous transmit stream can be maintained if the next data to be transmitted is put in DR once the transmission is started (TxE=1)
– Receiver mode: Received byte is copied into DR (RxNE=1). A continuous transmit stream can be maintained if DR is read before the next data byte is received (RxNE=1).
*Note: In slave mode, the address is not copied into DR.*
Write collision is not managed (DR can be written if TxE=0).
If an ARLO event occurs on ACK pulse, the received byte is not copied into DR and so cannot be read.

## 27.6.6 I²C Status register 1 (I2C_SR1)

Address offset: 0x14
Reset value: 0x0000

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| SMBALERT | TIMEOUT | Res. | PECERR | OVR | AF | ARLO | BERR | TxE | RxNE | Res. | STOPF | ADD10 | BTF | ADDR | SB |
| rc_w0 |  | rc_w0 | r |  | r |  |  |  |  |  |  |  |  |  |  |


Bit 15 **SMBALERT**: SMBus alert
In SMBus host mode:
0: no SMBALERT
1: SMBALERT event occurred on pin
In SMBus slave mode:
0: no SMBALERT response address header
1: SMBALERT response address header to SMBALERT LOW received
– Cleared by software writing 0, or by hardware when PE=0.

ST
RM0090 Rev 18
865/1749

--- Page 86 ---

**Inter-integrated circuit (I2C) interface** RM0090

**Bit 14 TIMEOUT**: Timeout or Tlow error
0: No timeout error
1: SCL remained LOW for 25 ms (Timeout)
or
Master cumulative clock low extend time more than 10 ms (Tlow:next)
or
Slave cumulative clock low extend time more than 25 ms (Tlow:sext)
– When set in slave mode: slave resets the communication and lines are released by hardware
– When set in master mode: Stop condition sent by hardware
– Cleared by software writing 0, or by hardware when PE=0.
*Note: This functionality is available only in SMBus mode.*

**Bit 13** Reserved, must be kept at reset value

**Bit 12 PECERR**: PEC Error in reception
0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)
1: PEC error: receiver returns NACK after PEC reception (whatever ACK)
– Cleared by software writing 0, or by hardware when PE=0.
*Note: When the received CRC is wrong, PECERR is not set in slave mode if the PEC control bit is not set before the end of the CRC reception. Nevertheless, reading the PEC value determines whether the received CRC is right or wrong.*

**Bit 11 OVR**: Overrun/Underrun
0: No overrun/underrun
1: Overrun or underrun
– Set by hardware in slave mode when NOSTRETCH=1 and:
– In reception when a new byte is received (including ACK pulse) and the DR register has not been read yet. New received byte is lost.
– In transmission when a new byte should be sent and the DR register has not been written yet. The same byte is sent twice.
– Cleared by software writing 0, or by hardware when PE=0.
*Note: If the DR write occurs very close to SCL rising edge, the sent data is unspecified and a hold timing error occurs*

**Bit 10 AF**: Acknowledge failure
0: No acknowledge failure
1: Acknowledge failure
– Set by hardware when no acknowledge is returned.
– Cleared by software writing 0, or by hardware when PE=0.

**Bit 9 ARLO**: Arbitration lost (master mode)
0: No Arbitration Lost detected
1: Arbitration Lost detected
Set by hardware when the interface loses the arbitration of the bus to another master
– Cleared by software writing 0, or by hardware when PE=0.
After an ARLO event the interface switches back automatically to Slave mode (MSL=0).
*Note: In SMBUS, the arbitration on the data in slave mode occurs only during the data phase, or the acknowledge transmission (not on the address acknowledge).*

866/1749 RM0090 Rev 18 ST

--- Page 87 ---

RM0090
Inter-integrated circuit (I2C) interface

**Bit 8 BERR**: Bus error
0: No misplaced Start or Stop condition
1: Misplaced Start or Stop condition
– Set by hardware when the interface detects an SDA rising or falling edge while SCL is high, occurring in a non-valid position during a byte transfer.
– Cleared by software writing 0, or by hardware when PE=0.

**Bit 7 TxE**: Data register empty (transmitters)
0: Data register not empty
1: Data register empty
– Set when DR is empty in transmission. TxE is not set during address phase.
– Cleared by software writing to the DR register or by hardware after a start or a stop condition or when PE=0.
TxE is not set if either a NACK is received, or if next byte to be transmitted is PEC (PEC=1)
*Note: TxE is not cleared by writing the first data being transmitted, or by writing data when BTF is set, as in both cases the data register is still empty.*

**Bit 6 RxNE**: Data register not empty (receivers)
0: Data register empty
1: Data register not empty
– Set when data register is not empty in receiver mode. RxNE is not set during address phase.
– Cleared by software reading or writing the DR register or by hardware when PE=0.
RxNE is not set in case of ARLO event.
*Note: RxNE is not cleared by reading data when BTF is set, as the data register is still full.*

**Bit 5** Reserved, must be kept at reset value

**Bit 4 STOPF**: Stop detection (slave mode)
0: No Stop condition detected
1: Stop condition detected
– Set by hardware when a Stop condition is detected on the bus by the slave after an acknowledge (if ACK=1).
– Cleared by software reading the SR1 register followed by a write in the CR1 register, or by hardware when PE=0
*Note: The STOPF bit is not set after a NACK reception.*
*It is recommended to perform the complete clearing sequence (READ SR1 then WRITE CR1) after the STOPF is set. Refer to Figure 242.*

**Bit 3 ADD10**: 10-bit header sent (Master mode)
0: No ADD10 event occurred.
1: Master has sent first address byte (header).
– Set by hardware when the master has sent the first byte in 10-bit address mode.
– Cleared by software reading the SR1 register followed by a write in the DR register of the second address byte, or by hardware when PE=0.
*Note: ADD10 bit is not set after a NACK reception*

STI
RM0090 Rev 18
867/1749

--- Page 88 ---

**Inter-integrated circuit (I2C) interface**
RM0090

**Bit 2 BTF:** Byte transfer finished
0: Data byte transfer not done
1: Data byte transfer succeeded
– Set by hardware when NOSTRETCH=0 and:
– In reception when a new byte is received (including ACK pulse) and DR has not been read yet (RxNE=1).
– In transmission when a new byte should be sent and DR has not been written yet (TxE=1).
– Cleared by software by either a read or write in the DR register or by hardware after a start or a stop condition in transmission or when PE=0.
*Note: The BTF bit is not set after a NACK reception*
*The BTF bit is not set if next byte to be transmitted is the PEC (TRA=1 in I2C_SR2 register and PEC=1 in I2C_CR1 register)*

**Bit 1 ADDR:** Address sent (master mode)/matched (slave mode)
This bit is cleared by software reading SR1 register followed reading SR2, or by hardware when PE=0.
Address matched (Slave)
0: Address mismatched or not received.
1: Received address matched.
– Set by hardware as soon as the received slave address matched with the OAR registers content or a general call or a SMBus Device Default Address or SMBus Host or SMBus Alert is recognized. (when enabled depending on configuration).
*Note: In slave mode, it is recommended to perform the complete clearing sequence (READ SR1 then READ SR2) after ADDR is set. Refer to Figure 242.*
Address sent (Master)
0: No end of address transmission
1: End of address transmission
– For 10-bit addressing, the bit is set after the ACK of the 2nd byte.
– For 7-bit addressing, the bit is set after the ACK of the byte.
*Note: ADDR is not set after a NACK reception*

**Bit 0 SB:** Start bit (Master mode)
0: No Start condition
1: Start condition generated.
– Set when a Start condition generated.
– Cleared by software by reading the SR1 register followed by writing the DR register, or by hardware when PE=0

**27.6.7 I²C Status register 2 (I2C_SR2)**

Address offset: 0x18
Reset value: 0x0000

*Note:*
*Reading I2C_SR2 after reading I2C_SR1 clears the ADDR flag, even if the ADDR flag was set after reading I2C_SR1. Consequently, I2C_SR2 must be read only when ADDR is found set in I2C_SR1 or when the STOPF bit is cleared.*

| PEC[7:0] | DUALF | SMB HOST | SMBDE FAULT | GEN CALL | Res. | TRA | BUSY | MSL |
|---|---|---|---|---|---|---|---|---|
| r |  |  | r |  |  |  |  |  |


868/1749
RM0090 Rev 18
ST

--- Page 89 ---

**RM0090**
**Inter-integrated circuit (I2C) interface**

Bits 15:8 **PEC[7:0]** Packet error checking register
This register contains the internal PEC when ENPEC=1.

Bit 7 **DUALF**: Dual flag (Slave mode)
0: Received address matched with OAR1
1: Received address matched with OAR2
– Cleared by hardware after a Stop condition or repeated Start condition, or when PE=0.

Bit 6 **SMBHOST**: SMBus host header (Slave mode)
0: No SMBus Host address
1: SMBus Host address received when SMBTYPE=1 and ENARP=1.
– Cleared by hardware after a Stop condition or repeated Start condition, or when PE=0.

Bit 5 **SMBDEFAULT**: SMBus device default address (Slave mode)
0: No SMBus Device Default address
1: SMBus Device Default address received when ENARP=1
– Cleared by hardware after a Stop condition or repeated Start condition, or when PE=0.

Bit 4 **GENCALL**: General call address (Slave mode)
0: No General Call
1: General Call Address received when ENGC=1
– Cleared by hardware after a Stop condition or repeated Start condition, or when PE=0.

Bit 3 Reserved, must be kept at reset value

Bit 2 **TRA**: Transmitter/receiver
0: Data bytes received
1: Data bytes transmitted
This bit is set depending on the R/W bit of the address byte, at the end of total address phase.
It is also cleared by hardware after detection of Stop condition (STOPF=1), repeated Start condition, loss of bus arbitration (ARLO=1), or when PE=0.

Bit 1 **BUSY**: Bus busy
0: No communication on the bus
1: Communication ongoing on the bus
– Set by hardware on detection of SDA or SCL low
– cleared by hardware on detection of a Stop condition.
It indicates a communication in progress on the bus. This information is still updated when the interface is disabled (PE=0).

Bit 0 **MSL**: Master/slave
0: Slave Mode
1: Master Mode
– Set by hardware as soon as the interface is in Master mode (SB=1).
– Cleared by hardware after detecting a Stop condition on the bus or a loss of arbitration (ARLO=1), or by hardware when PE=0.

*Note:* Reading I2C_SR2 after reading I2C_SR1 clears the ADDR flag, even if the ADDR flag was set after reading I2C_SR1. Consequently, I2C_SR2 must be read only when ADDR is found set in I2C_SR1 or when the STOPF bit is cleared.

ST
RM0090 Rev 18
869/1749

--- Page 90 ---

**Inter-integrated circuit (I2C) interface**
RM0090

**27.6.8 I²C Clock control register (I2C_CCR)**

Address offset: 0x1C
Reset value: 0x0000

*Note:* fPCLK1 must be at least 2 MHz to achieve Sm mode I²C frequencies. It must be at least 4 MHz to achieve Fm mode I²C frequencies. It must be a multiple of 10MHz to reach the 400 kHz maximum I²C Fm mode clock.

The CCR register must be configured only when the I2C is disabled (PE = 0).

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| F/S | DUTY | Reserved | CCR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |
| rw |  |  | rw |  |  |  |  |  |  |  |  |  |  |  |  |


**Bit 15 F/S:** I2C master mode selection
0: Sm mode I2C
1: Fm mode I2C

**Bit 14 DUTY:** Fm mode duty cycle
0: Fm mode tlow/thigh = 2
1: Fm mode tlow/thigh = 16/9 (see CCR)

**Bits 13:12** Reserved, must be kept at reset value

**Bits 11:0 CCR[11:0]:** Clock control register in Fm/Sm mode (Master mode)
Controls the SCL clock in master mode.

*Sm mode or SMBus:*
T_high = CCR * T_PCLK1
T_low = CCR * T_PCLK1

*Fm mode:*
If DUTY = 0:
T_high = CCR * T_PCLK1
T_low = 2 * CCR * T_PCLK1

If DUTY = 1: (to reach 400 kHz)
T_high = 9 * CCR * T_PCLK1
T_low = 16 * CCR * T_PCLK1

For instance: in Sm mode, to generate a 100 kHz SCL frequency:
If FREQR = 08, T_PCLK1 = 125 ns so CCR must be programmed with 0x28
(0x28 <=> 40d x 125 ns = 5000 ns.)

*Note:* The minimum allowed value is 0x04, except in FAST DUTY mode where the minimum allowed value is 0x01

t_high = t_r(SCL) + t_w(SCLH). See device datasheet for the definitions of parameters.
t_low = t_f(SCL) + t_w(SCLL). See device datasheet for the definitions of parameters.
I2C communication speed, fSCL ~ 1/(thigh + tlow). The real frequency may differ due to the analog noise filter input delay.

The CCR register must be configured only when the I²C is disabled (PE = 0).

870/1749
RM0090 Rev 18

--- Page 91 ---

RM0090
Inter-integrated circuit (I2C) interface

### 27.6.9 I²C TRISE register (I2C_TRISE)

Address offset: 0x20
Reset value: 0x0002

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | TRISE[5:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |  |


Bits 15:6 Reserved, must be kept at reset value
Bits 5:0 **TRISE[5:0]**: Maximum rise time in Fm/Sm mode (Master mode)
These bits should provide the maximum duration of the SCL feedback loop in master mode.
The purpose is to keep a stable SCL frequency whatever the SCL rising edge duration.
These bits must be programmed with the maximum SCL rise time given in the I²C bus specification, incremented by 1.
For instance: in Sm mode, the maximum allowed SCL rise time is 1000 ns.
If, in the I2C_CR2 register, the value of FREQ[5:0] bits is equal to 0x08 and T_PCLK1 = 125 ns therefore the TRISE[5:0] bits must be programmed with 09h.
(1000 ns / 125 ns = 8 + 1)
The filter value can also be added to TRISE[5:0].
If the result is not an integer, TRISE[5:0] must be programmed with the integer part, in order to respect the t_HIGH parameter.
*Note: TRISE[5:0] must be configured only when the I2C is disabled (PE = 0).*

### 27.6.10 I²C FLTR register (I2C_FLTR)

Address offset: 0x24
Reset value: 0x0000
The I2C_FLTR is available on STM32F42xxx and STM32F43xxx only.

| 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Reserved | ANOFF | DNF[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  | rw |  |  |  |  |


Bits 15:5 Reserved, must be kept at reset value
Bit 4 **ANOFF**: Analog noise filter OFF
0: Analog noise filter enable
1: Analog noise filter disable
*Note: ANOFF must be configured only when the I2C is disabled (PE = 0).*
Bits 3:0 **DNF[3:0]**: Digital noise filter
These bits are used to configure the digital noise filter on SDA and SCL inputs. The digital filter will suppress the spikes with a length of up to DNF[3:0] * TPCLK1.
0000: Digital noise filter disable
0001: Digital noise filter enabled and filtering capability up to 1* TPCLK1.
...
1111: Digital noise filter enabled and filtering capability up to 15* TPCLK1.
*Note: DNF[3:0] must be configured only when the I2C is disabled (PE = 0). If the analog filter is also enabled, the digital filter is added to the analog filter.*

ST
RM0090 Rev 18
871/1749

--- Page 92 ---

27.6.11
I²C register map

The table below provides the I²C register map and reset values.

Table 125. I²C register map and reset values

| Offset | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0x00 | I2C_CR1 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | SWRST | Reserved | ALERT | PEC | POS | ACK | STOP | NOSTRETCH | ENGC | ENPEC | ENARP | SMBTYPE | Reserved | SMBUS | PE |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x04 | I2C_CR2 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | LAST | DMAEN | PEC | ITBUFEN | ITEVTEN | ITERREN | Reserved | FREQ[5:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x08 | I2C_OAR1 | Reserved | 0 | ADDMODE | ADD[9:8] | ADD[7:1] | ADD0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x0C | I2C_OAR2 | Reserved | 0 | ENDUAL | ADD2[7:1] | DR[7:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x10 | I2C_DR | Reserved | 0 | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | Reserved |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x14 | I2C_SR1 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | SMBALERT | TIMEOUT | PECERR | OVR | AF | ARLO | BERR | TxE | RxNE | STOPF | ADD10 | BTF | ADDR | SB |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x18 | I2C_SR2 | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | PEC[7:0] | DUALF | SMBHOST | SMBDEFALT | GENCALL | TRA | Reserved | BUSY | MSL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x1C | I2C_CCR | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | F/S | DUTY | Reserved | CCR[11:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x20 | I2C_TRISE | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | TRISE[5:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0x24 | I2C_FLTR | Reserved | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | ANOFF | DNF[3:0] |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


Refer to Section 2.3: Memory map for the register boundary addresses table.

RM0090 Rev 18
872/1749

--- Page 93 ---

RM0090
Serial peripheral interface (SPI)

