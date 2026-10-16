**2.10 FSMC peripheral limitations**

**2.10.1 Dummy read cycles inserted when reading synchronous memories**

**Description**

When performing a burst read access to a synchronous memory, two dummy read accesses are performed at the end of the burst cycle whatever the type of AHB burst access.

However, the extra data values which are read are not used by the FSMC and there is no functional failure.

**Workaround**

None.

**2.10.2 FSMC synchronous mode and NWAIT signal disabled**

**Description**

When the FSMC synchronous mode operates with the NWAIT signal disabled, if the polarity (WAITPOL in the FSMC_BCRx register) of the NWAIT signal is identical to that of the NWAIT input signal level, the system hangs and no fault is generated.

**Workaround**

PD6 (NWAIT signal) must not be connected to AF12 and the NWAIT polarity must be configured to active high (set WAITPOL bit to 1 in FSMC_BCRx register).

**2.10.3 FSMC NOR Flash/PSRAM controller asynchronous access on bank 2 to 4 when bank 1 is in synchronous mode**

**(CBURSTRW bit is set)**

**Description**

If bank 1 of NOR/PSRAM controller is enabled in synchronous write mode (CBURSTRW bit set), while any other NOR/PSRAM banks (2 to 4) are enabled in asynchronous mode, two limitations occur:

• The byte lane NBL[1:0] are not active( kept at ‘1’) for the first write access to the

asynchronous memory.

• The system hangs without any fault generation when a write access is performed to an

asynchronous memory with the extended feature enabled.



37

--- Page 30 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

These two limitations occur only when the NOR/PSRAM bank 1 is configured in synchronous write mode (CBURSTRW bit set).

**Workaround**

If multiple banks are enabled with mixed asynchronous and synchronous write modes, use any NOR/PSRAM bank for synchronous write access, except for bank 1.

