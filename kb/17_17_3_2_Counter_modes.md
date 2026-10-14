**17.3.2 Counter modes**

| Upc | ounting mode |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| In up TIMx If the | counting mode, the counter counts from 0 to the au | кольцо поршня или наклонный цилиндр. to-reload value (content of the |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Кончик пальца не устанавливается автоматически на место или работает со сбоями ARR register), then restarts from 0 and generates _ repetition counter is used, the update event (UEV) | Отрегулируйте или замените a counter overflow event. is generated after upcounting is |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| наклоняется.repea Мачта вытягивается, приподнимается и вяло (TIMx Settin contr | Заклинивание поршня или изгибы штока поршня ted for the number of times programmed in the rep RCR+1). Else the update event is generated at e _ | Замените поврежденные детали. etition counter register plus one ach counter overflow. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| В цилиндре скопилось слишком много грязи. g the UG bit in the TIMx EGR register (by software _ oller) also generates an update event. | Очистить or by using the slave mode |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| перемещаетсяThe Каретка вил плавно поднимается или опускается, устройство плавно regist prelo Howe presc TIMx settin both Whe TIMx | Неправильная регулировка между роликами и швеллерной сталью UEV event can be disabled by software by setting th | Отрегулируйте зазор между роликом и швеллерной сталью e UDIS bit in the TIMx CR1 _ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Грязь в швеллере мачты или повреждение стали er. This is to avoid updating the shadow registers w ad registers. Then no update event occurs until the ver, the counter restarts from 0, as well as the coun | Удалите грязь или отремонтируйте hile writing new values in the UDIS bit has been written to 0. ter of the prescaler (but the |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Недостаточная смазка стали канала мачты ale rate does not change). In addition, if the URS b CR1 register is set, setting the UG bit generates a | Нанесите смазку на контактные поверхности деталей скольжения it (update request selection) in n update event UEV but without |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Прогиб мачты или погнутая каретка вил _ g the UIF flag (thus no interrupt or DMA request is update and capture interrupts when clearing the co | Отремонтировать или заменить sent). This is to avoid generating unter on the capture event. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Отклонение канала или изгиб досягаемого устройства n an update event occurs, all the registers are upda SR register) is set (depending on the URS bit): | Отремонтировать или заменить ted and the update flag (UIF bit in |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Каретка вил поднимается неравномерно• | Неправильная регулировка подъемных цепей _ The repetition counter is reloaded with the content o | Отрегулируйте натяжение подъемных цепей с обеих сторон на одинаковую натянутость f TIMx RCR register, _ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Подъемный ролик не может вращаться • • r The f frequ | Деформация или повреждение роликов The auto-reload shadow register is updated with the The buffer of the prescaler is reloaded with the prelo | Заменить preload value (TIMx ARR), _ ad value (content of the TIMx PSC _ |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Неправильная регулировка подъемного ролика. egister). ollowing figures show some examples of the counte | Регулировать. r behavior for different clock |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Ролики застревают из-за грязи encies when TIMx ARR=0x36. _ | Очистите и смажьте ролики. |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Слишком много шума от мачты | Недостаточная смазка. Figure 89. Counter timing diagram, intern | Смазать. al clock divided by 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| го ы |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Боковые ролики на к неправильно. Он ударяе когда удлиненная мачта оп CK PSC _ |  | к яе | арет тся уска | ет я | ке вилки отрегулиров о сталь швеллера ма ется. | в ст |  | ки ь |  | от вел |  | гул ер |  | ов ма | аны чты, | ы ы, | Отрегулируйте ролики и регулировочные прокладки боковой прокатки | ре |  | ли |  | те |  | ол |  | и |  |  |  |  |  |  |  |




--- Page 131 ---

**RM0090**

**Advanced-control timers (TIM1 and TIM8)**

**Figure 90. Counter timing diagram, internal clock divided by 2**

CK_PSC

CNT_EN

Timerclock = CK_CNT



Counter register 0034

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

MS31079V3

**Figure 91. Counter timing diagram, internal clock divided by 4**

CK_PSC

CNT_EN

Timerclock = CK_CNT



Counter register

0035 0001

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

MS31080V3

**Figure 92. Counter timing diagram, internal clock divided by N**

CK_PSC

Timerclock = CK_CNT

Counter register 1F

20 00

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

MS31081V3



588

--- Page 132 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

**Figure 93. Counter timing diagram, update event when ARPE=0 (TIMx_ARR not preloaded)**

CK_PSC



CEN

Timerclock = CK_CNT

Counter register 31 32

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

Auto-reload preload register FF

36

Write a new value in TIMx_ARR

MS31082V3

**Figure 94. Counter timing diagram, update event when ARPE=1 (TIMx_ARR preloaded)**

CK_PSC



CEN

Timerclock = CK_CNT



Counter register F0

03 04

Counter overflow

Update event (UEV)

Update interrupt flag

(UIF)

Auto-reload preload F5

36

register

Auto-reload shadow

register F5

36

Write a new value in TIMx_ARR

MS31083V2



--- Page 133 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**Downcounting mode**

In downcounting mode, the counter counts from the auto-reload value (content of the TIMx_ARR register) down to 0, then restarts from the auto-reload value and generates a counter underflow event.

If the repetition counter is used, the update event (UEV) is generated after downcounting is repeated for the number of times programmed in the repetition counter register plus one (TIMx_RCR+1). Else the update event is generated at each counter underflow.

Setting the UG bit in the TIMx_EGR register (by software or by using the slave mode controller) also generates an update event.

The UEV update event can be disabled by software by setting the UDIS bit in TIMx_CR1 register. This is to avoid updating the shadow registers while writing new values in the preload registers. Then no update event occurs until UDIS bit has been written to 0.

However, the counter restarts from the current auto-reload value, whereas the counter of the prescaler restarts from 0 (but the prescale rate doesn’t change).

In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the UG bit generates an update event UEV but without setting the UIF flag (thus no interrupt or DMA request is sent). This is to avoid generating both update and capture interrupts when clearing the counter on the capture event.

When an update event occurs, all the registers are updated and the update flag (UIF bit in TIMx_SR register) is set (depending on the URS bit):

• The repetition counter is reloaded with the content of TIMx_RCR register

• The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC

register)

• The auto-reload active register is updated with the preload value (content of the

TIMx_ARR register). Note that the auto-reload is updated before the counter is reloaded, so that the next period is the expected one

The following figures show some examples of the counter behavior for different clock frequencies when TIMx_ARR=0x36.



588

--- Page 134 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

**Figure 95. Counter timing diagram, internal clock divided by 1**

CK_PSC



CNT_EN

Timerclock = CK_CNT

Counter register 05

Counter underflow (cnt_udf)

Update event (UEV)

Update interrupt flag

(UIF)

MS31184V1

**Figure 96. Counter timing diagram, internal clock divided by 2**

CK_PSC

CNT_EN

Timerclock = CK_CNT



0001

Counter register 0002

Counter underflow

Update event (UEV)

Update interrupt flag

(UIF)

MS31185V1



--- Page 135 ---

**RM0090**

**Advanced-control timers (TIM1 and TIM8)**

**Figure 97. Counter timing diagram, internal clock divided by 4**

CK_PSC

CNT_EN

Timerclock = CK_CNT



Counter register

0001 0035

Counter underflow

Update event (UEV)

Update interrupt flag (UIF)

MS40510V1

**Figure 98. Counter timing diagram, internal clock divided by N**

CK_PSC

Timerclock = CK_CNT

Counter register 20 1F

00 36

Counter underflow

Update event (UEV)

Update interrupt flag

(UIF)

MS31187V1



588

--- Page 136 ---

**Advanced-control timers (TIM1 and TIM8) RM0090**

| Внутренняя конструкция переднего цилиндра 7 8 14 15 4 5 6 9 10 13 11 12 13 16 1 2 3 2 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 4 5 6 9 10 13 11 12 13 16 2 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 1. Поршневой шт 7.Уплотнительно кольцо |  |  |  |  |  |  |  |  |  |  |  | 5.Пылезащитное 6.Уплотнительное кольцо A1 кольцо B3 11.Уплотнительное 12.KR-кольцо кольцо |  |  |  |  |  |  |  |  |  |  |
| ок 2. Стопорное кольцо 3.Подшипник 4.Направля для отверстия ющая втулка | ипн | ик | 4.Нап | рав | ля | 5.Пы | ле | з ащитное | 6.Уп | лотн | ител | ьно |  |  |  |  |  |  |  |  |  |  |
| кольцо A1 | кольцо B | 3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| е 8.FR направляющее 9.Корпус 10.Поршень кольцо цилиндра |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 14.Омыватель 15.Гайка 16.Pin-код |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| ляющее кольцо |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| Внутренняя к | онструкция н | аклонного ц | илиндра |  |  | 8 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| 2 3 4 5 6 9 1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |


**Center-aligned mode (up/down counting)**

In center-aligned mode, the counter counts from 0 to the auto-reload value (content of the TIMx_ARR register) – 1, generates a counter overflow event, then counts from the auto- reload value down to 1 and generates a counter underflow event. Then it restarts counting from 0.

Center-aligned mode is active when the CMS bits in TIMx_CR1 register are not equal to '00'. The Output compare interrupt flag of channels configured in output is set when: the counter counts down (Center aligned mode 1, CMS = "01"), the counter counts up (Center aligned mode 2, CMS = "10") the counter counts up and down (Center aligned mode 3, CMS = "11").

In this mode, the DIR direction bit in the TIMx_CR1 register cannot be written. It is updated by hardware and gives the current direction of the counter.

The update event can be generated at each counter overflow and at each counter underflow or by setting the UG bit in the TIMx_EGR register (by software or by using the slave mode controller) also generates an update event. In this case, the counter restarts counting from 0, as well as the counter of the prescaler.

The UEV update event can be disabled by software by setting the UDIS bit in the TIMx_CR1 register. This is to avoid updating the shadow registers while writing new values in the preload registers. Then no update event occurs until UDIS bit has been written to 0. However, the counter continues counting up and down, based on the current auto-reload value.

In addition, if the URS bit (update request selection) in TIMx_CR1 register is set, setting the UG bit generates an UEV update event but without setting the UIF flag (thus no interrupt or DMA request is sent). This is to avoid generating both update and capture interrupts when clearing the counter on the capture event.



--- Page 137 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

When an update event occurs, all the registers are updated and the update flag (UIF bit in TIMx_SR register) is set (depending on the URS bit):

• The repetition counter is reloaded with the content of TIMx_RCR register

• The buffer of the prescaler is reloaded with the preload value (content of the TIMx_PSC register)

• The auto-reload active register is updated with the preload value (content of the TIMx_ARR register). Note that if the update source is a counter overflow, the auto- reload is updated before the counter is reloaded, so that the next period is the expected one (the counter is loaded with the new value).

The following figures show some examples of the counter behavior for different clock frequencies.

**Figure 100. Counter timing diagram, internal clock divided by 1, TIMx_ARR = 0x6**

CK_PSC



CEN

Timerclock = CK_CNT

Counter register 04

Counter underflow

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

MS31189V2

1. Here, center-aligned mode 1 is used (for more details refer to Section 17.4: TIM1 and TIM8 registers ).

**Figure 101. Counter timing diagram, internal clock divided by 2**

CK_PSC

CNT_EN

Timerclock = CK_CNT



Counter register 0003

Counter underflow

Update event (UEV)

Update interrupt flag (UIF)

MS31190V2



588

--- Page 138 ---

**Advanced-control timers (TIM1 and TIM8)**

**RM0090**

**Figure 102. Counter timing diagram, internal clock divided by 4, TIMx_ARR=0x36**

CK_PSC

CNT_EN

Timerclock = CK_CNT



Counter register 0034

0035

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

MS31191V2

1. Center-aligned mode 2 or 3 is used with an UIF on overflow.

**Figure 103. Counter timing diagram, internal clock divided by N**

CK_PSC

Timerclock = CK_CNT

Counter register 20

1F 01 00

Counter underflow

Update event (UEV)

Update interrupt flag (UIF)

MS31192V2



--- Page 139 ---

**RM0090 Advanced-control timers (TIM1 and TIM8)**

**Figure 104. Counter timing diagram, update event with ARPE=1 (counter underflow)**

CK_PSC



CEN

Timer clock = CK_CNT

Counter register 06

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

Auto-reload preload register FD 36

Write a new value in TIMx_ARR

Auto-reload active register FD 36

MS31193V3

**Figure 105. Counter timing diagram, Update event with ARPE=1 (counter overflow)**

CK_PSC



CEN

Timer clock = CK_CNT

Counter register F7

Counter overflow

Update event (UEV)

Update interrupt flag (UIF)

Auto-reload preload register FD 36

Write a new value in TIMx_ARR

Auto-reload active register FD 36

MS31194V2

