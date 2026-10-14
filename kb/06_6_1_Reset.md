## 6 Reset and clock control for

## STM32F42xxx and STM32F43xxx (RCC)

**6.1 Reset**

There are three types of reset, defined as system Reset, power Reset and backup domain Reset.

**6.1.1 System reset**

A system reset sets all registers to their reset values except the reset flags in the clock controller CSR register and the registers in the Backup domain (see Figure 15 ). A system reset is generated when one of the following events occurs:

1. A low level on the NRST pin (external reset)

2. Window watchdog end of count condition (WWDG reset)

3. Independent watchdog end of count condition (IWDG reset)

4. A software reset (SW reset) (see Software reset )

5. Low-power management reset (see Low-power management reset )

**Software reset**

The reset source can be identified by checking the reset flags in the RCC clock control & status register (RCC_CSR) .

The SYSRESETREQ bit in Cortex ® -M4 with FPU Application Interrupt and Reset Control Register must be set to force a software reset on the device. Refer to the Cortex ® -M4 with FPU technical reference manual for more details.

**Low-power management reset**

There are two ways of generating a low-power management reset:

1. Reset generated when entering the Standby mode:

This type of reset is enabled by resetting the nRST_STDBY bit in the user option bytes. In this case, whenever a Standby mode entry sequence is successfully executed, the device is reset instead of entering the Standby mode.

2. Reset when entering the Stop mode:

This type of reset is enabled by resetting the nRST_STOP bit in the user option bytes. In this case, whenever a Stop mode entry sequence is successfully executed, the device is reset instead of entering the Stop mode.

**6.1.2 Power reset**

A power reset is generated when one of the following events occurs:

1. Power-on/power-down reset (POR/PDR reset) or brownout (BOR) reset

2. When exiting the Standby mode

A power reset sets all registers to their reset values except the Backup domain (see Figure 15 )

150/1749 RM0090 Rev 18

--- Page 151 ---

**RM0090 Reset and clock control for   STM32F42xxx and STM32F43xxx (RCC)**

These sources act on the NRST pin and it is always kept low during the delay phase. The RESET service routine vector is fixed at address 0x0000_0004 in the memory map.

The system reset signal provided to the device is output on the NRST pin. The pulse generator guarantees a minimum reset pulse duration of 20 µs for each internal reset source. In case of an external reset, the reset pulse is generated while the NRST pin is asserted low.

**Figure 15. Simplified diagram of the reset circuit**

V DD /V DDA

R PU

External reset Filter System reset

NRST

(min 20 μs) generator Pulse WWDG reset IWDG reset Power reset Software reset

Low-power management reset

ai16095c

The Backup domain has two specific resets that affect only the Backup domain (see Figure 15 ).

**6.1.3 Backup domain reset**

The backup domain reset sets all RTC registers and the RCC_BDCR register to their reset values. The BKPSRAM is not affected by this reset. The only way of resetting the BKPSRAM is through the Flash interface by requesting a protection level change from 1 to 0.

A backup domain reset is generated when one of the following events occurs:

1. Software reset, triggered by setting the BDRST bit in the RCC Backup domain control

register (RCC_BDCR) .

2. V DD or V BAT power on, if both supplies have previously been powered off.

