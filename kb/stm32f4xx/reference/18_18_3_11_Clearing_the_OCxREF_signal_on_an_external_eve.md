**18.3.11 Clearing the OCxREF signal on an external event**

The OCxREF signal for a given channel can be driven Low by applying a High level to the ETRF input (OCxCE enable bit of the corresponding TIMx_CCMRx register set to '1'). The OCxREF signal remains Low until the next update event, UEV, occurs.

This function can only be used in output compare and PWM modes, and does not work in forced mode.

For example, the ETR signal can be connected to the output of a comparator to be used for current handling. In this case, ETR must be configured as follows:

614/1749
RM0090 Rev 18
ST

--- Page 30 ---

RM0090
General-purpose timers (TIM2 to TIM5)

1. The external trigger prescaler should be kept off: bits ETPS[1:0] in the TIMx_SMCR register are cleared to 00.
2. The external clock mode 2 must be disabled: bit ECE in the TIM1_SMCR register is cleared to 0.
3. The external trigger polarity (ETP) and the external trigger filter (ETF) can be configured according to the application’s needs.

Figure 167 shows the behavior of the OCxREF signal when the ETRF input becomes high, for both values of the OCxCE enable bit. In this example, the timer TIMx is programmed in PWM mode.

Figure 167. Clearing TIMx OCxREF

(CCRx)
Counter (CNT)
ETRF
OCxREF (OCxCE = '0')
OCxREF (OCxCE = '1')
ETRF becomes high
ETRF still high
MS37368V1

1. In case of a PWM with a 100% duty cycle (if CCRx>ARR), OCxREF is enabled again at the next counter overflow.

18.3.12 Encoder interface mode

To select Encoder Interface mode write SMS='001 in the TIMx_SMCR register if the counter is counting on TI2 edges only, SMS=010 if it is counting on TI1 edges only and SMS=011 if it is counting on both TI1 and TI2 edges.

Select the TI1 and TI2 polarity by programming the CC1P and CC2P bits in the TIMx_CCER register. When needed, program the input filter as well.

The two inputs TI1 and TI2 are used to interface to an incremental encoder. Refer to Table 97. The counter is clocked by each valid transition on TI1FP1 or TI2FP2 (TI1 and TI2 after input filter and polarity selection, TI1FP1=TI1 if not filtered and not inverted, TI2FP2=TI2 if not filtered and not inverted) assuming that it is enabled (CEN bit in TIMx_CR1 register written to '1'). The sequence of transitions of the two inputs is evaluated and generates count pulses as well as the direction signal. Depending on the sequence the counter counts up or down, the DIR bit in the TIMx_CR1 register is modified by hardware accordingly. The DIR bit is calculated at each transition on any input (TI1 or TI2), whatever the counter is counting on TI1 only, TI2 only or both TI1 and TI2.

Encoder interface mode acts simply as an external clock with direction selection. This means that the counter just counts continuously between 0 and the auto-reload value in the TIMx_ARR register (0 to ARR or ARR down to 0 depending on the direction). So the user must configure TIMx_ARR before starting. In the same way, the capture, compare, prescaler, trigger output features continue to work as normal.

ST
RM0090 Rev 18
615/1749

--- Page 31 ---

General-purpose timers (TIM2 to TIM5)
RM0090

In this mode, the counter is modified automatically following the speed and the direction of the incremental encoder and its content, therefore, always represents the encoder's position. The count direction correspond to the rotation direction of the connected sensor. The table summarizes the possible combinations, assuming T11 and T12 don't switch at the same time.

Table 97. Counting direction versus encoder signals

| Active edge | Level on opposite signal (TI1FP1 for T12, TI2FP2 for T11) | TI1FP1 signal | TI2FP2 signal |  |
|---|---|---|---|---|
| Rising | Falling | Rising | Falling |  |
| Counting on T11 only | High | Down | Up | No Count |
| Low | Up | Down | No Count |  |
| Counting on T12 only | High | No Count | Up | Down |
| Low | No Count | Down | Up |  |
| Counting on T11 and T12 | High | Down | Up | Down |
| Low | Up | Down | Up |  |


An external incremental encoder can be connected directly to the MCU without external interface logic. However, comparators are normally be used to convert the encoder's differential outputs to digital signals. This greatly increases noise immunity. The third encoder output which indicate the mechanical zero position, may be connected to an external interrupt input and trigger a counter reset.

*Figure 168* gives an example of counter operation, showing count signal generation and direction control. It also shows how input jitter is compensated where both edges are selected. This might occur if the sensor is positioned near to one of the switching points. For this example we assume that the configuration is the following:

* CC1S= '01' (TIMx_CCMR1 register, TI1FP1 mapped on T11)
* CC2S= '01' (TIMx_CCMR2 register, TI2FP2 mapped on T12)
* CC1P= '0', CC1NP= '0', IC1F ='0000' (TIMx_CCER register, TI1FP1 noninverted, TI1FP1=T11)
* CC2P= '0', CC2NP= '0', IC2F ='0000' (TIMx_CCER register, TI2FP2 noninverted, TI2FP2=T12)
* SMS= '011' (TIMx_SMCR register, both inputs are active on both rising and falling edges)
* CEN = 1 (TIMx_CR1 register, Counter is enabled)

616/1749
RM0090 Rev 18

--- Page 32 ---

RM0090
General-purpose timers (TIM2 to TIM5)

**Figure 168. Example of counter operation in encoder interface mode**

forward jitter backward jitter forward
T11
T12
Counter
up down up
MS33107V1

*Figure 169* gives an example of counter behavior when T11FP1 polarity is inverted (same configuration as above except CC1P=1).

**Figure 169. Example of encoder interface mode with T11FP1 polarity inverted**

forward jitter backward jitter forward
T11
T12
Counter
down up down
MS33108V1

The timer, when configured in Encoder Interface mode provides information on the sensor's current position. The user can obtain dynamic information (speed, acceleration, deceleration) by measuring the period between two encoder events using a second timer configured in capture mode. The output of the encoder which indicates the mechanical zero can be used for this purpose. Depending on the time between two events, the counter can also be read at regular times. The user can do this by latching the counter value into a third input capture register if available (then the capture signal must be periodic and can be generated by another timer). when available, it is also possible to read its value through a DMA request generated by a Real-Time clock.

ST
RM0090 Rev 18
617/1749

--- Page 33 ---

General-purpose timers (TIM2 to TIM5)
RM0090

**18.3.13 Timer input XOR function**

The TI1S bit in the TIM_CR2 register, allows the input filter of channel 1 to be connected to the output of a XOR gate, combining the three input pins TIMx_CH1 to TIMx_CH3.

The XOR output can be used with all the timer input functions such as trigger or input capture.

