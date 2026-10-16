### 30.3.4 Fractional baud rate generation

The baud rate for the receiver and transmitter (Rx and Tx) are both set to the same value as programmed in the Mantissa and Fraction values of USARTDIV.

**Equation 1: Baud rate for standard USART (SPI mode included)**

Tx/Rx baud = fCK / (8 × (2 − OVER8) × USARTDIV)

**Equation 2: Baud rate in Smartcard, LIN and IrDA modes**

Tx/Rx baud = fCK / (16 × USARTDIV)

USARTDIV is an unsigned fixed point number that is coded on the USART_BRR register.

*   When OVER8=0, the fractional part is coded on 4 bits and programmed by the DIV_fraction[3:0] bits in the USART_BRR register
*   When OVER8=1, the fractional part is coded on 3 bits and programmed by the DIV_fraction[2:0] bits in the USART_BRR register, and bit DIV_fraction[3] must be kept cleared.

**Note:** The baud counters are updated to the new value in the baud registers after a write operation to USART_BRR. Hence the baud rate register value should not be changed during communication.

978/1749 RM0090 Rev 18 ST

--- Page 4 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

**How to derive USARTDIV from USART_BRR register values when OVER8=0**

*Example 1:*
If DIV_Mantissa = 0d27 and DIV_Fraction = 0d12 (USART_BRR = 0x1BC), then
Mantissa (USARTDIV) = 0d27
Fraction (USARTDIV) = 12/16 = 0d0.75
Therefore USARTDIV = 0d27.75

*Example 2:*
To program USARTDIV = 0d25.62
This leads to:
DIV_Fraction = 16*0d0.62 = 0d9.92
The nearest real number is 0d10 = 0xA
DIV_Mantissa = mantissa (0d25.620) = 0d25 = 0x19
Then, USART_BRR = 0x19A hence USARTDIV = 0d25.625

*Example 3:*
To program USARTDIV = 0d50.99
This leads to:
DIV_Fraction = 16*0d0.99 = 0d15.84
The nearest real number is 0d16 = 0x10 => overflow of DIV_frac[3:0] => carry must be added up to the mantissa
DIV_Mantissa = mantissa (0d50.990 + carry) = 0d51 = 0x33
Then, USART_BRR = 0x330 hence USARTDIV = 0d51.000

**How to derive USARTDIV from USART_BRR register values when OVER8=1**

*Example 1:*
If DIV_Mantissa = 0x27 and DIV_Fraction[2:0]= 0d6 (USART_BRR = 0x1B6), then
Mantissa (USARTDIV) = 0d27
Fraction (USARTDIV) = 6/8 = 0d0.75
Therefore USARTDIV = 0d27.75

*Example 2:*
To program USARTDIV = 0d25.62
This leads to:
DIV_Fraction = 8*0d0.62 = 0d4.96
The nearest real number is 0d5 = 0x5
DIV_Mantissa = mantissa (0d25.620) = 0d25 = 0x19

ST
RM0090 Rev 18 979/1749

--- Page 5 ---

**Universal synchronous asynchronous receiver transmitter (USART)**
RM0090

Then, USART_BRR = 0x195 => USARTDIV = 0d25.625

**Example 3:**
To program USARTDIV = 0d50.99
This leads to:
DIV_Fraction = 8*0d0.99 = 0d7.92
The nearest real number is 0d8 = 0x8 => overflow of the DIV_frac[2:0] => carry must be added up to the mantissa
DIV_Mantissa = mantissa (0d50.990 + carry) = 0d51 = 0x33
Then, USART_BRR = 0x0330 => USARTDIV = 0d51.000

**Table 134. Error calculation for programmed baud rates at fPCLK = 8 MHz or fPCLK = 12 MHz, oversampling by 16(1)**

| Oversampling by 16 (OVER8=0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate7 | fPCLK = 8 MHz | fPCLK = 12 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired) B.rate / Desired B.rate | Actual | Value programmed in the baud rate register | % Error |
| 1 | 1.2 KBps | 416.6875 | 0 | 1.2 KBps | 625 | 0 |  |
| 2 | 2.4 KBps | 208.3125 | 0.01 | 2.4 KBps | 312.5 | 0 |  |
| 3 | 9.6 KBps | 9.604 KBps | 52.0625 | 0.04 | 9.6 KBps | 78.125 | 0 |
| 4 | 19.2 KBps | 19.185 KBps | 26.0625 | 0.08 | 19.2 KBps | 39.0625 | 0 |
| 5 | 38.4 KBps | 38.462 KBps | 13 | 0.16 | 38.339 KBps | 19.5625 | 0.16 |
| 6 | 57.6 KBps | 57.554 KBps | 8.6875 | 0.08 | 57.692 KBps | 13 | 0.16 |
| 7 | 115.2 KBps | 115.942 KBps | 4.3125 | 0.64 | 115.385 KBps | 6.5 | 0.16 |
| 8 | 230.4 KBps | 228.571 KBps | 2.1875 | 0.79 | 230.769 KBps | 3.25 | 0.16 |
| 9 | 460.8 KBps | 470.588 KBps | 1.0625 | 2.12 | 461.538 KBps | 1.625 | 0.16 |
| 10 | 921.6 KBps | NA |  |  |  |  |  |
| 11 | 2 MBps | NA |  |  |  |  |  |
| 12 | 3 MBps | NA |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.

980/1749
RM0090 Rev 18

--- Page 6 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Table 135. Error calculation for programmed baud rates at fPCLK = 8 MHz or fPCLK =12 MHz, oversampling by 8(1)

| Oversampling by 8 (OVER8 = 1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 8 MHz | fPCLK = 12 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired) B.rate / Desired B.rate | Actual | Value programmed in the baud rate register | % Error |
| 1 | 1.2 KBps | 833.375 | 0 | 1.2 KBps | 1250 | 0 |  |
| 2 | 2.4 KBps | 416.625 | 0.01 | 2.4 KBps | 625 | 0 |  |
| 3 | 9.6 KBps | 9.604 KBps | 104.125 | 0.04 | 9.6 KBps | 156.25 | 0 |
| 4 | 19.2 KBps | 19.185 KBps | 52.125 | 0.08 | 19.2 KBps | 78.125 | 0 |
| 5 | 38.4 KBps | 38.462 KBps | 26 | 0.16 | 38.339 KBps | 39.125 | 0.16 |
| 6 | 57.6 KBps | 57.554 KBps | 17.375 | 0.08 | 57.692 KBps | 26 | 0.16 |
| 7 | 115.2 KBps | 115.942 KBps | 8.625 | 0.64 | 115.385 KBps | 13 | 0.16 |
| 8 | 230.4 KBps | 228.571 KBps | 4.375 | 0.79 | 230.769 KBps | 6.5 | 0.16 |
| 9 | 460.8 KBps | 470.588 KBps | 2.125 | 2.12 | 461.538 KBps | 3.25 | 0.16 |
| 10 | 921.6 KBps | 888.889 KBps | 1.125 | 3.55 | 923.077 KBps | 1.625 | 0.16 |
| 11 | 2 MBps | NA |  |  |  |  |  |
| 12 | 3 MBps | NA |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.

Table 136. Error calculation for programmed baud rates at fPCLK = 16 MHz or fPCLK = 24 MHz, oversampling by 16(1)

| Oversampling by 16 (OVER8 = 0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 16 MHz | fPCLK = 24 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired) B.rate / Desired B.rate | Actual | Value programmed in the baud rate register | % Error |
| 1 | 1.2 KBps | 833.3125 | 0 | 1.2 | 1250 | 0 |  |
| 2 | 2.4 KBps | 416.6875 | 0 | 2.4 | 625 | 0 |  |
| 3 | 9.6 KBps | 9.598 KBps | 104.1875 | 0.02 | 9.6 | 156.25 | 0 |
| 4 | 19.2 KBps | 19.208 KBps | 52.0625 | 0.04 | 19.2 | 78.125 | 0 |
| 5 | 38.4 KBps | 38.369 KBps | 26.0625 | 0.08 | 38.4 | 39.0625 | 0 |
| 6 | 57.6 KBps | 57.554 KBps | 17.375 | 0.08 | 57.554 | 26.0625 | 0.08 |


ST
RM0090 Rev 18
981/1749

--- Page 7 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

**Table 136. Error calculation for programmed baud rates at fPCLK = 16 MHz or fPCLK = 24 MHz, oversampling by 16(1) (continued)**

| Oversampling by 16 (OVER8 = 0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 16 MHz | fPCLK = 24 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired) B.rate / Desired B.rate | Actual | Value programmed in the baud rate register | % Error |
| 7 | 115.2 KBps | 115.108 KBps | 8.6875 | 0.08 | 115.385 | 13 | 0.16 |
| 8 | 230.4 KBps | 231.884 KBps | 4.3125 | 0.64 | 230.769 | 6.5 | 0.16 |
| 9 | 460.8 KBps | 457.143 KBps | 2.1875 | 0.79 | 461.538 | 3.25 | 0.16 |
| 10 | 921.6 KBps | 941.176 KBps | 1.0625 | 2.12 | 923.077 | 1.625 | 0.16 |
| 11 | 2 MBps | NA |  |  |  |  |  |
| 12 | 3 MBps | NA |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.

**Table 137. Error calculation for programmed baud rates at fPCLK = 16 MHz or fPCLK = 24 MHz, oversampling by 8(1)**

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 16 MHz | fPCLK = 24 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired) B.rate / Desired B.rate | Actual | Value programmed in the baud rate register | % Error |
| 1 | 1.2 KBps | 1666.625 | 0 | 1.2 KBps | 2500 | 0 |  |
| 2 | 2.4 KBps | 833.375 | 0 | 2.4 KBps | 1250 | 0 |  |
| 3 | 9.6 KBps | 9.598 KBps | 208.375 | 0.02 | 9.6 KBps | 312.5 | 0 |
| 4 | 19.2 KBps | 19.208 KBps | 104.125 | 0.04 | 19.2 KBps | 156.25 | 0 |
| 5 | 38.4 KBps | 38.369 KBps | 52.125 | 0.08 | 38.4 KBps | 78.125 | 0 |
| 6 | 57.6 KBps | 57.554 KBps | 34.75 | 0.08 | 57.554 KBps | 52.125 | 0.08 |
| 7 | 115.2 KBps | 115.108 KBps | 17.375 | 0.08 | 115.385 KBps | 26 | 0.16 |
| 8 | 230.4 KBps | 231.884 KBps | 8.625 | 0.64 | 230.769 KBps | 13 | 0.16 |
| 9 | 460.8 KBps | 457.143 KBps | 4.375 | 0.79 | 461.538 KBps | 6.5 | 0.16 |
| 10 | 921.6 KBps | 941.176 KBps | 2.125 | 2.12 | 923.077 KBps | 3.25 | 0.16 |
| 11 | 2 MBps | 2000 KBps | 1 | 0 | 2000 KBps | 1.5 | 0 |
| 12 | 3 MBps | NA | 3000 KBps | 1 | 0 |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.

982/1749 RM0090 Rev 18

--- Page 8 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Table 138. Error calculation for programmed baud rates at fPCLK = 8 MHz or fPCLK = 16 MHz, oversampling by 16(1)

| Oversampling by 16 (OVER8=0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 8 MHz | fPCLK = 16 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 1. | 2.4 KBps | 2.400 KBps | 208.3125 | 0.00% | 2.400 KBps | 416.6875 | 0.00% |
| 2. | 9.6 KBps | 9.604 KBps | 52.0625 | 0.04% | 9.598 KBps | 104.1875 | 0.02% |
| 3. | 19.2 KBps | 19.185 KBps | 26.0625 | 0.08% | 19.208 KBps | 52.0625 | 0.04% |
| 4. | 57.6 KBps | 57.554 KBps | 8.6875 | 0.08% | 57.554 KBps | 17.3750 | 0.08% |
| 5. | 115.2 KBps | 115.942 KBps | 4.3125 | 0.64% | 115.108 KBps | 8.6875 | 0.08% |
| 6. | 230.4 KBps | 228.571 KBps | 2.1875 | 0.79% | 231.884 KBps | 4.3125 | 0.64% |
| 7. | 460.8 KBps | 470.588 KBps | 1.0625 | 2.12% | 457.143 KBps | 2.1875 | 0.79% |
| 8. | 896 KBps | NA | 888.889 KBps | 1.1250 | 0.79% |  |  |
| 9. | 921.6 KBps | NA | 941.176 KBps | 1.0625 | 2.12% |  |  |
| 10. | 1.792 MBps | NA |  |  |  |  |  |
| 11. | 1.8432 MBps | NA |  |  |  |  |  |
| 12. | 3.584 MBps | NA |  |  |  |  |  |
| 13. | 3.6864 MBps | NA |  |  |  |  |  |
| 14. | 7.168 MBps | NA |  |  |  |  |  |
| 15. | 7.3728 MBps | NA |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.

Table 139. Error calculation for programmed baud rates at fPCLK = 8 MHz or fPCLK = 16 MHz, oversampling by 8(1)

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 8 MHz | fPCLK = 16 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 1. | 2.4 KBps | 2.400 KBps | 416.625 | 0.01% | 2.400 KBps | 833.375 | 0.00% |
| 2. | 9.6 KBps | 9.604 KBps | 104.125 | 0.04% | 9.598 KBps | 208.375 | 0.02% |
| 3. | 19.2 KBps | 19.185 KBps | 52.125 | 0.08% | 19.208 KBps | 104.125 | 0.04% |


ST
RM0090 Rev 18
983/1749

--- Page 9 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

Table 139. Error calculation for programmed baud rates at fPCLK = 8 MHz or fPCLK = 16 MHz, oversampling by 8(1) (continued)

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 8 MHz | fPCLK = 16 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 4. | 57.6 KBps | 57.557 KBps | 17.375 | 0.08% | 57.554 KBps | 34.750 | 0.08% |
| 5. | 115.2 KBps | 115.942 KBps | 8.625 | 0.64% | 115.108 KBps | 17.375 | 0.08% |
| 6. | 230.4 KBps | 228.571 KBps | 4.375 | 0.79% | 231.884 KBps | 8.625 | 0.64% |
| 7. | 460.8 KBps | 470.588 KBps | 2.125 | 2.12% | 457.143 KBps | 4.375 | 0.79% |
| 8. | 896 KBps | 888.889 KBps | 1.125 | 0.79% | 888.889 KBps | 2.250 | 0.79% |
| 9. | 921.6 KBps | 888.889 KBps | 1.125 | 3.55% | 941.176 KBps | 2.125 | 2.12% |
| 10. | 1.792 MBps | NA | 1.7777 MBps | 1.125 | 0.79% |  |  |
| 11. | 1.8432 MBps | NA | 1.7777 MBps | 1.125 | 3.55% |  |  |
| 12. | 3.584 MBps | NA |  |  |  |  |  |
| 13. | 3.6864 MBps | NA |  |  |  |  |  |
| 14. | 7.168 MBps | NA |  |  |  |  |  |
| 15. | 7.3728 MBps | NA |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.

Table 140. Error calculation for programmed baud rates at fPCLK = 30 MHz or fPCLK = 60 MHz, oversampling by 16(1)(2)

| Oversampling by 16 (OVER8=0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 30 MHz | fPCLK = 60 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 1. | 2.4 KBps | 2.400 KBps | 781.2500 | 0.00% | 2.400 KBps | 1562.5000 | 0.00% |
| 2. | 9.6 KBps | 9.600 KBps | 195.3125 | 0.00% | 9.600 KBps | 390.6250 | 0.00% |
| 3. | 19.2 KBps | 19.194 KBps | 97.6875 | 0.03% | 19.200 KBps | 195.3125 | 0.00% |
| 4. | 57.6 KBps | 57.582 KBps | 32.5625 | 0.03% | 57.582 KBps | 65.1250 | 0.03% |
| 5. | 115.2 KBps | 115.385 KBps | 16.2500 | 0.16% | 115.163 KBps | 32.5625 | 0.03% |
| 6. | 230.4 KBps | 230.769 KBps | 8.1250 | 0.16% | 230.769 KBps | 16.2500 | 0.16% |
| 7. | 460.8 KBps | 461.538 KBps | 4.0625 | 0.16% | 461.538 KBps | 8.1250 | 0.16% |
| 8. | 896 KBps | 909.091 KBps | 2.0625 | 1.46% | 895.522 KBps | 4.1875 | 0.05% |


984/1749 RM0090 Rev 18 ST

--- Page 10 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

Table 140. Error calculation for programmed baud rates at fPCLK = 30 MHz or fPCLK = 60 MHz, oversampling by 16(1)(2) (continued)

| Oversampling by 16 (OVER8=0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 30 MHz | fPCLK = 60 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 9. | 921.6 KBps | 909.091 KBps | 2.0625 | 1.36% | 923.077 KBps | 4.0625 | 0.16% |
| 10. | 1.792 MBps | 1.1764 MBps | 1.0625 | 1.52% | 1.8182 MBps | 2.0625 | 1.36% |
| 11. | 1.8432 MBps | 1.8750 MBps | 1.0000 | 1.73% | 1.8182 MBps | 2.0625 | 1.52% |
| 12. | 3.584 MBps | NA | 3.2594 MBps | 1.0625 | 1.52% |  |  |
| 13. | 3.6864 MBps | NA | 3.7500 MBps | 1.0000 | 1.73% |  |  |
| 14. | 7.168 MBps | NA |  |  |  |  |  |
| 15. | 7.3728 MBps | NA |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.
2. Only USART1 and USART6 are clocked with PCLK2. Other USARTs are clocked with PCLK1. Refer to the device datasheets for the maximum values for PCLK1 and PCLK2.

Table 141. Error calculation for programmed baud rates at fPCLK = 30 MHz or fPCLK = 60 MHz, oversampling by 8(1) (2)

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 30 MHz | fPCLK =60 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 1. | 2.4 KBps | 2.400 KBps | 1562.5000 | 0.00% | 2.400 KBps | 3125.0000 | 0.00% |
| 2. | 9.6 KBps | 9.600 KBps | 390.6250 | 0.00% | 9.600 KBps | 781.2500 | 0.00% |
| 3. | 19.2 KBps | 19.194 KBps | 195.3750 | 0.03% | 19.200 KBps | 390.6250 | 0.00% |
| 4. | 57.6 KBps | 57.582 KBps | 65.1250 | 0.16% | 57.582 KBps | 130.2500 | 0.03% |
| 5. | 115.2 KBps | 115.385 KBps | 32.5000 | 0.16% | 115.163 KBps | 65.1250 | 0.03% |
| 6. | 230.4 KBps | 230.769 KBps | 16.2500 | 0.16% | 230.769 KBps | 32.5000 | 0.16% |
| 7. | 460.8 KBps | 461.538 KBps | 8.1250 | 0.16% | 461.538 KBps | 16.2500 | 0.16% |
| 8. | 896 KBps | 909.091 KBps | 4.1250 | 1.46% | 895.522 KBps | 8.3750 | 0.05% |
| 9. | 921.6 KBps | 909.091 KBps | 4.1250 | 1.36% | 923.077 KBps | 8.1250 | 0.16% |
| 10. | 1.792 MBps | 1.7647 MBps | 2.1250 | 1.52% | 1.8182 MBps | 4.1250 | 1.46% |


ST
RM0090 Rev 18
985/1749

--- Page 11 ---

Universal synchronous asynchronous receiver transmitter (USART)
RM0090

Table 141. Error calculation for programmed baud rates at fPCLK = 30 MHz or fPCLK = 60 MHz, oversampling by 8(1) (2) (continued)

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 30 MHz | fPCLK =60 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 11. | 1.8432 MBps | 1.8750 MBps | 2.0000 | 1.73% | 1.8182 MBps | 4.1250 | 1.36% |
| 12. | 3.584 MBps | 3.7500 MBps | 1.0000 | 4.63% | 3.5294 MBps | 2.1250 | 1.52% |
| 13. | 3.6864 MBps | 3.7500 MBps | 1.0000 | 1.73% | 3.7500 MBps | 2.0000 | 1.73% |
| 14. | 7.168 MBps | NA | 7.5000 MBps | 1.0000 | 4.63% |  |  |
| 15. | 7.3728 MBps | NA | 7.5000 MBps | 1.0000 | 1.73% |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.
2. Only USART1 and USART6 are clocked with PCLK2. Other USARTs are clocked with PCLK1. Refer to the device datasheets for the maximum values for PCLK1 and PCLK2.

Table 142. Error calculation for programmed baud rates at fPCLK = 42 MHz or fPCLK = 84 Hz, oversampling by 16(1)(2)

| Oversampling by 16 (OVER8=0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 42 MHz | fPCLK = 84 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 1. | 1.2 KBps | 2187.5 | 0 | 1.2 KBps | NA | 0 |  |
| 2. | 2.4 KBps | 1093.75 | 0 | 2.4 KBps | 2187.5 | 0 |  |
| 3. | 9.6 KBps | 273.4375 | 0 | 9.6 KBps | 546.875 | 0 |  |
| 4. | 19.2 KBps | 19.195 KBps | 136.75 | 0.02 | 19.2 KBps | 273.4375 | 0 |
| 5. | 38.4 KBps | 38.391 KBps | 68.375 | 0.02 | 38.391 KBps | 136.75 | 0.02 |
| 6. | 57.6 KBps | 57.613 KBps | 45.5625 | 0.02 | 57.613 KBps | 91.125 | 0.02 |
| 7. | 115.2 KBps | 115.068 KBps | 22.8125 | 0.11 | 115.226 KBps | 45.5625 | 0.02 |
| 8. | 230.4 KBps | 230.769 KBps | 11.375 | 0.16 | 230.137 KBps | 22.8125 | 0.11 |
| 9. | 460.8 KBps | 461.538 KBps | 5.6875 | 0.16 | 461.538 KBps | 11.375 | 0.16 |
| 10. | 921.6 KBps | 913.043 KBps | 2.875 | 0.93 | 923.076 KBps | 5.6875 | 0.93 |
| 11. | 1.792 MBps | 1.826 MBps | 1.4375 | 1.9 | 1.787 MBps | 2.9375 | 0.27 |
| 12. | 1.8432 MBps | 1.826 MBps | 1.4375 | 0.93 | 1.826 MBps | 2.875 | 0.93 |
| 13. | 3.584 MBps | N.A | 3.652 MBps | 1.4375 | 1.9 |  |  |
| 14. | 3.6864 MBps | N.A | 3.652 MBps | 1.4375 | 0.93 |  |  |


986/1749
RM0090 Rev 18

--- Page 12 ---

RM0090
Universal synchronous asynchronous receiver transmitter (USART)

Table 142. Error calculation for programmed baud rates at fPCLK = 42 MHz or fPCLK = 84 Hz, oversampling by 16(1)(2) (continued)

| Oversampling by 16 (OVER8=0) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 42 MHz | fPCLK = 84 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | % Error = (Calculated - Desired)B.Rate /Desired B.Rate | Actual | Value programmed in the baud rate register | % Error |
| 15. | 7.168 MBps | N.A |  |  |  |  |  |
| 16. | 7.3728 MBps | N.A |  |  |  |  |  |
| 17. | 9 MBps | N.A |  |  |  |  |  |
| 18. | 10.5 MBps | N.A |  |  |  |  |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.
2. Only USART1 and USART6 are clocked with PCLK2. Other USARTs are clocked with PCLK1. Refer to the device datasheets for the maximum values for PCLK1 and PCLK2.

Table 143. Error calculation for programmed baud rates at fPCLK = 42 MHz or fPCLK = 84 MHz, oversampling by 8(1)(2)

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 42 MHz | fPCLK = 84 MHz |  |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | Actual | Value programmed in the baud rate register | % Error |  |
| 1. | 2.4 KBps | 2187.5 | 0 | 2.4 KBps | NA | 0 |  |
| 2. | 9.6 KBps | 546.875 | 0 | 9.6 KBps | 1093.75 | 0 |  |
| 3. | 19.2 KBps | 19.195 KBps | 273.5 | 0.02 | 19.2 KBps | 546.875 | 0 |
| 4. | 38.4 KBps | 38.391 KBps | 136.75 | 0.02 | 38.391 KBps | 273.5 | 0.02 |
| 5. | 57.6 KBps | 57.613 KBps | 91.125 | 0.02 | 57.613 KBps | 182.25 | 0.02 |
| 6. | 115.2 KBps | 115.068 KBps | 45.625 | 0.11 | 115.226 KBps | 91.125 | 0.02 |
| 7. | 230.4 KBps | 230.769 KBps | 22.75 | 0.11 | 230.137 KBps | 45.625 | 0.11 |
| 8. | 460.8 KBps | 461.538 KBps | 11.375 | 0.16 | 461.538 KBps | 22.75 | 0.16 |
| 9. | 921.6 KBps | 913.043 KBps | 5.75 | 0.93 | 923.076 KBps | 11.375 | 0.93 |
| 10. | 1.792 MBps | 1.826 MBps | 2.875 | 1.9 | 1.787MBps | 5.875 | 0.27 |
| 11. | 1.8432 MBps | 1.826 MBps | 2.875 | 0.93 | 1.826 MBps | 5.75 | 0.93 |
| 12. | 3.584 MBps | 3.5 MBps | 1.5 | 2.34 | 3.652 MBps | 2.875 | 1.9 |
| 13. | 3.6864 MBps | 3.82 MBps | 1.375 | 3.57 | 3.652 MBps | 2.875 | 0.93 |
| 14. | 7.168 MBps | N.A | 7 MBps | 1.5 | 2.34 |  |  |
| 15. | 7.3728 MBps | N.A | 7.636 MBps | 1.375 | 3.57 |  |  |


ST
RM0090 Rev 18
987/1749

--- Page 13 ---

Universal synchronous asynchronous receiver transmitter (USART) RM0090

**Table 143. Error calculation for programmed baud rates at fPCLK = 42 MHz or fPCLK = 84 MHz, oversampling by 8(1)(2) (continued)**

| Oversampling by 8 (OVER8=1) |  |  |  |  |  |  |
|---|---|---|---|---|---|---|
| Baud rate | fPCLK = 42 MHz | fPCLK = 84 MHz |  |  |  |  |
| S.No | Desired | Actual | Value programmed in the baud rate register | Actual | Value programmed in the baud rate register | % Error |
| 16. | 9 MBps | N.A | 9.333 MBps | 1.125 | 3.7 |  |
| 17. | 10.5 MBps | N.A | 10.5 MBps | 1 | 0 |  |


1. The lower the CPU clock the lower the accuracy for a particular baud rate. The upper limit of the achievable baud rate can be fixed with these data.
2. Only USART1 and USART6 are clocked with PCLK2. Other USARTs are clocked with PCLK1. Refer to the device datasheets for the maximum values for PCLK1 and PCLK2.

### 30.3.5 USART receiver tolerance to clock deviation

The USART asynchronous receiver works correctly only if the total clock system deviation is smaller than the USART receiver's tolerance. The causes which contribute to the total deviation are:

*   DTRA: Deviation due to the transmitter error (which also includes the deviation of the transmitter's local oscillator)
*   DQUANT: Error due to the baud rate quantization of the receiver
*   DREC: Deviation of the receiver's local oscillator
*   DTCL: Deviation due to the transmission line (generally due to the transceivers which can introduce an asymmetry between the low-to-high transition timing and the high-to-low transition timing)

DTRA + DQUANT + DREC + DTCL < USART receiver's tolerance

The USART receiver's tolerance to properly receive data is equal to the maximum tolerated deviation and depends on the following choices:

*   10- or 11-bit character length defined by the M bit in the USART_CR1 register
*   oversampling by 8 or 16 defined by the OVER8 bit in the USART_CR1 register
*   use of fractional baud rate or not
*   use of 1 bit or 3 bits to sample the data, depending on the value of the ONEBIT bit in the USART_CR3 register

**Table 144. USART receiver's tolerance when DIV fraction is 0**

| M bit | OVER8 bit = 0 | OVER8 bit = 1 |  |  |
|---|---|---|---|---|
| ONEBIT=0 | ONEBIT=1 | ONEBIT=0 | ONEBIT=1 |  |
| 0 | 3.75% | 4.375% | 2.50% | 3.75% |
| 1 | 3.41% | 3.97% | 2.27% | 3.41% |


988/1749 RM0090 Rev 18

--- Page 14 ---

RM0090 Universal synchronous asynchronous receiver transmitter (USART)

**Table 145. USART receiver tolerance when DIV_Fraction is different from 0**

| M bit | OVER8 bit = 0 | OVER8 bit = 1 |  |  |
|---|---|---|---|---|
| ONEBIT=0 | ONEBIT=1 | ONEBIT=0 | ONEBIT=1 |  |
| 0 | 3.33% | 3.88% | 2% | 3% |
| 1 | 3.03% | 3.53% | 1.82% | 2.73% |


**Note:** The figures specified in *Table 144* and *Table 145* may slightly differ in the special case when the received frames contain some Idle frames of exactly 10-bit times when M=0 (11-bit times when M=1).

