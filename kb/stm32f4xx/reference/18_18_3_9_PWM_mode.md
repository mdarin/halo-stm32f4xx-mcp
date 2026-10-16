**18.3.9 PWM mode**

Pulse width modulation mode allows generating a signal with a frequency determined by the value of the TIMx_ARR register and a duty cycle determined by the value of the TIMx_CCRx register.

The PWM mode can be selected independently on each channel (one PWM per OCx output) by writing 110 (PWM mode 1) or '111 (PWM mode 2) in the OCxM bits in the TIMx_CCMRx register. The user must enable the corresponding preload register by setting the OCxPE bit in the TIMx_CCMRx register, and eventually the auto-reload preload register by setting the ARPE bit in the TIMx_CR1 register.

As the preload registers are transferred to the shadow registers only when an update event occurs, before starting the counter, the user has to initialize all the registers by setting the UG bit in the TIMx_EGR register.

OCx polarity is software programmable using the CCxP bit in the TIMx_CCER register. It can be programmed as active high or active low. OCx output is enabled by the CCxE bit in the TIMx_CCER register. Refer to the TIMx_CCERx register description for more details.

In PWM mode (1 or 2), TIMx_CNT and TIMx_CCRx are always compared to determine whether TIMx_CCRx<TIMx_CNT or TIMx_CNT<TIMx_CCRx (depending on the direction of the counter). However, to comply with the ETRF (OCREF can be cleared by an external event through the ETR signal until the next PWM period), the OCREF signal is asserted only:

*   When the result of the comparison changes, or
*   When the output compare mode (OCxM bits in TIMx_CCMRx register) switches from the "frozen" configuration (no comparison, OCxM='000) to one of the PWM modes (OCxM='110 or '111).

This forces the PWM by software while the timer is running.

610/1749
RM0090 Rev 18
ST

--- Page 26 ---

RM0090
General-purpose timers (TIM2 to TIM5)

The timer is able to generate PWM in edge-aligned mode or center-aligned mode depending on the CMS bits in the TIMx_CR1 register.

**PWM edge-aligned mode**

**Upcounting configuration**

Upcounting is active when the DIR bit in the TIMx_CR1 register is low. Refer to *Upcounting mode*.

In the following example, we consider PWM mode 1. The reference PWM signal OCxREF is high as long as TIMx_CNT <TIMx_CCRx else it becomes low. If the compare value in TIMx_CCRx is greater than the auto-reload value (in TIMx_ARR) then OCxREF is held at '1'. If the compare value is 0 then OCxREF is held at '0'. Figure 164 shows some edge-aligned PWM waveforms in an example where TIMx_ARR=8.

**Figure 164. Edge-aligned PWM waveforms (ARR=8)**

Counter register 0 1 2 3 4 5 6 7 8 0 1
CCRx=4 OCXREF CCxIF
CCRx=8 OCXREF CCxIF
CCRx>8 OCXREF '1' CCxIF
CCRx=0 OCXREF '0' CCxIF

MS31093V1

**Downcounting configuration**

Downcounting is active when DIR bit in TIMx_CR1 register is high. Refer to *Downcounting mode*.

In PWM mode 1, the reference signal ocxref is low as long as TIMx_CNT>TIMx_CCRx else it becomes high. If the compare value in TIMx_CCRx is greater than the auto-reload value in TIMx_ARR, then ocxref is held at '1'. 0% PWM is not possible in this mode.

**PWM center-aligned mode**

Center-aligned mode is active when the CMS bits in TIMx_CR1 register are different from '00 (all the remaining configurations having the same effect on the ocxref/OCx signals). The compare flag is set when the counter counts up, when it counts down or both when it counts

ST
RM0090 Rev 18
611/1749

--- Page 27 ---

**General-purpose timers (TIM2 to TIM5)** RM0090

up and down depending on the CMS bits configuration. The direction bit (DIR) in the TIMx_CR1 register is updated by hardware and must not be changed by software. Refer to *Center-aligned mode (up/down counting)*.

Figure 165 shows some center-aligned PWM waveforms in an example where:
* TIMx_ARR=8,
* PWM mode is the PWM mode 1,
* The flag is set when the counter counts down corresponding to the center-aligned mode 1 selected for CMS=01 in TIMx_CR1 register.

**Figure 165. Center-aligned PWM waveforms (ARR=8)**

| Counter register | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 | 1 |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| OCxREFCCRx = 4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CCxIF |  |  |  |  | CMS=01CMS=10CMS=11 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| OCxREFCCRx = 7 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CCxIF |  |  |  |  | CMS=10 or 11 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| OCxREFCCRx = 8 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CCxIF |  |  |  |  | CMS=01CMS=10CMS=11 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| OCxREFCCRx > 8 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CCxIF |  |  |  |  | CMS=01CMS=10CMS=11 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| OCxREFCCRx = 0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| CCxIF |  |  |  |  | CMS=01CMS=10CMS=11 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


ai14681b

Hints on using center-aligned mode:
* When starting in center-aligned mode, the current up-down configuration is used. It means that the counter counts up or down depending on the value written in the DIR bit

612/1749 RM0090 Rev 18 STI

--- Page 28 ---

RM0090
General-purpose timers (TIM2 to TIM5)

in the TIMx_CR1 register. Moreover, the DIR and CMS bits must not be changed at the same time by the software.

*   Writing to the counter while running in center-aligned mode is not recommended as it can lead to unexpected results. In particular:
    *   The direction is not updated if the user writes a value in the counter that is greater than the auto-reload value (TIMx_CNT>TIMx_ARR). For example, if the counter was counting up, it continues to count up.
    *   The direction is updated if the user writes 0 or write the TIMx_ARR value in the counter but no Update Event UEV is generated.
*   The safest way to use center-aligned mode is to generate an update by software (setting the UG bit in the TIMx_EGR register) just before starting the counter and not to write the counter while it is running.

