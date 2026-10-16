**3.4 Embedded Flash memory in STM32F42xxx and STM32F43xxx**

The Flash memory has the following main features:

• Capacity up to 2 Mbyte with dual bank architecture supporting read-while-write capability (RWW)

• 128 bits wide data read

• Byte, half-word, word and double word write

• Sector, bank, and mass erase (both banks)

• Dual bank memory organization

The Flash memory is organized as follows:

– For each bank, a main memory block (1 Mbyte) divided into 4 sectors of 16

Kbytes, 1 sector of 64 Kbytes, and 7 sectors of 128 Kbytes

– System memory from which the device boots in System memory boot mode

– 512 OTP (one-time programmable) bytes for user data

The OTP area contains 16 additional bytes used to lock the corresponding OTP data block .

– Option bytes to configure read and write protection, BOR level, watchdog, dual

bank boot mode, dual bank feature, software/hardware and reset when the device is in Standby or Stop mode.

• Dual bank organization on 1 Mbyte devices

The dual bank feature on 1 Mbyte devices is enabled by setting the DB1M option bit. To obtain a dual bank Flash memory, the last 512 Kbytes of the single bank (sectors [8:11]) are re-structured in the same way as the first 512 Kbytes.

The sector numbering of dual bank memory organization is different from the single bank: the single bank memory contains 12 sectors whereas the dual bank memory contains 16 sectors (see Table 7: 1 Mbyte Flash memory single bank vs dual bank organization   (STM32F42xxx and STM32F43xxx) ).

For erase operation, the right sector numbering must be considered according the DB1M option bit.

– When the DB1M bit is reset, the erase operation must be performed on the default

sector number.

– When the DB1M bit is set, to perform an erase operation on bank 2, the sector

number must be programmed (sector number from 12 to 19). Refer to FLASH_CR register for SNB (Sector number) configuration.

Refer to Table 8: 1 Mbyte single bank Flash memory organization  (STM32F42xxx and STM32F43xxx) and Table 9: 1 Mbyte dual bank Flash memory organization (STM32F42xxx and STM32F43xxx) for details on 1 Mbyte single bank and 1 Mbyte dual bank organizations.



--- Page 77 ---

**RM0090 Embedded Flash memory interface**

**Table 6. Flash module - 2 Mbyte dual bank organization (STM32F42xxx and STM32F43xxx)**

| Block | Bank | Name | Block base addresses | Size |
|---|---|---|---|---|
| Main memory | Bank 1 | Sector 0 | 0x0800 0000 - 0x0800 3FFF | 16 Kbytes |
| Sector 1 | 0x0800 4000 - 0x0800 7FFF | 16 Kbytes |  |  |
| Sector 2 | 0x0800 8000 - 0x0800 BFFF | 16 Kbytes |  |  |
| Sector 3 | 0x0800 C000 - 0x0800 FFFF | 16 Kbyte |  |  |
| Sector 4 | 0x0801 0000 - 0x0801 FFFF | 64 Kbytes |  |  |
| Sector 5 | 0x0802 0000 - 0x0803 FFFF | 128 Kbytes |  |  |
| Sector 6 | 0x0804 0000 - 0x0805 FFFF | 128 Kbytes |  |  |
| - |  |  |  |  |
| - |  |  |  |  |
| - |  |  |  |  |
| Sector 11 | 0x080E 0000 - 0x080F FFFF | 128 Kbytes |  |  |
| Bank 2 | Sector 12 | 0x0810 0000 - 0x0810 3FFF | 16 Kbytes |  |
| Sector 13 | 0x0810 4000 - 0x0810 7FFF | 16 Kbytes |  |  |
| Sector 14 | 0x0810 8000 - 0x0810 BFFF | 16 Kbytes |  |  |
| Sector 15 | 0x0810 C000 - 0x0810 FFFF | 16 Kbytes |  |  |
| Sector 16 | 0x0811 0000 - 0x0811 FFFF | 64 Kbytes |  |  |
| Sector 17 | 0x0812 0000 - 0x0813 FFFF | 128 Kbytes |  |  |
| Sector 18 | 0x0814 0000 - 0x0815 FFFF | 128 Kbytes |  |  |
|  | - |  |  |  |
|  | - |  |  |  |
|  | - |  |  |  |
| Sector 23 | 0x081E 0000 - 0x081F FFFF | 128 Kbytes |  |  |
| System memory | 0x1FFF 0000 - 0x1FFF 77FF | 30 Kbytes |  |  |
| OTP | 0x1FFF 7800 - 0x1FFF 7A0F | 528 bytes |  |  |
| Option bytes | Bank 1 |  | 0x1FFF C000 - 0x1FFF C00F | 16 bytes |
| Bank 2 |  | 0x1FFE C000 - 0x1FFE C00F | 16 bytes |  |




112

--- Page 78 ---

**Embedded Flash memory interface RM0090**

**Table 7. 1 Mbyte Flash memory single bank vs dual bank organization**

**(STM32F42xxx and STM32F43xxx)**

| 1 Mbyte single bank Flash memory (default) | 1 Mbyte dual bank Flash memory |  |  |  |  |
|---|---|---|---|---|---|
| DB1M=0 | DB1M=1 |  |  |  |  |
| Main memory | Sector number | Sector size | Main memory | Sector number | Sector size |
| 1MB | Sector 0 | 16 Kbytes | Bank 1 512KB | Sector 0 | 16 Kbytes |
| Sector 1 | 16 Kbytes | Sector 1 | 16 Kbytes |  |  |
| Sector 2 | 16 Kbytes | Sector 2 | 16 Kbytes |  |  |
| Sector 3 | 16 Kbytes | Sector 3 | 16 Kbytes |  |  |
| Sector 4 | 64 Kbytes | Sector 4 | 64 Kbytes |  |  |
| Sector 5 | 128 Kbytes | Sector 5 | 128 Kbytes |  |  |
| Sector 6 | 128 Kbytes | Sector 6 | 128 Kbytes |  |  |
| Sector 7 | 128 Kbytes | Sector 7 | 128 Kbytes |  |  |
| Sector 8 | 128 Kbytes | Bank 2 512KB | Sector 12 | 16 Kbytes |  |
| Sector 9 | 128 Kbytes | Sector 13 | 16 Kbytes |  |  |
| Sector 10 | 128 Kbytes | Sector 14 | 16 Kbytes |  |  |
| Sector 11 | 128 Kbytes | Sector 15 | 16 Kbytes |  |  |
| - | Sector 16 | 64 Kbytes |  |  |  |
| - | Sector 17 | 128 Kbytes |  |  |  |
| - | Sector 18 | 128 Kbytes |  |  |  |
| - | Sector 19 | 128 Kbytes |  |  |  |


**Table 8. 1 Mbyte single bank Flash memory organization**

**(STM32F42xxx and STM32F43xxx)**

| Block | Bank | Name | Block base addresses | Size |
|---|---|---|---|---|
| Main memory | Single bank | Sector 0 | 0x0800 0000 - 0x0800 3FFF | 16 Kbytes |
| Sector 1 | 0x0800 4000 - 0x0800 7FFF | 16 Kbytes |  |  |
| Sector 2 | 0x0800 8000 - 0x0800 BFFF | 16 Kbytes |  |  |
| Sector 3 | 0x0800 C000 - 0x0800 FFFF | 16 Kbytes |  |  |
| Sector 4 | 0x0801 0000 - 0x0801 FFFF | 64 Kbytes |  |  |
| Sector 5 | 0x0802 0000 - 0x0803 FFFF | 128 Kbytes |  |  |
| Sector 6 | 0x0804 0000 - 0x0805 FFFF | 128 Kbytes |  |  |
| Sector 7 | 0x0806 0000 - 0x0807 FFFF | 128 Kbytes |  |  |
| Sector 8 | 0x0808 0000 - 0x0809 FFFF | 128 Kbytes |  |  |
| Sector 9 | 0x080A 0000 - 0x080B FFFF | 128 Kbytes |  |  |
| Sector 10 | 0x080C 0000 - 0x080D FFFF | 128 Kbytes |  |  |
| Sector 11 | 0x080E 0000 - 0x080F FFFF | 128 Kbytes |  |  |




--- Page 79 ---

**RM0090 Embedded Flash memory interface**

**Table 8. 1 Mbyte single bank Flash memory organization**

**(STM32F42xxx and STM32F43xxx) (continued)**

| Block | Bank | Name | Block base addresses | Size |
|---|---|---|---|---|
| System memory | 0x1FFF 0000 - 0x1FFFF 77FF | 30 Kbytes |  |  |
| OTP | 0x1FFF 7800 - 0x1FFF 7A0F | 528 bytes |  |  |
| Option bytes | 0x1FFF C000 - 0x1FFF C00F | 16 bytes |  |  |
| 0x1FFE C000 - 0x1FFE C00F | 16 bytes |  |  |  |


**Table 9. 1 Mbyte dual bank Flash memory organization (STM32F42xxx and STM32F43xxx)**

| Block |  | Name | Block base addresses | Size |
|---|---|---|---|---|
| Main memory | Bank 1 | Sector 0 | 0x0800 0000 - 0x0800 3FFF | 16 Kbytes |
| Sector 1 | 0x0800 4000 - 0x0800 7FFF | 16 Kbytes |  |  |
| Sector 2 | 0x0800 8000 - 0x0800 BFFF | 16 Kbytes |  |  |
| Sector 3 | 0x0800 C000 - 0x0800 FFFF | 16 Kbyte |  |  |
| Sector 4 | 0x0801 0000 - 0x0801 FFFF | 64 Kbytes |  |  |
| Sector 5 | 0x0802 0000 - 0x0803 FFFF | 128 Kbytes |  |  |
| Sector 6 | 0x0804 0000 - 0x0805 FFFF | 128 Kbytes |  |  |
| Sector 7 | 0x0806 0000 - 0x0807 FFFF | 128 Kbytes |  |  |
| Bank 2 | Sector 12 | 0x0808 0000 - 0x0808 3FFF | 16 Kbytes |  |
| Sector 13 | 0x0808 4000 - 0x0808 7FFF | 16 Kbytes |  |  |
| Sector 14 | 0x0808 0000 - 0x0808 BFFF | 16 Kbytes |  |  |
| Sector 15 | 0x0808 C000 - 0x0808 FFFF | 16 Kbytes |  |  |
| Sector 16 | 0x0809 0000 - 0x0809 FFFF | 64 Kbytes |  |  |
| Sector 17 | 0x080A 0000 - 0x080B FFFF | 128 Kbytes |  |  |
| Sector 18 | 0x080C 0000 - 0x080D FFFF | 128 Kbytes |  |  |
| Sector 19 | 0x080E 0000 - 0x080F FFFF | 128 Kbytes |  |  |
| System memory | 0x1FFF 0000 - 0x1FFF 77FF | 30 Kbytes |  |  |
| OTP | 0x1FFF 7800 - 0x1FFF 7A0F | 528 bytes |  |  |
| Option bytes | Bank 1 | 0x1FFF C000 - 0x1FFF C00F | 16 bytes |  |
| Bank 2 | 0x1FFE C000 - 0x1FFE C00F | 16 bytes |  |  |




112

--- Page 80 ---

**Embedded Flash memory interface RM0090**

