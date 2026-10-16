### 38.6 ID codes and locking mechanism

There are several ID codes inside the STM32F4xx MCUs. ST strongly recommends tools designers to lock their debuggers using the MCU DEVICE ID code located in the external PPB memory map at address 0xE0042000.

### 38.6.1 MCU device ID code

The STM32F4xx MCUs integrate an MCU ID code. This ID identifies the ST MCU part-number and the die revision. It is part of the DBG_MCU component and is mapped on the external PPB bus (see Section 38.16). This code is accessible using the JTAG debug port (four to five pins) or the SW debug port (two pins) or by the user software. It is even accessible while the MCU is under system reset.

Only the DEV_ID[11:0] should be used for identification by the debugger/programmer tools.

**DBGMCU_IDCODE**

Address: 0xE004 2000

Only 32-bits access supported. Read-only.

| 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 |  |  |  |  |
|---|---|---|---|---|
| REV_ID[15:0] |  |  |  |  |
| r |  |  |  |  |
| 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 |  |  |  |  |
| Reserved | DEV_ID[11:0] |  |  |  |
|  |  |  |  | r |


Bits 31:16 **REV_ID[15:0]** Revision identifier
This field indicates the revision of the device.
STM32F405xx/07xx and STM32F415xx/17xx devices:
0x1000 = Revision A
0x1001 = Revision Z
0x1003 = Revision 1
0x1007 = Revision 2
0x100F= Revision Y and 4
STM32F42xxx and STM32F43xxx devices:
0x1000 = Revision A
0x1003 = Revision Y
0x1007 = Revision 1
0x2001= Revision 3
0x2003= Revision 5 and B

Bits 15:12 Reserved, must be kept at reset value.

Bits 11:0 **DEV_ID[11:0]**: Device identifier (STM32F405xx/07xx and STM32F415xx/17xx)
The device ID is 0x413.

Bits 11:0 **DEV_ID[11:0]**: Device identifier (STM32F42xxx and STM32F43xxx)
The device ID is 0x419

1690/1749 RM0090 Rev 18

--- Page 131 ---

RM0090
Debug support (DBG)

### 38.6.2 Boundary scan TAP

**JTAG ID code**

The TAP of the STM32F4xx BSC (boundary scan) integrates a JTAG ID code equal to .

*   0x06413041 for STM32F405xx/07xx and STM32F415xx/17xx devices
*   0x06419041 for STM32F42xxx and STM32F43xxx devices

### 38.6.3 Cortex®-M4 with FPU TAP

The TAP of the Arm® Cortex®-M4 with FPU integrates a JTAG ID code. This ID code is the Arm® default one and has not been modified. This code is only accessible by the JTAG Debug Port, it is 0x4BA00477 (corresponds to Cortex®-M4 with FPU r0p1, see Section 38.2).

### 38.6.4 Cortex®-M4 with FPU JEDEC-106 ID code

The Arm® Cortex®-M4 with FPU integrates a JEDEC-106 ID code. It is located in the 4 KB ROM table mapped on the internal PPB bus at address 0xE00FF000_0xE00FFFFF.

This code is accessible by the JTAG Debug Port (4 to 5 pins) or by the SW Debug Port (two pins) or by the user software.

