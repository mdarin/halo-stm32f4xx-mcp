**13.9 Multi ADC mode**

In devices with two ADCs or more, the Dual (with two ADCs) and Triple (with three ADCs) ADC modes can be used (see Figure 51 ).

In multi ADC mode, the start of conversion is triggered alternately or simultaneously by the ADC1 master to the ADC2 and ADC3 slaves, depending on the mode selected by the MULTI[4:0] bits in the ADC_CCR register.

Note: In multi ADC mode, when configuring conversion trigger by an external event, the application must set trigger by the master only and disable trigger by slaves to prevent spurious triggers that would start unwanted slave conversions.

The four possible modes below are implemented:

• Injected simultaneous mode

• Regular simultaneous mode

• Interleaved mode

• Alternate trigger mode

It is also possible to use the previous modes combined in the following ways:

• Injected simultaneous mode + Regular simultaneous mode

• Regular simultaneous mode + Alternate trigger mode

Note: In multi ADC mode, the converted data can be read on the multi-mode data register (ADC_CDR). The status bits can be read in the multi-mode status register (ADC_CSR).



432

--- Page 12 ---

**Analog-to-digital converter (ADC) RM0090**

**Figure 51. Multi ADC block diagram (1)**













1. Although external triggers are present on ADC2 and ADC3 they are not shown in this diagram.

2. In the Dual ADC mode, the ADC3 slave part is not present.

3. In Triple ADC mode, the ADC common data register (ADC_CDR) contains the ADC1, ADC2 and ADC3’s regular converted data. All 32 register bits are used according to a selected storage order.

In Dual ADC mode, the ADC common data register (ADC_CDR) contains both the ADC1 and ADC2’s regular converted data. All 32 register bits are used.



--- Page 13 ---

**RM0090 Analog-to-digital converter (ADC)**

• DMA requests in Multi ADC mode:

In Multi ADC mode the DMA may be configured to transfer converted data in three different modes. In all cases, the DMA streams to use are those connected to the ADC:

– DMA mode 1: On each DMA request (one data item is available), a half-word representing an ADC-converted data item is transferred.

In Triple ADC mode, ADC1 data are transferred on the first request, ADC2 data are transferred on the second request and ADC3 data are transferred on the third request; the sequence is repeated. So the DMA first transfers ADC1 data followed by ADC2 data followed by ADC3 data and so on.

DMA mode 1 is used in regular simultaneous triple mode only.

**Example:**

Regular simultaneous triple mode: 3 consecutive DMA requests are generated (one for each converted data item)

1st request: ADC_CDR[31:0] = ADC1_DR[15:0]

2nd request: ADC_CDR[31:0] = ADC2_DR[15:0]

3rd request: ADC_CDR[31:0] = ADC3_DR[15:0]

4th request: ADC_CDR[31:0] = ADC1_DR[15:0]

– DMA mode 2 : On each DMA request (two data items are available) two half- words representing two ADC-converted data items are transferred as a word. In Dual ADC mode, both ADC2 and ADC1 data are transferred on the first request (ADC2 data take the upper half-word and ADC1 data take the lower half-word) and so on.

In Triple ADC mode, three DMA requests are generated. On the first request, both ADC2 and ADC1 data are transferred (ADC2 data take the upper half-word and ADC1 data take the lower half-word). On the second request, both ADC1 and ADC3 data are transferred (ADC1 data take the upper half-word and ADC3 data take the lower half-word).On the third request, both ADC3 and ADC2 data are transferred (ADC3 data take the upper half-word and ADC2 data take the lower half-word) and so on.

DMA mode 2 is used in interleaved mode and in regular simultaneous mode (for Dual ADC mode only).

**Example:**

a) Interleaved dual mode: a DMA request is generated each time 2 data items are available:

1st request: ADC_CDR[31:0] = ADC2_DR[15:0] | ADC1_DR[15:0]

2nd request: ADC_CDR[31:0] = ADC2_DR[15:0] | ADC1_DR[15:0]

b) Interleaved triple mode: a DMA request is generated each time 2 data items are available

1st request: ADC_CDR[31:0] = ADC2_DR[15:0] | ADC1_DR[15:0]

2nd request: ADC_CDR[31:0] = ADC1_DR[15:0] | ADC3_DR[15:0]

3rd request: ADC_CDR[31:0] = ADC3_DR[15:0] | ADC2_DR[15:0]

4th request: ADC_CDR[31:0] = ADC2_DR[15:0] | ADC1_DR[15:0]

– DMA mode 3 : This mode is similar to the DMA mode 2. The only differences are that the on each DMA request (two data items are available) two bytes



432

--- Page 14 ---

**Analog-to-digital converter (ADC) RM0090**

representing two ADC converted data items are transferred as a half-word. The data transfer order is similar to that of the DMA mode 2.

DMA mode 3 is used in interleaved mode in 6-bit and 8-bit resolutions (dual and triple mode).

**Example:**

a) Interleaved dual mode: a DMA request is generated each time 2 data items are

available

1st request: ADC_CDR[15:0] = ADC2_DR[7:0] | ADC1_DR[7:0] 2nd request: ADC_CDR[15:0] = ADC2_DR[7:0] | ADC1_DR[7:0]

b) Interleaved triple mode: a DMA request is generated each time 2 data items are

available

1st request: ADC_CDR[15:0] = ADC2_DR[7:0] | ADC1_DR[7:0] 2nd request: ADC_CDR[15:0] = ADC1_DR[7:0] | ADC3_DR[7:0] 3rd request: ADC_CDR[15:0] = ADC3_DR[7:0] | ADC2_DR[7:0] 4th request: ADC_CDR[15:0] = ADC2_DR[7:0] | ADC1_DR[7:0] Overrun detection: If an overrun is detected on one of the concerned ADCs (ADC1 and ADC2 in dual and triple modes, ADC3 in triple mode only), the DMA requests are no longer issued to ensure that all the data transferred to the RAM are valid. It may happen that the EOC bit corresponding to one ADC remains set because the data register of this ADC contains valid data.

**13.9.1 Injected simultaneous mode**

This mode converts an injected group of channels. The external trigger source comes from the injected group multiplexer of ADC1 (selected by the JEXTSEL[3:0] bits in the ADC1_CR2 register). A simultaneous trigger is provided to ADC2 and ADC3.

Note: Do not convert the same channel on the two/three ADCs (no overlapping sampling times for the two/three ADCs when converting the same channel).

In simultaneous mode, one must convert sequences with the same length or ensure that the interval between triggers is longer than the longer of the 2 sequences (Dual ADC mode) /3 sequences (Triple ADC mode). Otherwise, the ADC with the shortest sequence may restart while the ADC with the longest sequence is completing the previous conversions. Regular conversions can be performed on one or all ADCs. In that case, they are independent of each other and are interrupted when an injected event occurs. They are resumed at the end of the injected conversion group.



--- Page 15 ---

**RM0090**

**Analog-to-digital converter (ADC)**

**Dual ADC mode**

At the end of conversion event on ADC1 or ADC2: •

The converted data are stored into the ADC_JDRx registers of each ADC interface.

| • A JEOC i ADC1/AD Figur ADC1 ADC2 Trigg Triple ADC |  | • A JEOC i ADC1/AD Figur | nterrupt is generated (if C2’s injected channels e 52. Injected simultan |
|---|---|---|---|
| ADC1 ADC2 Trigg |  |  |  |
| CH0 CH1 CH | 2 CH3 ... CH15 13 CH12 ... CH0 End of conversion on ADC1 and ADC2 ai16054 - Приподнимите вилы на 5-10 см от штабеля, отодвиньте мачту назад до конца. - Проверьте все вокруг грузовика, что убедиться, что путь движения свобод и медленно отъезжайте назад |  |  |
| D | CH15 CH14 CH er Sampling Conversion |  |  |
| T |  |  |  |
| mode |  |  |  |


At the end of conversion event on ADC1, ADC2 or ADC3:

| • The converted data are stored i • A JEOC interrupt is generated (i ADC1/ADC2/ADC3’s injected c Figure 53. Injected simulta ADC1 CH0 CH1 CH |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|
| _ led on one of the three ADC interfaces) when th ls have all been converted. mode on 4 channels: triple ADC mode Предупреждение  Очень опасно укладывать порожние грузы на возвышенность или с нее. |  |  |  |  |  |  |  |  |
| ADC1 CH0 CH1 CH |  |  | CH3 ... CH15 CH12 ... CH0 CH5 ... CH2 End of conversion on ADC1, ADC2 and ADC3 ai16055 Если вы не будете соблюдать этап укладки, это может привести к смертельному исходу, опасному для оператора.  При большой загрузке в высоком положении запрещается наклонять мачту вперед или назад в рабочем направлении.  Запрещается использовать ручку |  |  |  |  |  |
|  |  | CH3 ... CH15 CH12 ... CH0 CH5 ... CH2 End of conversion on ADC1, ADC2 and ADC3 ai16055 укладки, это может привести к смертельному исходу, опасному оператора.  При большой загрузке в высоко положении запрещается наклон мачту вперед или назад в рабоч направлении. |  |  |  |  |  |  |
|  | CH0 |  | CH1 |  | CH | 2 |  | CH3 |
| ADC2 CH15 CH14 CH13 ADC3 CH10 CH12 CH8 Trigger Sampling Conversion |  | CH15 |  | CH14 |  | CH13 |  | CH12 |
|  | CH10 |  | CH12 |  | CH8 |  | CH5 |  |
| of channels. The external trigger source comes 1 (selected by the EXTSEL[3:0] bits in the er is provided to ADC2 and ADC3. o/three ADCs (no overlapping sampling times fo ame channel). onvert sequences with the same length or ensure than the long conversion time of the 2 sequence C mode). Otherwise, the ADC with the shortest he longest sequence is completing the previous бокового переключения передач без защиты каретки вил при погрузке в верхнем положении.  Запрещается поднимать мачту во время управления вилочным погрузчиком или перемещать мачту вперед/ назад при погрузке.  Рекомендуется, чтобы водитель не использовал объединенную операцию, то есть привод и направление работы при управлении многоступенчатой рукояткой, в противном случае это может привести к серьезным последствиям. |  |  |  |  |  |  |  |  |




432

--- Page 16 ---

**Analog-to-digital converter (ADC) RM0090**

**Dual ADC mode**

At the end of conversion event on ADC1 or ADC2:

• A 32-bit DMA transfer request is generated (if DMA[1:0] bits in the ADC_CCR register are equal to 0b10). This request transfers the ADC2 converted data stored in the upper half-word of the ADC_CDR 32-bit register to the SRAM and then the ADC1 converted data stored in the lower half-word of ADC_CDR to the SRAM.

• An EOC interrupt is generated (if enabled on one of the two ADC interfaces) when the ADC1/ADC2’s regular channels have all been converted.

**Figure 54. Regular simultaneous mode on 16 channels: dual ADC mode**

| кир | CH0 уйте | ко | CH1 леса. | CH2 | CH3 |  |
|---|---|---|---|---|---|---|
| у п | CH15 итани | я | CH14 | CH13 | CH12 |  |


ADC1 ... CH15

ADC2 ... CH0

Trigger End of conversion on ADC1 and ADC2

Sampling

Conversion

ai16054

**Triple ADC mode**

At the end of conversion event on ADC1, ADC2 or ADC3:

• Three 32-bit DMA transfer requests are generated (if DMA[1:0] bits in the ADC_CCR register are equal to 0b01). Three transfers then take place from the ADC_CDR 32-bit register to SRAM: first the ADC1 converted data, then the ADC2 converted data and finally the ADC3 converted data. The process is repeated for each new three conversions.

• An EOC interrupt is generated (if enabled on one of the three ADC interfaces) when the ADC1/ADC2/ADC3’s regular channels are have all been converted.

**Figure 55. Regular simultaneous mode on 16 channels: triple ADC mode**

| те | CH0 транс | по | CH1 ртно | е | CH2 |  | CH3 масло |
|---|---|---|---|---|---|---|---|
| м | CH15 или че | м | CH14 -то |  | CH13 | - | CH12 Очис |
|  | CH10 |  | CH12 |  | CH8 |  | CH5 посто |


ADC1 ... CH15

ADC2 ... CH0

ADC3 ... CH2

Trigger End of conversion on ADC1, ADC2  and ADC3

Sampling

Conversion

ai16055

406/1749 RM0090 Rev 18

--- Page 17 ---

**RM0090 Analog-to-digital converter (ADC)**

**13.9.3 Interleaved mode**

This mode can be started only on a regular group (usually one channel). The external trigger source comes from the regular channel multiplexer of ADC1.

**Dual ADC mode**

After an external trigger occurs:

• ADC1 starts immediately

• ADC2 starts after a delay of several ADC clock cycles

The minimum delay which separates 2 conversions in interleaved mode is configured in the DELAY bits in the ADC_CCR register. However, an ADC cannot start a conversion if the complementary ADC is still sampling its input (only one ADC can sample the input signal at a given time). In this case, the delay becomes the sampling time + 2 ADC clock cycles. For instance, if DELAY = 5 clock cycles and the sampling takes 15 clock cycles on both ADCs, then 17 clock cycles will separate conversions on ADC1 and ADC2).

If the CONT bit is set on both ADC1 and ADC2, the selected regular channels of both ADCs are continuously converted.

Note: If the conversion sequence is interrupted (for instance when DMA end of transfer occurs), the multi-ADC sequencer must be reset by configuring it in independent mode first (bits DUAL[4:0] = 00000) before reprogramming the interleaved mode.

After an EOC interrupt is generated by ADC2 (if enabled through the EOCIE bit) a 32-bit DMA transfer request is generated (if the DMA[1:0] bits in ADC_CCR are equal to 0b10). This request first transfers the ADC2 converted data stored in the upper half-word of the ADC_CDR 32-bit register into SRAM, then the ADC1 converted data stored in the register’s lower half-word into SRAM.

**Figure 56. Interleaved mode on 1 channel in continuous conversion mode: dual ADC**

**mode**

End of conversion on ADC1 Sampling ADC1 ... Conversion ADC2 CH0 ...







Trigger

End of conversion on ADC2

8 ADCCLK

cycles ai16056

**Triple ADC mode**

After an external trigger occurs:

• ADC1 starts immediately and

• ADC2 starts after a delay of several ADC clock cycles

• ADC3 starts after a delay of several ADC clock cycles referred to the ADC2 conversion The minimum delay which separates 2 conversions in interleaved mode is configured in the DELAY bits in the ADC_CCR register. However, an ADC cannot start a conversion if the complementary ADC is still sampling its input (only one ADC can sample the input signal at



432

--- Page 18 ---

**Analog-to-digital converter (ADC) RM0090**

a given time). In this case, the delay becomes the sampling time + 2 ADC clock cycles. For instance, if DELAY = 5 clock cycles and the sampling takes 15 clock cycles on the three ADCs, then 17 clock cycles will separate the conversions on ADC1, ADC2 and ADC3). If the CONT bit is set on ADC1, ADC2 and ADC3, the selected regular channels of all ADCs are continuously converted.

Note: If the conversion sequence is interrupted (for instance when DMA end of transfer occurs), the multi-ADC sequencer must be reset by configuring it in independent mode first (bits DUAL[4:0] = 00000) before reprogramming the interleaved mode.

In this mode a DMA request is generated each time 2 data items are available, (if the DMA[1:0] bits in the ADC_CCR register are equal to 0b10). The request first transfers the first converted data stored in the lower half-word of the ADC_CDR 32-bit register to SRAM, then it transfers the second converted data stored in ADC_CDR’s upper half-word to SRAM. The sequence is the following:

• 1st request: ADC_CDR[31:0] = ADC2_DR[15:0] | ADC1_DR[15:0]

• 2nd request: ADC_CDR[31:0] = ADC1_DR[15:0] | ADC3_DR[15:0]

• 3rd request: ADC_CDR[31:0] = ADC3_DR[15:0] | ADC2_DR[15:0]

• 4th request: ADC_CDR[31:0] = ADC2_DR[15:0] | ADC1_DR[15:0], ...

**Figure 57. Interleaved mode on 1 channel in continuous conversion mode: triple ADC**

**mode**









**13.9.4 Alternate trigger mode**

This mode can be started only on an injected group. The source of external trigger comes from the injected group multiplexer of ADC1.

Note: Regular conversions can be enabled on one or all ADCs. In this case the regular conversions are independent of each other. A regular conversion is interrupted when the



--- Page 19 ---

**RM0090 Analog-to-digital converter (ADC)**

| вых 5 зарядок в с руководством по | плотность не перестанут раст появятся пузырьки, как тольк |
|---|---|
| ному зарядному | возобновится после 1-часово Дополнительный заряд |
| ющий заряд | – Если одной зарядки недост работы в течение целого дня, |
| напряжение, плотность и еи, как правило, становятся | дополнительную зарядку во в – При низкой температуре ок |
| анными. | среды может быть произведе дополнительный заряд. |
| и удельный вес некоторых личиваются медленнее по | Заряд при хранении |
| ругими элементами при ьшаются более быстрыми | – Перед хранением аккумуля ему выравнивающий заряд. |
| азрядке. | – Наносите выравнивающий 15-30 дней во время хранени |


finished.

•

•

•

have been converted.

| з | следующих ситуаций: a. Напряжение разряда часто падает ниже напряжения отключения; b. Ток разряда часто слишком высок; c. Аккумулятор не заряжается своевременно после завершения разряда; d. Электролит смешан с незначительными примесями; e. Аккумулятор часто недостаточно |  | руко Зам 1. Пр | водством пользователя аккумулятор ена батареи ипаркуйте погрузчик на ровной | а. |
|---|---|---|---|---|---|
| яда часто слишком высок; ятор не заряжается нно после завершения разряда; лит смешан с незначительными ; ятор часто недостаточно | площ Извл акку 2. Вы | адке и включите ручной тормоз. еките ключ и выньте вилку мулятора. верните ручку 2 и снимите панель 1 3 | 1. |  |  |
| ми |  |  |  |  |  |
|  | я или долгое время не |  |  |  |  |
| аряжаетс |  |  |  |  |  |
| ался; |  |  |  |  |  |


If the injected discontinuous mode is enabled for both ADC1 and ADC2:

• When the 1st trigger occurs, the first injected ADC1 channel is converted.

• When the 2nd trigger occurs, the first injected ADC2 channel are converted

• and so on

A JEOC interrupt, if enabled, is generated after all injected ADC1 channels in the group have been converted.

A JEOC interrupt, if enabled, is generated after all injected ADC2 channels in the group have been converted.

If another external trigger occurs after all injected channels in the group have been converted then the alternate trigger process restarts.



432

--- Page 20 ---

**Analog-to-digital converter (ADC) RM0090**

**Figure 59. Alternate trigger: 4 injected channels (each ADC) in discontinuous mode**



**Triple ADC mode**

• When the 1st trigger occurs, all injected ADC1 channels in the group are converted.

• When the 2nd trigger occurs, all injected ADC2 channels in the group are converted.

• When the 3rd trigger occurs, all injected ADC3 channels in the group are converted.

• and so on

A JEOC interrupt, if enabled, is generated after all injected ADC1 channels in the group have been converted.

A JEOC interrupt, if enabled, is generated after all injected ADC2 channels in the group have been converted.

A JEOC interrupt, if enabled, is generated after all injected ADC3 channels in the group have been converted.

If another external trigger occurs after all injected channels in the group have been converted then the alternate trigger process restarts by converting the injected ADC1 channels in the group.

**Figure 60. Alternate trigger: injected group of each ADC**



**13.9.5 Combined regular/injected simultaneous mode**

It is possible to interrupt the simultaneous conversion of a regular group to start the simultaneous conversion of an injected group.

Note: In combined regular/injected simultaneous mode, one must convert sequences with the same length or ensure that the interval between triggers is longer than the long conversion time of the 2 sequences (Dual ADC mode) /3 sequences (Triple ADC mode). Otherwise, the



--- Page 21 ---

**RM0090 Analog-to-digital converter (ADC)**

ADC with the shortest sequence may restart while the ADC with the longest sequence is completing the previous conversions.

**13.9.6 Combined regular simultaneous + alternate trigger mode**

It is possible to interrupt the simultaneous conversion of a regular group to start the alternate trigger conversion of an injected group. Figure 61 shows the behavior of an alternate trigger interrupting a simultaneous regular conversion.

The injected alternate conversion is immediately started after the injected event. If regular conversion is already running, in order to ensure synchronization after the injected conversion, the regular conversion of all (master/slave) ADCs is stopped and resumed synchronously at the end of the injected conversion.

Note: In combined regular simultaneous + alternate trigger mode, one must convert sequences with the same length or ensure that the interval between triggers is longer than the long conversion time of the 2 sequences (Dual ADC mode) /3 sequences (Triple ADC mode). Otherwise, the ADC with the shortest sequence may restart while the ADC with the longest sequence is completing the previous conversions.

If the conversion sequence is interrupted (for instance when DMA end of transfer occurs), the multi-ADC sequencer must be reset by configuring it in independent mode first (bits DUAL[4:0] = 00000) before reprogramming the interleaved mode.

**Figure 61. Alternate + regular simultaneous**

1st trigger

| CH0 | CH1 |  | CH2 | CH3 |  |  |
|---|---|---|---|---|---|---|
|  |  | CH |  | CH | 0 |  |
| CH3 | CH5 |  | CH6 | CH7 |  |  |


| CH3 | CH4 |  |
|---|---|---|
| CH7 | CH8 |  |


ADC1 reg

ADC1 inj

ADC2 reg



ADC2 inj

synchro not lost

2nd trigger ai16062

If a trigger occurs during an injected conversion that has interrupted a regular conversion, it is ignored. Figure 62 shows the behavior in this case (2nd trigger is ignored).



432

--- Page 22 ---

**Analog-to-digital converter (ADC) RM0090**

**Figure 62. Case of trigger occurring during injected conversion**



| CH0 | CH1 |  | CH2 | CH3 |  |  |
|---|---|---|---|---|---|---|
|  |  | CH |  | CH | 0 |  |
| CH3 | CH5 |  | CH6 | CH7 |  |  |


| CH3 | CH4 |  |
|---|---|---|
|  |  | CH0 |
| CH7 | CH8 |  |




