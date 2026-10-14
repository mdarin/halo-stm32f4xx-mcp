**3.7 Option bytes**

**3.7.1 Description of user option bytes**

The option bytes are configured by the end user depending on the application requirements. Table 14 shows the organization of these bytes inside the user configuration sector.

**Table 14. Option byte organization**

| Address | [63:16] | [15:0] |
|---|---|---|
| 0x1FFF C000 | Reserved | ROP & user option bytes (RDP & USER) |
| 0x1FFF C008 | Reserved | SPRMOD and Write protection nWRP bits for sectors 0 to 11 |




--- Page 89 ---

**RM0090 Embedded Flash memory interface**

**Table 14. Option byte organization (continued)**

| Address | [63:16] | [15:0] |
|---|---|---|
| 0x1FFE C000 | Reserved |  |
| 0x1FFE C008 | Reserved | SPRMOD and Write protection nWRP bits for sectors 12 to 23 |


**Table 15. Description of the option bytes (STM32F405xx/07xx and**

**STM32F415xx/17xx)**

| Option bytes (word, address 0x1FFF C000) |  |
|---|---|
| RDP: Read protection option byte. The read protection is used to protect the software code stored in Flash memory. |  |
| Bits 15:8 | 0xAA: Level 0, no protection 0xCC: Level 2, chip protection (debug and boot from RAM features disabled) Others: Level 1, read protection of memories (debug features limited) |
| USER: User option byte This byte is used to configure the following features: – Select the watchdog event: Hardware or software – Reset event when entering the Stop mode – Reset event when entering the Standby mode |  |
| Bit 7 | nRST STDBY _ 0: Reset generated when entering the Standby mode 1: No reset generated |
| Bit 6 | nRST STOP _ 0: Reset generated when entering the Stop mode 1: No reset generated |
| Bit 5 | WDG SW _ 0: Hardware independent watchdog 1: Software independent watchdog |
| Bit 4 | 0x1: Not used |
| Bits 3:2 | BOR LEV: BOR reset Level _ These bits contain the supply level threshold that activates/releases the reset. They can be written to program a new BOR level value into Flash memory. 00: BOR Level 3 (VBOR3), brownout threshold level 3 01: BOR Level 2 (VBOR2), brownout threshold level 2 10: BOR Level 1 (VBOR1), brownout threshold level 1 11: BOR off, POR/PDR reset threshold level is applied Note: For full details on BOR characteristics, refer to the “Electrical characteristics” section of the product datasheet. |
| Bits 1:0 | 0x1: Not used |




112

--- Page 90 ---

**Embedded Flash memory interface RM0090**

**Table 15. Description of the option bytes (STM32F405xx/07xx and**

**STM32F415xx/17xx) (continued)**

| Option bytes (word, address 0x1FFF C008) |  |
|---|---|
| Bits 15:12 | 0xF: Not used |
| nWRP: Flash memory write protection option bytes Sectors 0 to 11 can be write protected. |  |
| Bits 11:0 | nWRPi 0: Write protection active on selected sector 1: Write protection not active on selected sector |


**Table 16. Description of the option bytes**

**(STM32F42xxx and STM32F43xxx)**

| Option bytes (word, address 0x1FFF C000) |  |
|---|---|
| RDP: Read protection option byte. The read protection is used to protect the software code stored in Flash memory. |  |
| Bit 15:8 | 0xAA: Level 0, no protection 0xCC: Level 2, chip protection (debug and boot from RAM features disabled) Others: Level 1, read protection of memories (debug features limited) |
| USER: User option byte This byte is used to configure the following features: Select the watchdog event: Hardware or software Reset event when entering the Stop mode Reset event when entering the Standby mode |  |
| Bit 7 | nRST STDBY _ 0: Reset generated when entering the Standby mode 1: No reset generated |
| Bit 6 | nRST STOP _ 0: Reset generated when entering the Stop mode 1: No reset generated |
| Bit 5 | WDG SW _ 0: Hardware independent watchdog 1: Software independent watchdog |
| Bit 4 | BFB2: Dual bank boot 0: Boot from Flash memory bank 1 or system memory depending on boot pin state (Default). 1: Boot always from system memory (Dual bank boot mode). |




--- Page 91 ---

**RM0090 Embedded Flash memory interface**

**Table 16. Description of the option bytes**

**(STM32F42xxx and STM32F43xxx) (continued)**

| Bits 3:2 | BOR LEV: BOR reset Level _ These bits contain the supply level threshold that activates/releases the reset. They can be written to program a new BOR level value into Flash memory. 00: BOR Level 3 (VBOR3), brownout threshold level 3 01: BOR Level 2 (VBOR2), brownout threshold level 2 10: BOR Level 1 (VBOR1), brownout threshold level 1 11: BOR off, POR/PDR reset threshold level is applied Note: For full details on BOR characteristics, refer to the “Electrical characteristics” section of the product datasheet. |
|---|---|
| Bits 1:0 | 0x1: Not used |
| Option bytes (word, address 0x1FFF C008) |  |
| Bit 15 | SPRMOD: Selection of protection mode of nWPRi bits 0: nWPRi bits used for sector i write protection (Default) 1: nWPRi bits used for sector i PCROP protection (Sector) |
| Bit 14 | DB1M: Dual bank 1 Mbyte Flash memory devices 0: 1 Mbyte single Flash memory (contiguous addresses in bank 1) 1: 1 Mbyte dual bank Flash memory. The Flash memory is organized as two banks of 512 Kbytes each (see Table 7: 1 Mbyte Flash memory single bank vs dual bank organization (STM32F42xxx and STM32F43xxx) and Table 9: 1 Mbyte dual bank Flash memory organization (STM32F42xxx and STM32F43xxx)). To perform an erase operation, the right sector must be programmed (see Table 7 for information on the sector numbering scheme). |
| Bits 13:12 | 0x2: not used |
| nWRP: Flash memory write protection option bytes for bank 1. Sectors 0 to 11 can be write protected. |  |
| Bits 11:0 | nWRPi: If SPRMOD is reset (default value): 0: Write protection active on sector i. 1: Write protection not active on sector i. If SPRMOD is set (active): 0: PCROP protection not active on sector i. 1: PCROP protection active on sector i. |
| Option bytes (word, address 0x1FFE C000) |  |
| Bit 15:0 | 0xFFFF: not used |




112

--- Page 92 ---

**Embedded Flash memory interface RM0090**

**Table 16. Description of the option bytes**

**(STM32F42xxx and STM32F43xxx) (continued)**

| Option bytes (word, address 0x1FFE C008) |  |
|---|---|
| Bit 15:12 | 0xF: not used |
| nWRP: Flash memory write protection option bytes for bank 2. Sectors 12 to 23 can be write protected. |  |
| Bits 11: 0 | nWRPi: If SPRMOD is reset (default value): 0: Write protection active on sector i. 1: Write protection not active on sector i. If SPRMOD is set (active): 0: PCROP protection not active on sector i. 1: PCROP protection active on sector i. |


**3.7.2 Programming user option bytes**

To run any operation on this sector, the option lock bit (OPTLOCK) in the Flash option control register (FLASH_OPTCR) must be cleared. To be allowed to clear this bit, you have to perform the following sequence:

1. Write OPTKEY1 = 0x0819 2A3B in the Flash option key register (FLASH_OPTKEYR)

2. Write OPTKEY2 = 0x4C5D 6E7F in the Flash option key register (FLASH_OPTKEYR) The user option bytes can be protected against unwanted erase/program operations by setting the OPTLOCK bit by software.

**Modifying user option bytes on STM32F405xx/07xx and STM32F415xx/17xx**

To modify the user option value, follow the sequence below:

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register

2. Write the desired option value in the FLASH_OPTCR register.

3. Set the option start bit (OPTSTRT) in the FLASH_OPTCR register

4. Wait for the BSY bit to be cleared.

Note: The value of an option is automatically modified by first erasing the user configuration sector and then programming all the option bytes with the values contained in the FLASH_OPTCR register.

**Modifying user option bytes on STM32F42xxx and STM32F43xxx**

The user option bytes for bank 1 and bank 2 cannot be modified independently. They must be updated concurrently.

To modify the user option byte value, follow the sequence below:

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register

2. Write the bank 2 option byte value in the FLASH_OPTCR1 register

3. Write the bank 1 option byte value in the FLASH_OPTCR register.

4. Set the option start bit (OPTSTRT) in the FLASH_OPTCR register

5. Wait for the BSY bit to be cleared



--- Page 93 ---

**RM0090 Embedded Flash memory interface**

Note: The value of an option byte is automatically modified by first erasing the user configuration sector (bank 1 and 2) and then programming all the option bytes with the values contained in the FLASH_OPTCR and FLASH_OPTCR1 registers.

**3.7.3 Read protection (RDP)**

The user area in the Flash memory can be protected against read operations by an entrusted code. Three read protection levels are defined:

• Level 0: no read protection

When the read protection level is set to Level 0 by writing 0xAA into the read protection option byte (RDP), all read/write operations (if no write protection is set) from/to the Flash memory or the backup SRAM are possible in all boot configurations (Flash user boot, debug or boot from RAM).

• Level 1: read protection enabled

It is the default read protection level after option byte erase. The read protection Level 1 is activated by writing any value (except for 0xAA and 0xCC used to set Level 0 and Level 2, respectively) into the RDP option byte. When the read protection Level 1 is set:

– No access (read, erase, program) to Flash memory or backup SRAM can be

performed while the debug feature is connected or while booting from RAM or system memory bootloader. A bus error is generated in case of read request.

– When booting from Flash memory, accesses (read, erase, program) to Flash

memory and backup SRAM from user code are allowed.

When Level 1 is active, programming the protection option byte (RDP) to Level 0 causes the Flash memory and the backup SRAM to be mass-erased. As a result the user code area is cleared before the read protection is removed. The mass erase only erases the user code area. The other option bytes including write protections remain unchanged from before the mass-erase operation. The OTP area is not affected by mass erase and remains unchanged. Mass erase is performed only when Level 1 is active and Level 0 requested. When the protection level is increased (0->1, 1->2, 0->2) there is no mass erase.

• Level 2: debug/chip read protection disabled

The read protection Level 2 is activated by writing 0xCC to the RDP option byte. When the read protection Level 2 is set:

– All protections provided by Level 1 are active.

– Booting from RAM or system memory bootloader is no more allowed.

– JTAG, SWV (single-wire viewer), ETM, and boundary scan are disabled.

– User option bytes can no longer be changed.

– When booting from Flash memory, accesses (read, erase and program) to Flash

memory and backup SRAM from user code are allowed.

Memory read protection Level 2 is an irreversible operation. When Level 2 is activated, the level of protection cannot be decreased to Level 0 or Level 1.

Note: The JTAG port is permanently disabled when Level 2 is active (acting as a JTAG fuse). As a consequence, boundary scan cannot be performed. STMicroelectronics is not able to perform analysis on defective parts on which the Level 2 protection has been set.



112

--- Page 94 ---

**Embedded Flash memory interface**

**RM0090**

--

**Table 17. Access versus read protection level**

| Memory area | Protection Level | Debug features, Boot from RAM or from System memory bootloader | Booting from Flash memory |  |  |
|---|---|---|---|---|---|
| Read | Write | Erase | Read | Write | Erase |
| Main Flash Memory and Backup SRAM | Level 1 | NO | NO(1) | YES |  |
| Level 2 | NO | YES |  |  |  |
| Option Bytes | Level 1 | YES |  |  |  |
| Level 2 | NO |  |  |  |  |
| OTP | Level 1 | NO | NA | YES | NA |
| Level 2 | NO | NA | YES | NA |  |


1. The main Flash memory and backup SRAM are only erased when the RDP changes from level 1 to 0. The OTP area remains unchanged.

Figure 6 shows how to go from one RDP level to another.

**Figure 6. RDP levels**

RDP /= AAh & /= CCh

Others options modified

Level 1

RDP /= AAh

RDP /= CCh

Write options

default Write options

including

including Write options including RDP = AAh RDP /= CCh & /= AAh

RDP = CCh

Level 0

Level 2

RDP = CCh

Write options RDP = AAh

including

RDP = CCh

RDP = AAh

Others option(s) modified

Options write (RDP level increase) includes

- Options erase

- New options program

Options write (RDP level decrease) includes

Options write (RDP level identical) includes

- Mass erase

- Options erase

- Options erase

- New options program

- New options program

MS47541V1

**3.7.4 Write protections**

Up to 24 user sectors in Flash memory can be protected against unwanted write operations due to loss of program counter contexts. When the non-write protection nWRPi bit (0 ≤ i ≤ 11) in the FLASH_OPTCR or FLASH_OPTCR1 registers is low, the corresponding sector



--- Page 95 ---

**RM0090 Embedded Flash memory interface**

cannot be erased or programmed. Consequently, a mass erase cannot be performed if one of the sectors is write-protected.

If an erase/program operation to a write-protected part of the Flash memory is attempted (sector protected by write protection bit, OTP part locked or part of the Flash memory that can never be written like the ICP), the write protection error flag (WRPERR) is set in the FLASH_SR register.

On STM32F42xxx and STM32F43xxx devices, when the PCROP mode is set, the active level of nWRPi is high, and the corresponding sector i is write protected when nWRPi is high. A PCROP sector is automatically write protected.

Note: When the memory read protection level is selected (RDP level = 1), it is not possible to program or erase Flash memory sector i if the CPU debug features are connected (JTAG or single wire) or boot code is being executed from RAM, even if nWRPi = 1.

**Write protection error flag**

If an erase/program operation to a write protected area of the Flash memory is performed, the Write Protection Error flag (WRPERR) is set in the FLASH_SR register.

If an erase operation is requested, the WRPERR bit is set when:

• Mass, bank, sector erase are configured (MER or MER/MER1 and SER = 1)

• A sector erase is requested and the Sector Number SNB field is not valid

• A mass erase is requested while at least one of the user sector is write protected by

option bit (MER or MER/MER1 = 1 and nWRPi = 0 with 0 ≤ i ≤ 11 bits in the FLASH_OPTCRx register

• A sector erase is requested on a write protected sector. (SER = 1, SNB = i and

nWRPi = 0 with 0 ≤ i ≤ 11 bits in the FLASH_OPTCRx register)

• The Flash memory is readout protected and an intrusion is detected.

If a program operation is requested, the WRPERR bit is set when:

• A write operation is performed on system memory or on the reserved part of the user

specific sector.

• A write operation is performed to the user configuration sector

• A write operation is performed on a sector write protected by option bit.

• A write operation is requested on an OTP area which is already locked

• The Flash memory is read protected and an intrusion is detected.

**3.7.5 Proprietary code readout protection (PCROP)**

The proprietary readout protection (PCROP) is available only on STM32F42xxx and STM32F43xxx devices.

Flash memory user sectors (0 to 23) can be protected against D-bus read accesses by using the proprietary readout protection (PCROP).

The PCROP protection is selected as follows, through the SPRMOD option bit in the FLASH_OPTCR register:

• SPRMOD = 0: nWRPi control the write protection of respective user sectors

• SPRMOD = 1: nWRPi control the read and write protection (PCROP) of respective

user sectors.



112

--- Page 96 ---

**Embedded Flash memory interface RM0090**

When a sector is readout protected (PCROP mode activated), it can only be accessed for code fetch through ICODE Bus on Flash interface:

• Any read access performed through the D-bus triggers a RDERR flag error.

• Any program/erase operation on a PCROPed sector triggers a WRPERR flag error.

**Figure 7. PCROP levels**

Write options

SPMOD = active

and valid nWRPi*

Level 1

RDP /= 0xAA

RDP /= 0xCC

Write options default Write options No restriction on SPMOD = active SPMOD = active Write options and valid nWRPi* and valid nWRPi*

Level 2 Level 0 RDP = 0xCC Write options RDP = 0xAA

and valid nWRPi* SPMOD = active Write options

SPMOD = active User option sector erase and valid nWRPi* Program new options

Global mass erase

User option section erase

Program new options

* Valid nWRPi means that none of the nWRP bits set can be reset (transition from 1 to 0) MS30467V1

The deactivation of the SPRMOD and/or the unprotection of PCROPed user sectors can only occur when, at the same time, the RDP level changes from 1 to 0. If this condition is not respected, the user option byte modification is cancelled and the write error WRPERR flag is set. The modification of the users option bytes (BOR_LEV, RST_STDBY, ..) is allowed since none of the active nWRPi bits is reset and SPRMOD is kept active.

Note: The active value of nWRPi bits is inverted when PCROP mode is active (SPRMOD =1). If SPRMOD = 1 and nWRPi =1, then user sector i of bank 1, respectively bank 2 is read/write protected (PCROP).



--- Page 97 ---

**RM0090 Embedded Flash memory interface**

**3.8 One-time programmable bytes**

Table 18 shows the organization of the one-time programmable (OTP) part of the OTP area.

**Table 18. OTP area organization**

| Block | [128:96] | [95:64] | [63:32] | [31:0] | Address byte 0 |
|---|---|---|---|---|---|
| 0 | OTP0 | 0x1FFF 7800 |  |  |  |
| OTP0 | 0x1FFF 7810 |  |  |  |  |
| 1 | OTP1 | 0x1FFF 7820 |  |  |  |
| OTP1 | 0x1FFF 7830 |  |  |  |  |
| . . . |  |  |  |  |  |
| 15 | OTP15 | 0x1FFF 79E0 |  |  |  |
| OTP15 | 0x1FFF 79F0 |  |  |  |  |
| Lock block | LOCKB15 ... LOCKB12 | LOCKB11 ... LOCKB8 | LOCKB7 ... LOCKB4 | LOCKB3 ... LOCKB0 | 0x1FFF 7A00 |


The OTP area is divided into 16 OTP data blocks of 32 bytes and one lock OTP block of 16 bytes. The OTP data and lock blocks cannot be erased. The lock block contains 16 bytes LOCKBi (0 ≤ i ≤ 15) to lock the corresponding OTP data block (blocks 0 to 15). Each OTP data block can be programmed until the value 0x00 is programmed in the corresponding OTP lock byte. The lock bytes must only contain 0x00 and 0xFF values, otherwise the OTP bytes might not be taken into account correctly.



112

--- Page 98 ---

**Embedded Flash memory interface**

**RM0090**

