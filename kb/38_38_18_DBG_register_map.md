### 38.18 DBG register map

The following table summarizes the Debug registers.

Table 314. DBG register map and reset values

| Addr. | Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0xE0042000 | DBGMCU_IDCODE | REV_ID | Reserved | DEV_ID |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value(1) | X |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xE0042004 | DBGMCU_CR | Reserved | DBG_TIM7_STOP | DBG_TIM6_STOP | DBG_TIM5_STOP | DBG_TIM8_STOP | DBG_I2C2_SMBUS_TIMEOUT | Reserved | TRACE_MODE_[1:0] | TRACE_ | Reserved | DBG_STANDBY | DBG_STOP | DBG_SLEEP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xE0042008 | DBGMCUAPB1_FZ | Reserved | DBG_CAN2_STOP | DBG_CAN1_STOP | Reserved | DBG_I2C3_SMBUS_TIMEOUT | DBG_I2C2_SMBUS_TIMEOUT | DBG_I2C1_SMBUS_TIMEOUT | Reserved | DBG_IWDG_STOP | DBG_WWDG_STOP | Reserved | DBG_RTC_STOP | DBG_TIM14_STOP | DBG_TIM13_STOP | DBG_TIM12_STOP | DBG_TIM7_STOP | DBG_TIM6_STOP | DBG_TIM5_STOP | DBG_TIM4_STOP | DBG_TIM3_STOP | DBG_TIM2_STOP | DBG_TIM1_STOP |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 0xE004200C | DBGMCUAPB2_FZ | Reserved | DBG_TIM11_STOP | DBG_TIM10_STOP | DBG_TIM9_STOP | Reserved | DBG_TIM8_STOP | DBG_TIM1_STOP |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Reset value | 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


1. The reset value is product dependent. For more information, refer to Section 38.6.1: MCU device ID code.

ST
RM0090 Rev 18
1713/1749

--- Page 154 ---

Device electronic signature
RM0090

