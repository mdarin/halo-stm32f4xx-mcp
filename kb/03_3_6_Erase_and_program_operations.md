**3.6 Erase and program operations**

For any Flash memory program operation (erase or program), the CPU clock frequency (HCLK) must be at least 1 MHz. The contents of the Flash memory are not guaranteed if a device reset occurs during a Flash memory operation.

Any attempt to read the Flash memory on STM32F4xx while it is being written or erased, causes the bus to stall. Read operations are processed correctly once the program operation has completed. This means that code or data fetches cannot be performed while a write/erase operation is ongoing.

On STM32F42xxx and STM32F43xxx devices, two banks are available allowing read operation from one bank while a write/erase operation is performed to the other bank.

**3.6.1 Unlocking the Flash control register**

After reset, write is not allowed in the Flash control register (FLASH_CR) to protect the Flash memory against possible unwanted operations due, for example, to electric disturbances. The following sequence is used to unlock this register:

1. Write KEY1 = 0x45670123 in the Flash key register (FLASH_KEYR)

2. Write KEY2 = 0xCDEF89AB in the Flash key register (FLASH_KEYR)

Any wrong sequence will return a bus error and lock up the FLASH_CR register until the next reset.

The FLASH_CR register can be locked again by software by setting the LOCK bit in the FLASH_CR register.



--- Page 85 ---

**RM0090 Embedded Flash memory interface**

Note: The FLASH_CR register is not accessible in write mode when the BSY bit in the FLASH_ SR register is set. Any attempt to write to it with the BSY bit set will cause the AHB bus to stall until the BSY bit is cleared.

**3.6.2 Program/erase parallelism**

The Parallelism size is configured through the PSIZE field in the FLASH_CR register. It represents the number of bytes to be programmed each time a write operation occurs to the Flash memory. PSIZE is limited by the supply voltage and by whether the external V PP supply is used or not. It must therefore be correctly configured in the FLASH_CR register before any programming/erasing operation.

A Flash memory erase operation can only be performed by sector, bank or for the whole Flash memory (mass erase). The erase time depends on PSIZE programmed value. For more details on the erase time, refer to the electrical characteristics section of the device datasheet.

Table 12 provides the correct PSIZE values.

**Table 12. Program/erase parallelism**

|  | Voltage range 2.7 - 3.6 V with External V PP | Voltage range 2.7 - 3.6 V | Voltage range 2.4 - 2.7 V | Voltage range 2.1 - 2.4 V | Voltage range 1.8 V - 2.1 V |
|---|---|---|---|---|---|
| Parallelism size | x64 | x32 | x16 | x8 |  |
| PSIZE(1:0) | 11 | 10 | 01 | 00 |  |


Note: Any program or erase operation started with inconsistent program parallelism/voltage range settings may lead to unpredicted results. Even if a subsequent read operation indicates that the logical value was effectively written to the memory, this value may not be retained. To use V PP , an external high-voltage supply (between 8 and 9 V) must be applied to the V PP pad. The external supply must be able to sustain this voltage range even if the DC consumption exceeds 10 mA. It is advised to limit the use of VPP to initial programming on the factory line. The V PP supply must not be applied for more than an hour, otherwise the Flash memory might be damaged.

**3.6.3 Erase**

The Flash memory erase operation can be performed at sector level or on the whole Flash memory (Mass Erase). Mass Erase does not affect the OTP sector or the configuration sector.

**Sector Erase**

To erase a sector, follow the procedure below:

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register

2. Set the SER bit and select the sector out of the 12 sectors (for STM32F405xx/07xx and

STM32F415xx/17xx) and out of 24 (for STM32F42xxx and STM32F43xxx) in the main memory block you wish to erase (SNB) in the FLASH_CR register

3. Set the STRT bit in the FLASH_CR register

4. Wait for the BSY bit to be cleared



112

--- Page 86 ---

**Embedded Flash memory interface RM0090**

**Bank erase in STM32F42xxx and STM32F43xxx devices**

To erase bank 1 or bank 2, follow the procedure below:

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register

2. Set MER or MER1 bit accordingly in the FLASH_CR register

3. Set the STRT bit in the FLASH_CR register

4. Wait for the BSY bit to be reset.

**Mass Erase**

To perform Mass Erase, the following sequence is recommended:

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register

2. Set the MER bit in the FLASH_CR register (on STM32F405xx/07xx and

STM32F415xx/17xx devices)

3. Set both the MER and MER1 bits in the FLASH_CR register (on STM32F42xxx and

STM32F43xxx devices).

4. Set the STRT bit in the FLASH_CR register

5. Wait for the BSY bit to be cleared

Note: If MERx and SER bits are both set in the FLASH_CR register, mass erase is performed. If both MERx and SER bits are reset and the STRT bit is set, an unpredictable behavior may occur without generating any error flag. This condition should be forbidden.

**3.6.4 Programming**

**Standard programming**

The Flash memory programming sequence is as follows:

1. Check that no main Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register.

2. Set the PG bit in the FLASH_CR register

3. Perform the data write operation(s) to the desired memory address (inside main

memory block or OTP area):

– Byte access in case of x8 parallelism

– Half-word access in case of x16 parallelism

– Word access in case of x32 parallelism

– Double word access in case of x64 parallelism

4. Wait for the BSY bit to be cleared.

Note: Successive write operations are possible without the need of an erase operation when changing bits from ‘1’ to ‘0’. Writing ‘1’ requires a Flash memory erase operation. If an erase and a program operation are requested simultaneously, the erase operation is performed first.



--- Page 87 ---

**RM0090 Embedded Flash memory interface**

**Programming errors**

It is not allowed to program data to the Flash memory that would cross the 128-bit row boundary. In such a case, the write operation is not performed and a program alignment error flag (PGAERR) is set in the FLASH_SR register.

The write access type (byte, half-word, word or double word) must correspond to the type of parallelism chosen (x8, x16, x32 or x64). If not, the write operation is not performed and a program parallelism error flag (PGPERR) is set in the FLASH_SR register.

If the standard programming sequence is not respected (for example, if there is an attempt to write to a Flash memory address when the PG bit is not set), the operation is aborted and a program sequence error flag (PGSERR) is set in the FLASH_SR register.

**Programming and caches**

If a Flash memory write access concerns some data in the data cache, the Flash write access modifies the data in the Flash memory and the data in the cache.

If an erase operation in Flash memory also concerns data in the data or instruction cache, you have to make sure that these data are rewritten before they are accessed during code execution. If this cannot be done safely, it is recommended to flush the caches by setting the DCRST and ICRST bits in the FLASH_CR register.

Note: The I/D cache should be flushed only when it is disabled (I/DCEN = 0).

**3.6.5 Read-while-write (RWW)**

In STM32F42xxx and STM32F43xxx devices, the Flash memory is divided into two banks allowing read-while-write operations. This feature allows to perform a read operation from one bank while an erase or program operation is performed to the other bank.

Note: Write-while-write operations are not allowed. As an exampled, It is not possible to perform an erase operation on one bank while programming the other one.

**Read from bank 1 while erasing bank 2**

While executing a program code from bank 1, it is possible to perform an erase operation on bank 2 (and vice versa). Follow the procedure below:

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register (BSY is active when erase/program operation is on going to bank 1 or bank 2)

2. Set MER or MER1 bit in the FLASH_CR register

3. Set the STRT bit in the FLASH_CR register

4. Wait for the BSY bit to be reset (or use the EOP interrupt).

**Read from bank 1 while programming bank 2**

While executing a program code (over the I-Code bus) from bank 1,it is possible to perform an program operation to the bank 2 (and vice versa). Follow the procedure below:



112

--- Page 88 ---

**Embedded Flash memory interface RM0090**

1. Check that no Flash memory operation is ongoing by checking the BSY bit in the

FLASH_SR register (BSY is active when erase/program operation is on going on bank 1 or bank 2)

2. Set the PG bit in the FLASH_CR register

3. Perform the data write operation(s) to the desired memory address inside main

memory block or OTP area

4. Wait for the BSY bit to be reset.

**3.6.6 Interrupts**

Setting the end of operation interrupt enable bit (EOPIE) in the FLASH_CR register enables interrupt generation when an erase or program operation ends, that is when the busy bit (BSY) in the FLASH_SR register is cleared (operation completed, correctly or not). In this case, the end of operation (EOP) bit in the FLASH_SR register is set.

If an error occurs during a program, an erase, or a read operation request, one of the following error flags is set in the FLASH_SR register:

• PGAERR, PGPERR, PGSERR (Program error flags)

• WRPERR (Protection error flag)

• RDERR (Read protection error flag) for STM32F42xxx and STM32F43xxx devices

only.

In this case, if the error interrupt enable bit (ERRIE) is set in the FLASH_CR register, an interrupt is generated and the operation error bit (OPERR) is set in the FLASH_SR register. Note: If several successive errors are detected (for example, in case of DMA transfer to the Flash memory), the error flags cannot be cleared until the end of the successive write requests.

**Table 13. Flash interrupt request**

| Interrupt event | Event flag | Enable control bit |
|---|---|---|
| End of operation | EOP | EOPIE |
| Write protection error | WRPERR | ERRIE |
| Programming error | PGAERR, PGPERR, PGSERR | ERRIE |
| Read protection error | RDERR | ERRIE |


