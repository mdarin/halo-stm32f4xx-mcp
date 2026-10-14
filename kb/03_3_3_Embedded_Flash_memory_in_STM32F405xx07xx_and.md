**3.3 Embedded Flash memory in STM32F405xx/07xx and STM32F415xx/17xx**

The Flash memory has the following main features:

• Capacity up to 1 Mbyte

• 128 bits wide data read

• Byte, half-word, word and double word write

• Sector and mass erase

• Memory organization

The Flash memory is organized as follows:

–

A main memory block divided into 4 sectors of 16 Kbytes, 1 sector of 64 Kbytes,

and 7 sectors of 128 Kbytes

–

System memory from which the device boots in System memory boot mode

– 512 OTP (one-time programmable) bytes for user data

The OTP area contains 16 additional bytes used to lock the corresponding OTP

data block .

–

Option bytes to configure read and write protection, BOR level, watchdog software/hardware and reset when the device is in Standby or Stop mode. Low-power modes (for details refer to the Power control (PWR) section of the reference

•

manual)



--- Page 75 ---

**RM0090 Embedded Flash memory interface**

**Table 5. Flash module organization (STM32F40x and STM32F41x)**

| Block | Name | Block base addresses | Size |
|---|---|---|---|
| Main memory | Sector 0 | 0x0800 0000 - 0x0800 3FFF | 16 Kbytes |
| Sector 1 | 0x0800 4000 - 0x0800 7FFF | 16 Kbytes |  |
| Sector 2 | 0x0800 8000 - 0x0800 BFFF | 16 Kbytes |  |
| Sector 3 | 0x0800 C000 - 0x0800 FFFF | 16 Kbytes |  |
| Sector 4 | 0x0801 0000 - 0x0801 FFFF | 64 Kbytes |  |
| Sector 5 | 0x0802 0000 - 0x0803 FFFF | 128 Kbytes |  |
| Sector 6 | 0x0804 0000 - 0x0805 FFFF | 128 Kbytes |  |
| . . . |  |  |  |
| Sector 11 | 0x080E 0000 - 0x080F FFFF | 128 Kbytes |  |
| System memory | 0x1FFF 0000 - 0x1FFF 77FF | 30 Kbytes |  |
| OTP area | 0x1FFF 7800 - 0x1FFF 7A0F | 528 bytes |  |
| Option bytes | 0x1FFF C000 - 0x1FFF C00F | 16 bytes |  |




112

--- Page 76 ---

**Embedded Flash memory interface RM0090**

