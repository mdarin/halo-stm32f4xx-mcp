> [RM0090 ch.13 §13.13] ADC registers**

### 13.13.2 ADC control register 1 (ADC_CR1)**

**13.13.2 ADC control register 1 (ADC_CR1)**

Address offset: 0x04

Reset value: 0x0000 0000

31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16

| Отрегулируйте блокировку рулевого Рулевое |  |  | 〇 〇 〇 |  |
|---|---|---|---|---|
| 〇 |  |  |  |  |


15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

| состояние управлен | рулево ия | го | 〇 | 〇 〇 |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Шесте | См р | азка з | убчаты | х |  |  | 〇 |  | 〇 |  | 〇 |  |


Bits 31:27 Reserved, must be kept at reset value.

Bit 26 OVRIE: Overrun interrupt enable

This bit is set and cleared by software to enable/disable the Overrun interrupt.

0: Overrun interrupt disabled

1: Overrun interrupt enabled. An interrupt is generated when the OVR bit is set. Bits 25:24 RES[1:0]: Resolution

These bits are written by software to select the resolution of the conversion.

00: 12-bit (15 ADCCLK cycles)

01: 10-bit (13 ADCCLK cycles)

10: 8-bit (11 ADCCLK cycles)

11: 6-bit (9 ADCCLK cycles)

Bit 23 AWDEN: Analog watchdog enable on regular channels

This bit is set and cleared by software.

0: Analog watchdog disabled on regular channels

1: Analog watchdog enabled on regular channels

Bit 22 JAWDEN: Analog watchdog enable on injected channels

This bit is set and cleared by software.

0: Analog watchdog disabled on injected channels

1: Analog watchdog enabled on injected channels

Bits 21:16 Reserved, must be kept at reset value.

Bits 15:13 DISCNUM[2:0]: Discontinuous mode channel count

These bits are written by software to define the number of regular channels to be converted in discontinuous mode, after receiving an external trigger.

000: 1 channel

001: 2 channels

...

111: 8 channels

Bit 12 JDISCEN: Discontinuous mode on injected channels

This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group.

0: Discontinuous mode on injected channels disabled

1: Discontinuous mode on injected channels enabled



--- Page 27 ---

**RM0090**

**Analog-to-digital converter (ADC)**



Bit 11 DISCEN: Discontinuous mode on regular channels

This bit is set and cleared by software to enable/disable Discontinuous mode on regular

channels.

|  | внешней мачте и поперечинах Износ, растрескивание и повреждение сварных швов на кронштейнах |  | 〇 |  |  |
|---|---|---|---|---|---|
|  | наклонного цилиндра и мачте Износ, растрескивание |  |  |  |  |
| или повреждение сварки на внутренней и внешней мачтах |  | 〇 |  |  |  |
| Mast and fo | Износ, растрескивани и повреждение сварк на каретке вил | е и | 〇 |  |  |
| Незакрепленные роликовые подшипник | и | 〇 |  |  |  |


1: Analog watchdog enabled on a single channel

| в о П б | тулок подшипников поры мачты роверьте нижние олты подъемного |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|
| ц г U н г п П р с т | илиндра, болты оловки штока поршня, Т -образные болты и | естовый молоток |  |  | 〇 |  |  |
| аправляющие болты оловки поршня на редмет ослабления роверьте ролики, |  |  |  |  |  |  |  |
| оликовые штифты и варные детали на наличие рещин или повреждений |  |  |  | 〇 |  |  |  |
| Другие Пункт |  |  |  |  |  | Каждый | Каждые |
| Требуемое И обслуживание | нстру- Е мент | жедневно (8 часов) | Еженедельно (40 часов) | Ежемесячно (166 часов) | квартал (500 часов) | полгода (1000часов) |  |
| Верхнее огражден и грузовая спинка Сигнал поворот | Проверьте Те надежность мо ие крепления | стовый лоток | 〇 |  |  |  |  |
| Проверьте на наличие деформации, трещин, повреждений |  | 〇 |  |  |  |  |  |
| Эксплуатация и а монтаж |  | 〇 |  |  |  |  |  |
| Звуково сигнал Свет и фары | й Эксплуатация и монтаж |  | 〇 |  |  |  |  |
| Эксплуатация и монтаж |  | 〇 |  |  |  |  |  |
| Инструме | нт Эксплуатация и монтаж |  | 〇 |  |  |  |  |
| провод | Повреждение или ослабление провода |  |  | 〇 |  |  |  |
| Неплотность соединения электрической |  |  |  | 〇 |  |  |  |
|  | цепи |  |  |  |  |  |  |


RM0090 Rev 18 417/1749

432

--- Page 28 ---

**Analog-to-digital converter (ADC)**

**RM0090**

