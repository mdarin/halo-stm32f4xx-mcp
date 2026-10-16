**14.4 Dual DAC channel conversion**

To efficiently use the bus bandwidth in applications that require the two DAC channels at the same time, three dual registers are implemented: DHR8RD, DHR12RD and DHR12LD. A unique register access is then required to drive both DAC channels at the same time. Eleven possible conversion modes are possible using the two DAC channels and these dual registers. All the conversion modes can nevertheless be obtained using separate DHRx registers if needed.

All modes are described in the paragraphs below.



--- Page 51 ---

**RM0090 Digital-to-analog converter (DAC)**

**14.4.1 Independent trigger without wave generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure different trigger sources by setting different values in the TSEL1[2:0] and

TSEL2[2:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a DAC channel1 trigger arrives, the DHR1 register is transferred into DAC_DOR1 (three APB1 clock cycles later).

When a DAC channel2 trigger arrives, the DHR2 register is transferred into DAC_DOR2 (three APB1 clock cycles later).

**14.4.2 Independent trigger with single LFSR generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure different trigger sources by setting different values in the TSEL1[2:0] and

TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “01” and the same LFSR mask

value in the MAMPx[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DHR12RD, DHR12LD

or DHR8RD)

When a DAC channel1 trigger arrives, the LFSR1 counter, with the same mask, is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). Then the LFSR1 counter is updated.

When a DAC channel2 trigger arrives, the LFSR2 counter, with the same mask, is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). Then the LFSR2 counter is updated.

**14.4.3 Independent trigger with different LFSR generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure different trigger sources by setting different values in the TSEL1[2:0] and

TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “01” and set different LFSR masks

values in the MAMP1[3:0] and MAMP2[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a DAC channel1 trigger arrives, the LFSR1 counter, with the mask configured by MAMP1[3:0], is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). Then the LFSR1 counter is updated.

When a DAC channel2 trigger arrives, the LFSR2 counter, with the mask configured by MAMP2[3:0], is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). Then the LFSR2 counter is updated.



454

--- Page 52 ---

**Digital-to-analog converter (DAC) RM0090**

**14.4.4 Independent trigger with single triangle generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure different trigger sources by setting different values in the TSEL1[2:0] and

TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “1x” and the same maximum

amplitude value in the MAMPx[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a DAC channel1 trigger arrives, the DAC channel1 triangle counter, with the same triangle amplitude, is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). The DAC channel1 triangle counter is then updated.

When a DAC channel2 trigger arrives, the DAC channel2 triangle counter, with the same triangle amplitude, is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). The DAC channel2 triangle counter is then updated.

**14.4.5 Independent trigger with different triangle generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure different trigger sources by setting different values in the TSEL1[2:0] and

TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “1x” and set different maximum

amplitude values in the MAMP1[3:0] and MAMP2[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a DAC channel1 trigger arrives, the DAC channel1 triangle counter, with a triangle amplitude configured by MAMP1[3:0], is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). The DAC channel1 triangle counter is then updated.

When a DAC channel2 trigger arrives, the DAC channel2 triangle counter, with a triangle amplitude configured by MAMP2[3:0], is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). The DAC channel2 triangle counter is then updated.

**14.4.6 Simultaneous software start**

To configure the DAC in this conversion mode, the following sequence is required:

• Load the dual DAC channel data to the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

In this configuration, one APB1 clock cycle later, the DHR1 and DHR2 registers are transferred into DAC_DOR1 and DAC_DOR2, respectively.



--- Page 53 ---

**RM0090 Digital-to-analog converter (DAC)**

**14.4.7 Simultaneous trigger without wave generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure the same trigger source for both DAC channels by setting the same value in

the TSEL1[2:0] and TSEL2[2:0] bits

• Load the dual DAC channel data to the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a trigger arrives, the DHR1 and DHR2 registers are transferred into DAC_DOR1 and DAC_DOR2, respectively (after three APB1 clock cycles).

**14.4.8 Simultaneous trigger with single LFSR generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure the same trigger source for both DAC channels by setting the same value in

the TSEL1[2:0] and TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “01” and the same LFSR mask

value in the MAMPx[3:0] bits

• Load the dual DAC channel data to the desired DHR register (DHR12RD, DHR12LD or

DHR8RD)

When a trigger arrives, the LFSR1 counter, with the same mask, is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). The LFSR1 counter is then updated. At the same time, the LFSR2 counter, with the same mask, is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). The LFSR2 counter is then updated.

**14.4.9 Simultaneous trigger with different LFSR generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure the same trigger source for both DAC channels by setting the same value in

the TSEL1[2:0] and TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “01” and set different LFSR mask

values using the MAMP1[3:0] and MAMP2[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a trigger arrives, the LFSR1 counter, with the mask configured by MAMP1[3:0], is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). The LFSR1 counter is then updated.

At the same time, the LFSR2 counter, with the mask configured by MAMP2[3:0], is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). The LFSR2 counter is then updated.



454

--- Page 54 ---

**Digital-to-analog converter (DAC) RM0090**

**14.4.10 Simultaneous trigger with single triangle generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure the same trigger source for both DAC channels by setting the same value in

the TSEL1[2:0] and TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “1x” and the same maximum

amplitude value using the MAMPx[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a trigger arrives, the DAC channel1 triangle counter, with the same triangle amplitude, is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). The DAC channel1 triangle counter is then updated.

At the same time, the DAC channel2 triangle counter, with the same triangle amplitude, is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). The DAC channel2 triangle counter is then updated.

**14.4.11 Simultaneous trigger with different triangle generation**

To configure the DAC in this conversion mode, the following sequence is required:

• Set the two DAC channel trigger enable bits TEN1 and TEN2

• Configure the same trigger source for both DAC channels by setting the same value in

the TSEL1[2:0] and TSEL2[2:0] bits

• Configure the two DAC channel WAVEx[1:0] bits as “1x” and set different maximum

amplitude values in the MAMP1[3:0] and MAMP2[3:0] bits

• Load the dual DAC channel data into the desired DHR register (DAC_DHR12RD,

DAC_DHR12LD or DAC_DHR8RD)

When a trigger arrives, the DAC channel1 triangle counter, with a triangle amplitude configured by MAMP1[3:0], is added to the DHR1 register and the sum is transferred into DAC_DOR1 (three APB1 clock cycles later). Then the DAC channel1 triangle counter is updated.

At the same time, the DAC channel2 triangle counter, with a triangle amplitude configured by MAMP2[3:0], is added to the DHR2 register and the sum is transferred into DAC_DOR2 (three APB1 clock cycles later). Then the DAC channel2 triangle counter is updated.



--- Page 55 ---

**RM0090 Digital-to-analog converter (DAC)**

