### 3.19 Low-power modes

The STM32F405xx and STM32F407xx support three low-power modes to achieve the best compromise between low-power consumption, short startup time and available wakeup sources:

*   **Sleep mode**
    In Sleep mode, only the CPU is stopped. All peripherals continue to operate and can wake up the CPU when an interrupt/event occurs.
*   **Stop mode**
    The Stop mode achieves the lowest power consumption while retaining the contents of SRAM and registers. All clocks in the V12 domain are stopped, the PLL, the HSI RC and the HSE crystal oscillators are disabled. The voltage regulator can also be put either in normal or in low-power mode.
    The device can be woken up from the Stop mode by any of the EXTI line (the EXTI line source can be one of the 16 external lines, the PVD output, the RTC alarm / wakeup / tamper / time stamp events, the USB OTG FS/HS wakeup or the Ethernet wakeup).
*   **Standby mode**
    The Standby mode is used to achieve the lowest power consumption. The internal voltage regulator is switched off so that the entire V12 domain is powered off. The PLL, the HSI RC and the HSE crystal oscillators are also switched off. After entering

32/206
DS8626 Rev 12
ST

--- Page 33 ---

**STM32F405xx, STM32F407xx**
**Functional overview**

Standby mode, the SRAM and register contents are lost except for registers in the backup domain and the backup SRAM when selected.

The device exits the Standby mode when an external reset (NRST pin), an IWDG reset, a rising edge on the WKUP pin, or an RTC alarm / wakeup / tamper /time stamp event occurs.

The standby mode is not supported when the embedded voltage regulator is bypassed and the V12 domain is controlled by an external power.

### 3.20 VBAT operation

The VBAT pin allows to power the device VBAT domain from an external battery, an external supercapacitor, or from VDD when no external battery and an external supercapacitor are present.

VBAT operation is activated when VDD is not present.

The VBAT pin supplies the RTC, the backup registers and the backup SRAM.

*Note:* When the microcontroller is supplied from VBAT, external interrupts and RTC alarm/events do not exit it from VBAT operation.

When PDR_ON pin is not connected to VDD (internal reset OFF), the VBAT functionality is no more available and VBAT pin should be connected to VDD.

