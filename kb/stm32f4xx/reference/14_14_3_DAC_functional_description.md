**14.3 DAC functional description**

**14.3.1 DAC channel enable**

Each DAC channel can be powered on by setting its corresponding ENx bit in the DAC_CR register. The DAC channel is then enabled after a startup time t WAKEUP .

Note: The ENx bit enables the analog DAC Channelx macrocell only. The DAC Channelx digital interface is enabled even if the ENx bit is reset.

**14.3.2 DAC output buffer enable**

The DAC integrates two output buffers that can be used to reduce the output impedance, and to drive external loads directly without having to add an external operational amplifier. Each DAC channel output buffer can be enabled and disabled using the corresponding BOFFx bit in the DAC_CR register.

**14.3.3 DAC data format**

Depending on the selected configuration mode, the data have to be written into the specified register as described below:

• Single DAC channelx, there are three possibilities:

– 8-bit right alignment: the software has to load data into the DAC_DHR8Rx [7:0]

bits (stored into the DHRx[11:4] bits)

– 12-bit left alignment: the software has to load data into the DAC_DHR12Lx [15:4]

bits (stored into the DHRx[11:0] bits)

– 12-bit right alignment: the software has to load data into the DAC_DHR12Rx [11:0]

bits (stored into the DHRx[11:0] bits)

Depending on the loaded DAC_DHRyyyx register, the data written by the user is shifted and stored into the corresponding DHRx (data holding registerx, which are internal non-memory- mapped registers). The DHRx register is then loaded into the DORx register either





454

--- Page 46 ---

**Digital-to-analog converter (DAC) RM0090**

**Figure 65. Data registers in single DAC channel mode**

31 24 15 7 0

8-bit right aligned

12-bit left aligned

12-bit right aligned

ai14710b

• Dual DAC channels, there are three possibilities:

– 8-bit right alignment: data for DAC channel1 to be loaded into the DAC_DHR8RD

[7:0] bits (stored into the DHR1[11:4] bits) and data for DAC channel2 to be loaded into the DAC_DHR8RD [15:8] bits (stored into the DHR2[11:4] bits)

– 12-bit left alignment: data for DAC channel1 to be loaded into the DAC_DHR12LD

[15:4] bits (stored into the DHR1[11:0] bits) and data for DAC channel2 to be loaded into the DAC_DHR12LD [31:20] bits (stored into the DHR2[11:0] bits)

– 12-bit right alignment: data for DAC channel1 to be loaded into the

DAC_DHR12RD [11:0] bits (stored into the DHR1[11:0] bits) and data for DAC channel2 to be loaded into the DAC_DHR12LD [27:16] bits (stored into the DHR2[11:0] bits)

Depending on the loaded DAC_DHRyyyD register, the data written by the user is shifted and stored into DHR1 and DHR2 (data holding registers, which are internal non-memory- mapped registers). The DHR1 and DHR2 registers are then loaded into the DOR1 and DOR2 registers, respectively, either automatically, by software trigger or by an external event trigger.

**Figure 66. Data registers in dual DAC channel mode**

| 1 3124 24 15 15 77 0 0 8-bit right aligned 8-bit right ali 12-bit left aligned 12-bit left ali 12-bit right aligned 12-bit right a ai14709 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | 8- |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 1 |


ai14709b

**14.3.4 DAC conversion**

The DAC_DORx cannot be written directly and any data transfer to the DAC channelx must be performed by loading the DAC_DHRx register (write to DAC_DHR8Rx, DAC_DHR12Lx, DAC_DHR12Rx, DAC_DHR8RD, DAC_DHR12LD or DAC_DHR12LD).

Data stored in the DAC_DHRx register are automatically transferred to the DAC_DORx register after one APB1 clock cycle, if no hardware trigger is selected (TENx bit in DAC_CR register is reset). However, when a hardware trigger is selected (TENx bit in DAC_CR register is set) and a trigger occurs, the transfer is performed three APB1 clock cycles later.



--- Page 47 ---

**RM0090 Digital-to-analog converter (DAC)**

When DAC_DORx is loaded with the DAC_DHRx contents, the analog output voltage becomes available after a time t SETTLING that depends on the power supply voltage and the analog output load.

**Figure 67. Timing diagram for conversion with trigger disabled TEN = 0**

APB1_CLK

DHR 0x1AC

Output voltage DOR 0x1AC available on DAC_OUT pin

t SETTLING

ai14711c

**14.3.5 DAC output voltage**

Digital inputs are converted to output voltages on a linear conversion between 0 and V REF+ . The analog output voltages on each DAC channel pin are determined by the following equation:

DACoutput = V REF × DOR ------------- 4096

**14.3.6 DAC trigger selection**

If the TENx control bit is set, conversion can then be triggered by an external event (timer counter, external interrupt line). The TSELx[2:0] control bits determine which out of 8 possible events will trigger conversion as shown in Table 75 .

**Table 75. External triggers**

| Source | Type | TSEL[2:0] |
|---|---|---|
| Timer 6 TRGO event | Internal signal from on-chip timers | 000 |
| Timer 8 TRGO event | 001 |  |
| Timer 7 TRGO event | 010 |  |
| Timer 5 TRGO event | 011 |  |
| Timer 2 TRGO event | 100 |  |
| Timer 4 TRGO event | 101 |  |
| EXTI line9 | External pin | 110 |
| SWTRIG | Software control bit | 111 |


Each time a DAC interface detects a rising edge on the selected timer TRGO output, or on the selected external interrupt line 9, the last data stored into the DAC_DHRx register are transferred into the DAC_DORx register. The DAC_DORx register is updated three APB1 cycles after the trigger occurs.



454

--- Page 48 ---

**Digital-to-analog converter (DAC) RM0090**

If the software trigger is selected, the conversion starts once the SWTRIG bit is set. SWTRIG is reset by hardware once the DAC_DORx register has been loaded with the DAC_DHRx register contents.

Note: TSELx[2:0] bit cannot be changed when the ENx bit is set.

When software trigger is selected, the transfer from the DAC_DHRx register to the DAC_DORx register takes only one APB1 clock cycle.

**14.3.7 DMA request**

Each DAC channel has a DMA capability. Two DMA channels are used to service DAC channel DMA requests.

A DAC DMA request is generated when an external trigger (but not a software trigger) occurs while the DMAENx bit is set. The value of the DAC_DHRx register is then transferred into the DAC_DORx register.

In dual mode, if both DMAENx bits are set, two DMA requests are generated. If only one DMA request is needed, the user should set only the corresponding DMAENx bit. In this way, the application can manage both DAC channels in dual mode by using one DMA request and a unique DMA channel.

**DMA underrun**

The DAC DMA request is not queued so that if a second external trigger arrives before the acknowledgement for the first external trigger is received (first request), then no new request is issued and the DMA channelx underrun flag DMAUDRx in the DAC_SR register is set, reporting the error condition. DMA data transfers are then disabled and no further DMA request is treated. The DAC channelx continues to convert old data.

The software should clear the DMAUDRx flag by writing “1”, clear the DMAEN bit of the used DMA stream and re-initialize both DMA and DAC channelx to restart the transfer correctly. The software should modify the DAC trigger conversion frequency or lighten the DMA workload to avoid a new DMA underrun. Finally, the DAC conversion could be resumed by enabling both DMA data transfer and conversion trigger.

For each DAC channelx, an interrupt is also generated if its corresponding DMAUDRIEx bit in the DAC_CR register is enabled.

**14.3.8 Noise generation**

In order to generate a variable-amplitude pseudonoise, an LFSR (linear feedback shift register) is available. DAC noise generation is selected by setting WAVEx[1:0] to “01”. The preloaded value in LFSR is 0xAAA. This register is updated three APB1 clock cycles after each trigger event, following a specific calculation algorithm.

438/1749 RM0090 Rev 18

--- Page 49 ---

**RM0090 Digital-to-analog converter (DAC)**

**Figure 68. DAC LFSR register calculation algorithm**

XOR

X 6 X 4 X X 0 X 12

11 10 9 8 7 6 5 4 3 2 1 0

12

NOR

ai14713c

The LFSR value, that may be masked partially or totally by means of the MAMPx[3:0] bits in the DAC_CR register, is added up to the DAC_DHRx contents without overflow and this value is then stored into the DAC_DORx register.

If LFSR is 0x0000, a ‘1 is injected into it (antilock-up mechanism).

It is possible to reset LFSR wave generation by resetting the WAVEx[1:0] bits.

**Figure 69. DAC conversion (SW trigger enabled) with LFSR wave generation**

APB1_CLK

| 0x00 |  |  |
|---|---|---|
| 0xAAA |  |  |


DHR

DOR 0xD55

SWTRIG

ai14714b

Note: The DAC trigger must be enabled for noise generation by setting the TENx bit in the DAC_CR register.

**14.3.9 Triangle-wave generation**

It is possible to add a small-amplitude triangular waveform on a DC or slowly varying signal. DAC triangle-wave generation is selected by setting WAVEx[1:0] to “10”. The amplitude is configured through the MAMPx[3:0] bits in the DAC_CR register. An internal triangle counter is incremented three APB1 clock cycles after each trigger event. The value of this counter is then added to the DAC_DHRx register without overflow and the sum is stored into the DAC_DORx register. The triangle counter is incremented as long as it is less than the maximum amplitude defined by the MAMPx[3:0] bits. Once the configured amplitude is reached, the counter is decremented down to 0, then incremented again and so on.



454

--- Page 50 ---

**Digital-to-analog converter (DAC) RM0090**

It is possible to reset triangle wave generation by resetting the WAVEx[1:0] bits.

**Figure 70. DAC triangle wave generation**

MAMPx[3:0] max amplitude

+ DAC_DHRx base value Incrementation Decrementation

DAC_DHRx base value

0

ai14715c

**Figure 71. DAC conversion (SW trigger enabled) with triangle wave generation**

APB1_CLK

| 0x00 |  |  |
|---|---|---|
| 0xAAA |  |  |


DHR

DOR 0xD55

SWTRIG

ai14714b

Note: The DAC trigger must be enabled for noise generation by setting the TENx bit in the DAC_CR register.

The MAMPx[3:0] bits must be configured before enabling the DAC, otherwise they cannot be changed.

