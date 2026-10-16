**2.4 I2C peripheral limitations**

**2.4.1 SMBus standard not fully supported**

**Description**

The I 2 C peripheral is not fully compliant with the SMBus v2.0 standard since It does not support the capability to NACK an invalid byte/command.

**Workarounds**

A higher-level mechanism should be used to verify that a write operation is being performed correctly at the target device, such as:

1. Using the SMBAL pin if supported by the host

2. the alert response address (ARA) protocol

3. the Host notify protocol

**2.4.2 Start cannot be generated after a misplaced Stop**

**Description**

If a master generates a misplaced Stop on the bus (bus error), the peripheral cannot generate a Start anymore.

**Workaround**

In the I²C standard, it is allowed to send a Stop only at the end of the full byte (8 bits + acknowledge), so this scenario is not allowed. Other derived protocols like CBUS allow it, but they are not supported by the I²C peripheral.

A software workaround consists in asserting the software reset using the SWRST bit in the I2C_CR1 control register.

**2.4.3 Mismatch on the “Setup time for a repeated Start condition” timing parameter**

**Description**

In case of a repeated Start, the “Setup time for a repeated Start condition” (named Tsu;sta in the I²C specification) can be slightly violated when the I²C operates in Master Standard mode at a frequency between 88 kHz and 100 kHz.



37

--- Page 18 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

The limitation can occur only in the following configuration:

• in Master mode

• in Standard mode at a frequency between 88 kHz and 100 kHz (no limitation in Fast-

mode)

• SCL rise time:

– If the slave does not stretch the clock and the SCL rise time is more than 300 ns (if

the SCL rise time is less than 300 ns, the limitation cannot occur)

– If the slave stretches the clock

The setup time can be violated independently of the APB peripheral frequency.

**Workaround**

Reduce the frequency down to 88 kHz or use the I²C Fast-mode, if supported by the slave.

**2.4.4 Data valid time (t VD;DAT ) violated without the OVR flag being set**

**Description**

The data valid time (t VD;DAT , t VD;ACK ) described by the I²C standard can be violated (as well as the maximum data hold time of the current data (t HD;DAT )) under the conditions described below. This violation cannot be detected because the OVR flag is not set (no transmit buffer underrun is detected).

This limitation can occur only under the following conditions:

• in Slave transmit mode

• with clock stretching disabled (NOSTRETCH=1)

• if the software is late to write the DR data register, but not late enough to set the OVR

flag (the data register is written before)

**Workaround**

If the master device allows it, use the clock stretching mechanism by programming the bit NOSTRETCH=0 in the I2C_CR1 register.

If the master device does not allow it, ensure that the software is fast enough when polling the TXE or ADDR flag to immediately write to the DR data register. For instance, use an interrupt on the TXE or ADDR flag and boost its priority to the higher level.



--- Page 19 ---

**STM32F40x and STM32F41x STM32F40x and STM32F41x silicon limitations**

**2.4.5 Both SDA and SCL maximum rise time (t r ) violated when VDD_I2C bus higher than ((VDD+0.3) / 0.7) V**

**Description**

When an external legacy I 2 C bus voltage (VDD_I2C) is set to 5 V while the MCU is powered from V DD , the internal 5-Volt tolerant circuitry is activated as soon the input voltage (V IN ) reaches the V DD + diode threshold level. An additional internal large capacitance then prevents the external pull-up resistor (R P ) from rising the SDA and SCL signals within the maximum timing (t r ) which is 300 ns in fast mode and 1000 ns in Standard mode.

The rise time (t r ) is measured from V IL and V IH with levels set at 0.3VDD_I2C and 0.7VDD_I2C.

**Workaround**

The external VDD_I2C bus voltage should be limited to a maximum value of ((VDD+0.3) / 0.7) V. As a result, when the MCU is powered from V DD =3.3 V, VDD_I2C should not exceed 5.14 V to be compliant with I 2 C specifications.

**2.4.6 Spurious Bus Error detection in master mode**

**Description**

In master mode, a bus error can be detected by mistake, so the BERR flag can be wrongly raised in the status register. This will generate a spurious Bus Error interrupt if the interrupt is enabled. A bus error detection has no effect on the transfer in master mode, therefore the I2C transfer can continue normally.

**Workaround**

If a bus error interrupt is generated in master mode, the BERR flag must be cleared by software. No other action is required and the on-going transfer can be handled normally.

**2.4.7 Wrong behavior related with MCU Stop mode when wakeup from Stop mode by I2C peripheral is disabled**

**Description**

When wakeup from Stop mode by I2C peripheral is disabled (WUPEN = 0) and the MCU enters Stop mode while a transaction is on-going on the I²C bus, the following wrong operation may occur: 1. BUSY flag may be wrongly set when the MCU exits Stop mode. This prevents from initiating a transfer in master mode, as the START condition cannot be sent when BUSY is set. This failure may occur in master mode of the I2C peripheral used in multi-master I²C-bus environment. 2. If I²C-bus clock stretching is enabled in I2C peripheral (NOSTRETCH = 0), the I2C peripheral may pull SCL low as long as the MCU remains in Stop mode, suspending all I²C-bus activity during that time. This may occur when the MCU enters Stop mode during the address phase of an I²C-bus transaction, in low period of SCL. This failure may occur in slave mode of the I2C peripheral or, in master mode of the I2C peripheral used in multi-master I²C-bus environment. Its probability depends on the timing configuration, operating clock frequency of I2C peripheral and the I²C-bus timing.



37

--- Page 20 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

**Workaround**

Disable the I2C peripheral (PE=0) before entering Stop mode and re-enable it in Run mode.

**2.5 I2S peripheral limitations**

**2.5.1 In I2S slave mode, WS level must be set by the external master when enabling the I2S**

**Description**

In slave mode, the WS signal level is used only to start the communication. If the I2S (in slave mode) is enabled while the master is already sending the clock and the WS signal level is low (for I2S protocol) or is high (for the LSB or MSB-justified mode), the slave starts communicating data immediately. In this case, the master and slave will be desynchronized throughout the whole communication.

**Workaround**

The I2S peripheral must be enabled when the external master sets the WS line at:

• High level when the I2S protocol is selected.

• Low level when the LSB or MSB-justified mode is selected.

**2.5.2 I2S2 in full-duplex mode may not work properly when SCK and WS signals are mapped on PI1 and PI0 respectively**

**Description**

When SCK and WS signals are used to support I2S full-duplex through GPIO port I: PI1 and PI0 respectively, the I2S2 peripheral cannot be able to provide internally SCK signal and WS signal to I2S2_ext interface. In this case, I2S2_ext interface will not be able to send/receive data.

**Workaround**

Other mapped pins for SCK and WS signals can be used on GPIO Port B as below:

• I2S2 CK signal: PB10 pin or PB13 pin.

• I2S2 WS signal: PB12 pin or PB9 pin.

