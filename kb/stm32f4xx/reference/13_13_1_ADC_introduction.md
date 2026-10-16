# 13 Analog-to-digital converter (ADC)

This section applies to the whole STM32F4xx family, unless otherwise specified.

## 13.1 ADC introduction

The 12-bit ADC is a successive approximation analog-to-digital converter. It has up to 19 multiplexed channels allowing it to measure signals from 16 external sources, two internal sources, and the VBAT channel. The A/D conversion of the channels can be performed in single, continuous, scan or discontinuous mode. The result of the ADC is stored into a left- or right-aligned 16-bit data register.

The analog watchdog feature allows the application to detect if the input voltage goes beyond the user-defined, higher or lower thresholds.

## 13.2 ADC main features

*   12-bit, 10-bit, 8-bit or 6-bit configurable resolution
*   Interrupt generation at the end of conversion, end of injected conversion, and in case of analog watchdog or overrun events
*   Single and continuous conversion modes
*   Scan mode for automatic conversion of channel 0 to channel 'n'
*   Data alignment with in-built data coherency
*   Channel-wise programmable sampling time
*   External trigger option with configurable polarity for both regular and injected conversions
*   Discontinuous mode
*   Dual/Triple mode (on devices with 2 ADCs or more)
*   Configurable DMA data storage in Dual/Triple ADC mode
*   Configurable delay between conversions in Dual/Triple interleaved mode
*   ADC conversion type (refer to the datasheets)
*   ADC supply requirements: 2.4 V to 3.6 V at full speed and down to 1.8 V at slower speed
*   ADC input range: VREF- ≤VIN ≤VREF+
*   DMA request generation during regular channel conversion

*Figure 44 shows the block diagram of the ADC.*

**Note:** VREF- if available (depending on package), must be tied to VSSA.

388/1749
RM0090 Rev 18
ST

--- Page 194 ---

RM0090
Analog-to-digital converter (ADC)

