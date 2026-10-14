### 33.5.9 Precision time protocol (IEEE1588 PTP)

The IEEE 1588 standard defines a protocol that allows precise clock synchronization in measurement and control systems implemented with technologies such as network communication, local computing and distributed objects. The protocol applies to systems that communicate by local area networks supporting multicast messaging, including (but not limited to) Ethernet. This protocol is used to synchronize heterogeneous systems that include clocks of varying inherent precision, resolution and stability. The protocol supports system-wide synchronization accuracy in the submicrosecond range with minimum network and local clock computing resources. The message-based protocol, known as the precision time protocol (PTP), is transported over UDP/IP. The system or network is classified into Master and Slave nodes for distributing the timing/clock information. The protocol’s technique for synchronizing a slave node to a master node by exchanging PTP messages is described in Figure 372.

ST
RM0090 Rev 18
1157/1749

--- Page 183 ---

**Ethernet (ETH): media access control (MAC) with DMA controller** RM0090

**Figure 372. Networked time synchronization**

Master clock time Slave clock time

t1 Sync message
t2m Follow_up message containing value of t1
t3m Delay_Req message
t4 Delay_Resp message containing value of t4
time

Data at slave clock
t2 : t2
t1, t2
t3 : t1, t2, t3
t1, t2, t3, t4

ai15669

1. The master broadcasts PTP Sync messages to all its nodes. The Sync message contains the master's reference time information. The time at which this message leaves the master's system is t1. For Ethernet ports, this time has to be captured at the MII.
2. A slave receives the Sync message and also captures the exact time, t2, using its timing reference.
3. The master then sends the slave a Follow_up message, which contains the t1 information for later use.
4. The slave sends the master a Delay_Req message, noting the exact time, t3, at which this frame leaves the MII.
5. The master receives this message and captures the exact time, t4, at which it enters its system.
6. The master sends the t4 information to the slave in the Delay_Resp message.
7. The slave uses the four values of t1, t2, t3, and t4 to synchronize its local timing reference to the master's timing reference.

Most of the protocol implementation occurs in the software, above the UDP layer. As described above, however, hardware support is required to capture the exact time when specific PTP packets enter or leave the Ethernet port at the MII. This timing information has to be captured and returned to the software for a proper, high-accuracy implementation of PTP.

**Reference timing source**

To get a snapshot of the time, the core requires a reference time in 64-bit format (split into two 32-bit channels, with the upper 32 bits providing time in seconds, and the lower 32 bits indicating time in nanoseconds) as defined in the IEEE 1588 specification.

The PTP reference clock input is used to internally generate the reference time (also called the System Time) and to capture time stamps. The frequency of this reference clock must

1158/1749 RM0090 Rev 18 ST

--- Page 184 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

be greater than or equal to the resolution of time stamp counter. The synchronization accuracy target between the master node and the slaves is around 100 ns.

The generation, update and modification of the System Time are described in *System Time correction methods*.

The accuracy depends on the PTP reference clock input period, the characteristics of the oscillator (drift) and the frequency of the synchronization procedure.

Due to the synchronization from the Tx and Rx clock input domain to the PTP reference clock domain, the uncertainty on the time stamp latched value is 1 reference clock period. If we add the uncertainty due to resolution, we will add half the period for time stamping.

**Transmission of frames with the PTP feature**

When a frame's SFD is output on the MII, a time stamp is captured. Frames for which time stamp capture is required are controllable on a per-frame basis. In other words, each transmitted frame can be marked to indicate whether a time stamp must be captured or not for that frame. The transmitted frames are not processed to identify PTP frames. Frame control is exercised through the control bits in the transmit descriptor. Captured time stamps are returned to the application in the same way as the status is provided for frames. The time stamp is sent back along with the Transmit status of the frame, inside the corresponding transmit descriptor, thus connecting the time stamp automatically to the specific PTP frame. The 64-bit time stamp information is written back to the TDES2 and TDES3 fields, with TDES2 holding the time stamp's 32 least significant bits.

**Reception of frames with the PTP feature**

When the IEEE 1588 time stamping feature is enabled, the Ethernet MAC captures the time stamp of all frames received on the MII. The MAC provides the time stamp as soon as the frame reception is complete. Captured time stamps are returned to the application in the same way as the frame status is provided. The time stamp is sent back along with the Receive status of the frame, inside the corresponding receive descriptor. The 64-bit time stamp information is written back to the RDES2 and RDES3 fields, with RDES2 holding the time stamp's 32 least significant bits.

**System Time correction methods**

The 64-bit PTP time is updated using the PTP input reference clock, HCLK. This PTP time is used as a source to take snapshots (time stamps) of the Ethernet frames being transmitted or received at the MII. The System Time counter can be initialized or corrected using either the Coarse or the Fine correction method.

In the Coarse correction method, the initial value or the offset value is written to the Time stamp update register (refer to Section 33.8.3). For initialization, the System Time counter is written with the value in the Time stamp update registers, whereas for system time correction, the offset value (Time stamp update register) is added to or subtracted from the system time.

In the Fine correction method, the slave clock (reference clock) frequency drift with respect to the master clock (as defined in IEEE 1588) is corrected over a period of time, unlike in the Coarse correction method where it is corrected in a single clock cycle. The longer correction time helps maintain linear time and does not introduce drastic changes (or a large jitter) in the reference time between PTP Sync message intervals. In this method, an accumulator sums up the contents of the Addend register as shown in Figure 373. The arithmetic carry that the accumulator generates is used as a pulse to increment the system time counter.

ST
RM0090 Rev 18
1159/1749

--- Page 185 ---

**Ethernet (ETH): media access control (MAC) with DMA controller**
RM0090

The accumulator and the addend are 32-bit registers. Here, the accumulator acts as a high-precision frequency multiplier or divider. *Figure 373* shows this algorithm.

**Figure 373. System time update using the Fine correction method**

ai15670

The system time update logic requires a 50 MHz clock frequency to achieve 20 ns accuracy. The frequency division is the ratio of the reference clock frequency to the required clock frequency. Hence, if the reference clock (HCLK) is, let us say, 66 MHz, the ratio is calculated as 66 MHz/50 MHz = 1.32. Hence, the default addend value to be set in the register is 2^32/1.32, which is equal to 0xC1F0 7C1F.

If the reference clock drifts lower, to 65 MHz for example, the ratio is 65/50 or 1.3 and the value to set in the addend register is 2^32/1.30 equal to 0xC4EC 4EC4. If the clock drifts higher, to 67 MHz for example, the addend register must be set to 0xBF0 B7672. When the clock drift is zero, the default addend value of 0xC1F0 7C1F (2^32/1.32) should be programmed.

In *Figure 373*, the constant value used to increment the subsecond register is 0d43. This makes an accuracy of 20 ns in the system time (in other words, it is incremented by 20 ns steps).

The software has to calculate the drift in frequency based on the Sync messages, and to update the Addend register accordingly. Initially, the slave clock is set with FreqCompensationValue0 in the Addend register. This value is as follows:

FreqCompensationValue0 = 2^32 / FreqDivisionRatio

If MasterToSlaveDelay is initially assumed to be the same for consecutive Sync messages, the algorithm described below must be applied. After a few Sync cycles, frequency lock occurs. The slave clock can then determine a precise MasterToSlaveDelay value and re-synchronize with the master using the new value.

1160/1749
RM0090 Rev 18
ST

--- Page 186 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

The algorithm is as follows:

*   At time MasterSyncTime (n) the master sends the slave clock a Sync message. The slave receives this message when its local clock is SlaveClockTime (n) and computes MasterClockTime (n) as:
    MasterClockTime (n) = MasterSyncTime (n) + MasterToSlaveDelay (n)
*   The master clock count for current Sync cycle, MasterClockCount (n) is given by:
    MasterClockCount (n) = MasterClockTime (n) – MasterClockTime (n – 1) (assuming that MasterToSlaveDelay is the same for Sync cycles n and n – 1)
*   The slave clock count for current Sync cycle, SlaveClockCount (n) is given by:
    SlaveClockCount (n) = SlaveClockTime (n) – SlaveClockTime (n – 1)
*   The difference between master and slave clock counts for current Sync cycle, ClockDiffCount (n) is given by:
    ClockDiffCount (n) = MasterClockCount (n) – SlaveClockCount (n)
*   The frequency-scaling factor for slave clock, FreqScaleFactor (n) is given by:
    FreqScaleFactor (n) = (MasterClockCount (n) + ClockDiffCount (n)) / SlaveClockCount (n)
*   The frequency compensation value for Addend register, FreqCompensationValue (n) is given by:
    FreqCompensationValue (n) = FreqScaleFactor (n) × FreqCompensationValue (n – 1)

In theory, this algorithm achieves lock in one Sync cycle; however, it may take several cycles, due to changing network propagation delays and operating conditions.

This algorithm is self-correcting: if for any reason the slave clock is initially set to a value from the master that is incorrect, the algorithm corrects it at the cost of more Sync cycles.

**Programming steps for system time generation initialization**

The time stamping feature can be enabled by setting bit 0 in the Time stamp control register (ETH__PTPTSCR). However, it is essential to initialize the time stamp counter after this bit is set to start time stamp operation. The proper sequence is the following:

1.  Mask the Time stamp trigger interrupt by setting bit 9 in the MACIMR register.
2.  Program Time stamp register bit 0 to enable time stamping.
3.  Program the Subsecond increment register based on the PTP clock frequency.
4.  If you are using the Fine correction method, program the Time stamp addend register and set Time stamp control register bit 5 (addend register update).
5.  Poll the Time stamp control register until bit 5 is cleared.
6.  To select the Fine correction method (if required), program Time stamp control register bit 1.
7.  Program the Time stamp high update and Time stamp low update registers with the appropriate time value.
8.  Set Time stamp control register bit 2 (Time stamp init).
9.  The Time stamp counter starts operation as soon as it is initialized with the value written in the Time stamp update register.
10. Enable the MAC receiver and transmitter for proper time stamping.

*Note:* If time stamp operation is disabled by clearing bit 0 in the ETH__PTPTSCR register, the above steps must be repeated to restart the time stamp operation.

ST
RM0090 Rev 18
1161/1749

--- Page 187 ---

Ethernet (ETH): media access control (MAC) with DMA controller RM0090

**Programming steps for system time update in the Coarse correction method**

To synchronize or update the system time in one process (coarse correction method), perform the following steps:

1. Write the offset (positive or negative) in the Time stamp update high and low registers.
2. Set bit 3 (TSSTU) in the Time stamp control register.
3. The value in the Time stamp update registers is added to or subtracted from the system time when the TSSTU bit is cleared.

**Programming steps for system time update in the Fine correction method**

To synchronize or update the system time to reduce system-time jitter (fine correction method), perform the following steps:

1. With the help of the algorithm explained in *System Time correction methods*, calculate the rate by which you want to speed up or slow down the system time increments.
2. Update the time stamp.
3. Wait the time you want the new value of the Addend register to be active. You can do this by activating the Time stamp trigger interrupt after the system time reaches the target value.
4. Program the required target time in the Target time high and low registers. Unmask the Time stamp interrupt by clearing bit 9 in the ETH_MACIMR register.
5. Set Time stamp control register bit 4 (TSARU).
6. When this trigger causes an interrupt, read the ETH_MACSR register.
7. Reprogram the Time stamp addend register with the old value and set ETH_TPTSCR bit 5 again.

**PTP trigger internal connection with TIM2**

The MAC provides a trigger interrupt when the system time becomes greater than the target time. Using an interrupt introduces a known latency plus an uncertainty in the command execution time.

In order to avoid this uncertainty, a PTP trigger output signal is set high when the system time is greater than the target time. It is internally connected to the TIM2 input trigger. With this signal, the input capture feature, the output compare feature and the waveforms of the timer can be used, triggered by the synchronized PTP system time. No uncertainty is introduced since the clock of the timer (PCLK1: TIM2 APB1 clock) and PTP reference clock (HCLK) are synchronous.

This PTP trigger signal is connected to the TIM2 ITR1 input selectable by software. The connection is enabled through bits 11 and 10 in the TIM2 option register (TIM2_OR). *Figure 374* shows the connection.

**Figure 374. PTP trigger output to TIM2 ITR1 connection**

Ethernet MAC —PTP trigger—> ITR1 —> TIM2

ai15671

1162/1749 RM0090 Rev 18 STI

--- Page 188 ---

RM0090
Ethernet (ETH): media access control (MAC) with DMA controller

**PTP pulse-per-second output signal**

This PTP pulse output is used to check the synchronization between all nodes in the network. To be able to test the difference between the local slave clock and the master reference clock, both clocks were given a pulse-per-second (PPS) output signal that may be connected to an oscilloscope if necessary. The deviation between the two signals can therefore be measured. The pulse width of the PPS output is 125 ms.

The PPS output is enabled through a GPIO alternate function. (GPIO_AFR register).

The default frequency of the PPS output is 1 Hz. PPSFREQ[3:0] (in ETH_PTPPPSCR) can be used to set the frequency of the PPS output to $2^{PPSFREQ}$ Hz.

When set to 1 Hz, the PPS pulse width is 125 ms with binary rollover (TSSSR=0, bit 9 in ETH_PTPTSCR) and 100 ms with digital rollover (TSSSR=1). When set to 2 Hz and higher, the duty cycle of the PPS output is 50% with binary rollover.

With digital rollover (TSSSR=1), it is recommended not to use the PPS output with a frequency other than 1 Hz as it would have irregular waveforms (though its average frequency would always be correct during any one-second window).

Figure 375. PPS output

Ethernet MAC
PPS output
ai15672

