> [RM0090 ch.27 §27.3] I²C functional description


### Unique device identifier (UDID)

In order to provide a mechanism to isolate each device for the purpose of address assignment, each device must implement a unique device identifier (UDID).

854/1749
RM0090 Rev 18
STI

--- Page 75 ---

RM0090
Inter-integrated circuit (I2C) interface

For the details on 128-bit UDID and more information on ARP, refer to SMBus specification version 2.0.

**SMBus alert mode**

SMBus Alert is an optional signal with an interrupt line for devices that want to trade their ability to master for a pin. SMBA is a wired-AND signal just as the SCL and SDA signals are. SMBA is used in conjunction with the SMBus General Call Address. Messages invoked with the SMBus are two bytes long.

A slave-only device can signal the host through SMBA that it wants to talk by setting ALERT bit in I2C_CR1 register. The host processes the interrupt and simultaneously accesses all SMBA devices through the *Alert Response Address* (known as ARA having a value 0001 100X). Only the device(s) which pulled SMBA low will acknowledge the Alert Response Address. This status is identified using SMBALERT Status flag in I2C_SR1 register. The host performs a modified Receive Byte operation. The 7 bit device address provided by the slave transmit device is placed in the 7 most significant bits of the byte. The eighth bit can be a zero or one.

If more than one device pulls SMBA low, the highest priority (lowest address) device will win communication rights via standard arbitration during the slave address transfer. After acknowledging the slave address the device must disengage its SMBA pull-down. If the host still sees SMBA low when the message transfer is complete, it knows to read the ARA again.

A host which does not implement the SMBA signal may periodically access the ARA.

For more details on SMBus Alert mode, refer to SMBus specification version 2.0.

**Timeout error**

There are differences in the timing specifications between I²C and SMBus. SMBus defines a clock low timeout, TIMEOUT of 35 ms. Also SMBus specifies TLOW: SEXT as the cumulative clock low extend time for a slave device. SMBus specifies TLOW: MEXT as the cumulative clock low extend time for a master device. For more details on these timeouts, refer to SMBus specification version 2.0.

The status flag Timeout or Tlow Error in I2C_SR1 shows the status of this feature.

**How to use the interface in SMBus mode**

To switch from I²C mode to SMBus mode, the following sequence should be performed.

*   Set the SMBus bit in the I2C_CR1 register
*   Configure the SMBTYPE and ENARP bits in the I2C_CR1 register as required for the application

If you want to configure the device as a master, follow the Start condition generation procedure in Section 27.3.3. Otherwise, follow the sequence in Section 27.3.2.

The application has to control the various SMBus protocols by software.

*   SMB Device Default Address acknowledged if ENARP=1 and SMBTYPE=0
*   SMB Host Header acknowledged if ENARP=1 and SMBTYPE=1
*   SMB Alert Response Address acknowledged if SMBALERT=1

ST
RM0090 Rev 18
855/1749

--- Page 76 ---

Inter-integrated circuit (I2C) interface RM0090

