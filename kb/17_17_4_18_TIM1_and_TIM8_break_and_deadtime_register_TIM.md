**17.4.18 TIM1 and TIM8 break and dead-time register (TIMx_BDTR)**

Address offset: 0x44

Reset value: 0x0000

15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| MOE | AOE | BKP | BKE | OSSR | OSSI | LOCK[1:0] | DTG[7:0] |
|---|---|---|---|---|---|---|---|
| rw |  |  |  |  |  |  |  |


Note: As the bits AOE, BKP, BKE, OSSI, OSSR and DTG[7:0] can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.

Bit 15 MOE : Main output enable

This bit is cleared asynchronously by hardware as soon as the break input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output.

0: OC and OCN outputs are disabled or forced to idle state.

1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register).

See OC/OCN enable description for more details ( Section 17.4.9: TIM1 and TIM8 capture/compare enable register (TIMx_CCER) ).

Bit 14 AOE : Automatic output enable

0: MOE can be set only by software

1: MOE can be set by software or automatically at the next update event (if the break input is not be active)

Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits

in TIMx_BDTR register).



588

--- Page 194 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

Bit 13 BKP : Break polarity

0: Break input BRK is active low

1: Break input BRK is active high

Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits

in TIMx_BDTR register).

Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective. Bit 12 BKE : Break enable

0: Break inputs (BRK and CSS clock failure event) disabled

1; Break inputs (BRK and CSS clock failure event) enabled

Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in

TIMx_BDTR register).

Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective. Bit 11 OSSR : Off-state selection for Run mode

This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer.

See OC/OCN enable description for more details ( Section 17.4.9: TIM1 and TIM8 capture/compare enable register (TIMx_CCER) ).

0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0). 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1. Then, OC/OCN enable output signal=1

Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK

bits in TIMx_BDTR register).

Bit 10 OSSI : Off-state selection for Idle mode

This bit is used when MOE=0 on channels configured as outputs.

See OC/OCN enable description for more details ( Section 17.4.9: TIM1 and TIM8 capture/compare enable register (TIMx_CCER) ).

0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0). 1: When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1)

Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK

bits in TIMx_BDTR register).

Bits 9:8 LOCK[1:0] : Lock configuration

These bits offer a write protection against software errors.

00: LOCK OFF - No bit is write protected.

01: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written.

11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written.

Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register

has been written, their content is frozen until the next reset.

584/1749 RM0090 Rev 18

--- Page 195 ---

**RM0090**

**Advanced-control timers (TIM1 and TIM8)**

Bits 7:0 DTG[7:0] : Dead-time generator setup

This bit-field defines the duration of the dead-time inserted between the complementary

outputs. DT correspond to this duration.

dtg with t dtg =t DTS .

DTG[7:5]=0xx => DT=DTG[7:0]x t

DTG[7:5]=10x => DT=(64+DTG[5:0])xt DTG[7:5]=110 => DT=(32+DTG[4:0])xt DTG[7:5]=111 => DT=(32+DTG[4:0])xt Example if T DTS

dtg with T dtg =2xt DTS .

dtg with T dtg =8xt DTS .

dtg with T dtg =16xt DTS .

=125ns (8MHz), dead-time possible values are:

0 to 15875 ns by 125 ns steps,

16 us to 31750 ns by 250 ns steps,

32 us to 63us by 1 us steps,

64 us to 126 us by 2 us steps

Note:

This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed

(LOCK bits in TIMx_BDTR register).

**17.4.19 TIM1 and TIM8 DMA control register (TIMx_DCR)**

Address offset: 0x48

Reset value: 0x0000

15 14 13 12 11 10 9 8

7 6 5 4 3 2 1 0

| Reserved | DBL[4:0] | Reserved | DBA[4:0] |
|---|---|---|---|
| rw |  |  |  |


Bits 15:13 Reserved, must be kept at reset value.

Bits 12:8 DBL[4:0] : DMA burst length

This 5-bit vector defines the number of DMA transfers (the timer detects a burst transfer when a read or a write access to the TIMx_DMAR register address is performed).

the TIMx_DMAR address)

00000: 1 transfer

00001: 2 transfers

00010: 3 transfers

...

10001: 18 transfers

Bits 7:5 Reserved, must be kept at reset value.

Bits 4:0 DBA[4:0] : DMA base address

This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the

address of the TIMx_CR1 register.

Example:

00000: TIMx_CR1,

00001: TIMx_CR2,

00010: TIMx_SMCR,

...

**Example:**

Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address.

RM0090 Rev 18 585/1749

588
--- Page 1 ---

Advanced-control timers (TIM1 and TIM8)
RM0090

