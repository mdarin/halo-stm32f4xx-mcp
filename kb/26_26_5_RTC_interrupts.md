## 26.5 RTC interrupts

All RTC interrupts are connected to the EXTI controller.

To enable the RTC Alarm interrupt, the following sequence is required:
1. Configure and enable the EXTI Line 17 in interrupt mode and select the rising edge sensitivity.
2. Configure and enable the RTC_Alarm IRQ channel in the NVIC.
3. Configure the RTC to generate RTC alarms (Alarm A or Alarm B).

To enable the RTC Wakeup interrupt, the following sequence is required:
1. Configure and enable the EXTI Line 22 in interrupt mode and select the rising edge sensitivity.
2. Configure and enable the RTC_WKUP IRQ channel in the NVIC.
3. Configure the RTC to generate the RTC wakeup timer event.

To enable the RTC Tamper interrupt, the following sequence is required:
1. Configure and enable the EXTI Line 21 in interrupt mode and select the rising edge sensitivity.
2. Configure and Enable the TAMP_STAMP IRQ channel in the NVIC.
3. Configure the RTC to detect the RTC tamper event.

To enable the RTC TimeStamp interrupt, the following sequence is required:

814/1749
RM0090 Rev 18
ST

--- Page 35 ---

RM0090
Real-time clock (RTC)

1. Configure and enable the EXTI Line 21 in interrupt mode and select the rising edge sensitivity.
2. Configure and Enable the TAMP_STAMP IRQ channel in the NVIC.
3. Configure the RTC to detect the RTC timestamp event.

Table 120. Interrupt control bits

| Interrupt event | Event flag | Enable control bit | Exit the Sleep mode | Exit the Stop mode | Exit the Standby mode |
|---|---|---|---|---|---|
| Alarm A | ALRAF | ALRAIE | yes | yes(1) |  |
| Alarm B | ALRBF | ALRBIE | yes | yes(1) |  |
| Wakeup | WUTF | WUTIE | yes | yes(1) |  |
| TimeStamp | TSF | TSIE | yes | yes(1) |  |
| Tamper1 detection | TAMP1F | TAMPIE | yes | yes(1) |  |
| Tamper2 detection(2) | TAMP2F | TAMPIE | yes | yes(1) |  |


1. Wakeup from STOP and Standby modes is possible only when the RTC clock source is LSE or LSI.
2. If RTC_TAMPER2 pin is present. Refer to device datasheet pinout.

ST
RM0090 Rev 18
815/1749

--- Page 36 ---

Real-time clock (RTC)
RM0090

