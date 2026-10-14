### 35.13.5 Host programming model

**Channel initialization**

The application must initialize one or more channels before it can communicate with connected devices. To initialize and enable a channel, the application must perform the following steps:

ST
RM0090 Rev 18
1489/1749

--- Page 125 ---

USB on-the-go high-speed (OTG_HS)
RM0090

1. Program the GINTMSK register to unmask the following:
2. Channel interrupt
    - Nonperiodic transmit FIFO empty for OUT transactions (applicable for Slave mode that operates in pipelined transaction-level with the packet count field programmed with more than one).
    - Nonperiodic transmit FIFO half-empty for OUT transactions (applicable for Slave mode that operates in pipelined transaction-level with the packet count field programmed with more than one).
3. Program the OTG_HS_HAINTMSK register to unmask the selected channels' interrupts.
4. Program the OTG_HS_HCINTMSK register to unmask the transaction-related interrupts of interest given in the host channel interrupt register.
5. Program the selected channel's OTG_HS_HCTSIZx register with the total transfer size, in bytes, and the expected number of packets, including short packets. The application must program the PID field with the initial data PID (to be used on the first OUT transaction or to be expected from the first IN transaction).
6. Program the selected channels in the OTG_HS_HCSPLTx register(s) with the hub and port addresses (split transactions only).
7. Program the selected channels in the HCDMAs register(s) with the buffer start address.
8. Program the OTG_HS_HCCHARx register of the selected channel with the device's endpoint characteristics, such as type, speed, direction, and so forth. (The channel can be enabled by setting the channel enable bit to 1 only when the application is ready to transmit or receive any packet).

**Halting a channel**

The application can disable any channel by programming the OTG_HS_HCCHARx register with the CHDIS and CHENA bits set to 1. This enables the OTG_HS host to flush the posted requests (if any) and generates a channel halted interrupt. The application must wait for the CHH interrupt in OTG_HS_HCINTx before reallocating the channel for other transactions. The OTG_HS host does not interrupt the transaction that has already been started on the USB.

To disable a channel in DMA mode operation, the application does not need to check for space in the request queue. The OTG_HS host checks for space to write the disable request on the disabled channel's turn during arbitration. Meanwhile, all posted requests are dropped from the request queue when the CHDIS bit in HCCHARx is set to 1.

Before disabling a channel, the application must ensure that there is at least one free space available in the nonperiodic request queue (when disabling a nonperiodic channel) or the periodic request queue (when disabling a periodic channel). The application can simply flush the posted requests when the Request queue is full (before disabling the channel), by programming the OTG_HS_HCCHARx register with the CHDIS bit set to 1, and the CHENA bit cleared to 0.

The application is expected to disable a channel on any of the following conditions:
1. When an XFRC interrupt in OTG_HS_HCINTx is received during a nonperiodic IN transfer or high-bandwidth interrupt IN transfer (Slave mode only)
2. When an STALL, TXERR, BBERR or DTERR interrupt in OTG_HS_HCINTx is received for an IN or OUT channel (Slave mode only). For high-bandwidth interrupt INs in Slave mode, once the application has received a DTERR interrupt it must disable the

1490/1749
RM0090 Rev 18
ST

--- Page 126 ---

RM0090
USB on-the-go high-speed (OTG_HS)

channel and wait for a channel halted interrupt. The application must be able to receive other interrupts (DTERR, NAK, Data, TXERR) for the same channel before receiving the halt.

3. When a DISCINT (Disconnect Device) interrupt in OTG_HS_GINTSTS is received. (The application is expected to disable all enabled channels
4. When the application aborts a transfer before normal completion.

**Ping protocol**

When the OTG_HS host operates in high speed, the application must initiate the ping protocol when communicating with high-speed bulk or control (data and status stage) OUT endpoints.

The application must initiate the ping protocol when it receives a NAK/NYET/TXERR interrupt. When the HS_OTG host receives one of the above responses, it does not continue any transaction for a specific endpoint, drops all posted or fetched OUT requests (from the request queue), and flushes the corresponding data (from the transmit FIFO).

This is valid in slave mode only. In Slave mode, the application can send a ping token either by setting the DOPING bit in HCTSIZx before enabling the channel or by just writing the HCTSIZx register with the DOPING bit set when the channel is already enabled. This enables the HS_OTG host to write a ping request entry to the request queue. The application must wait for the response to the ping token (a NAK, ACK, or TXERR interrupt) before continuing the transaction or sending another ping token. The application can continue the data transaction only after receiving an ACK from the OUT endpoint for the requested ping. In DMA mode operation, the application does not need to set the DOPING bit in HCTSIZx for a NAK/NYET response in case of Bulk/Control OUT. The OTG_HS host automatically sets the DOPING bit in HCTSIZx, and issues the ping tokens for Bulk/Control OUT. The HS_OTG host continues sending ping tokens until it receives an ACK, and then switches automatically to the data transaction.

**Operational model**

The application must initialize a channel before communicating to the connected device. This section explains the sequence of operation to be performed for different types of USB transactions.

*   **Writing the transmit FIFO**

The OTG_HS host automatically writes an entry (OUT request) to the periodic/nonperiodic request queue, along with the last word write of a packet. The application must ensure that at least one free space is available in the periodic/nonperiodic request queue before starting to write to the transmit FIFO. The application must always write to the transmit FIFO in words. If the packet size is non-word-aligned, the application must use padding. The OTG_HS host determines the actual packet size based on the programmed maximum packet size and transfer size.

ST
RM0090 Rev 18
1491/1749

--- Page 127 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 416. Transmit FIFO write task

Start
Read GNPTXSTS/HPTXFSIZ registers for available FIFO and queue spaces
Wait for TXFELVL or PTXFELVL interrupt in OTG_FS_GAHBCFG
1 MPS or LPS FIFO space available?
No
Yes
Write 1 packet data to Transmit FIFO
More packets to send?
No
Yes
Done

MPS: Maximum packet size
LPS: Last packet size
ai15673

*   **Reading the receive FIFO**
    The application must ignore all packet statuses other than IN data packet (bx0010).

1492/1749
RM0090 Rev 18
ST

--- Page 128 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Figure 417. Receive FIFO read task**

ai15674

*   **Bulk and control OUT/SETUP transactions**
    A typical bulk or control OUT/SETUP pipelined transaction-level operation is shown in Figure 418. See channel 1 (ch_1). Two bulk OUT packets are transmitted. A control SETUP transaction operates in the same way but has only one packet. The assumptions are:
    - The application is attempting to send two maximum-packet-size packets (transfer size = 1, 024 bytes).
    - The nonperiodic transmit FIFO can hold two packets (128 bytes for FS).
    - The nonperiodic request queue depth = 4.

*   **Normal bulk and control OUT/SETUP operations**
    The sequence of operations for channel 1 is as follows:
    a) Initialize channel 1
    b) Write the first packet for channel 1
    c) Along with the last word write, the core writes an entry to the nonperiodic request queue
    d) As soon as the nonperiodic queue becomes nonempty, the core attempts to send an OUT token in the current frame
    e) Write the second (last) packet for channel 1
    f) The core generates the XFRC interrupt as soon as the last transaction is completed successfully
    g) In response to the XFRC interrupt, de-allocate the channel for other transfers
    h) Handling nonACK responses

ST
RM0090 Rev 18
1493/1749

--- Page 129 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 418. Normal bulk/control OUT/SETUP and bulk/control IN transactions - DMA mode

Application
init_reg(ch_2)
init_reg(ch_1)
De-allocate (ch_1)
De-allocate (ch_2)

AHB
1 MPS
1 MPS
1 MPS
1 MPS

Host
Non-Periodic Request Queue
Assume that this queue can hold 4 entries.
ch_1
ch_2
ch_1
ch_2
ChHtd interrupt
ch_1
ch_2
ch_2
ch_2
ch_2

USB
OUT
DATA0 MPS
ACK
IN
DATA0
ACK
OUT
DATA1 MPS
ACK
IN
DATA1
ACK

Device

1494/1749
RM0090 Rev 18
ST

--- Page 130 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 419. Normal bulk/control OUT/SETUP and bulk/control IN transactions - Slave mode

The channel-specific interrupt service routine for bulk and control OUT/SETUP transactions in Slave mode is shown in the following code samples.

*   **Interrupt service routine for bulk/control OUT/SETUP and bulk/control IN transactions**
    a) Bulk/Control OUT/SETUP
    Unmask (NAK/TXERR/STALL/XFRC)
    if (XFRC)
    {
    Reset Error Count

STI
RM0090 Rev 18
1495/1749

--- Page 131 ---

USB on-the-go high-speed (OTG_HS)

RM0090

```
Mask ACK
De-allocate Channel
}
else if (STALL)
{
Transfer Done = 1
Unmask CHH
Disable Channel
}
else if (NAK or TXERR )
{
Rewind Buffer Pointers
Unmask CHH
Disable Channel
if (TXERR)
{
Increment Error Count
Unmask ACK
}
else
{
Reset Error Count
}
}
else if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel
}
}
else if (ACK)
{
Reset Error Count
Mask ACK
}
```

The application is expected to write the data packets into the transmit FIFO as and when the space is available in the transmit FIFO and the Request queue. The application can make use of the NPTXFE interrupt in OTG_HS_GINTSTS to find the transmit FIFO space.

b) Bulk/Control IN

Unmask (TXERR/XFRC/BBERR/STALL/DTERR)
if (XFRC)
{
Reset Error Count
Unmask CHH
Disable Channel

1496/1749

RM0090 Rev 18

ST

--- Page 132 ---

RM0090
USB on-the-go high-speed (OTG_HS)

```text
Reset Error Count
Mask ACK
}
else if (TXERR or BBERR or STALL)
{
Unmask CHH
Disable Channel
if (TXERR)
{
Increment Error Count
Unmask ACK
}
}
else if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel
}
}
else if (ACK)
{
Reset Error Count
Mask ACK
}
else if (DTERR)
{
Reset Error Count
}
```

The application is expected to write the requests as and when the Request queue space is available and until the XFRC interrupt is received.

*   **Bulk and control IN transactions**

A typical bulk or control IN pipelined transaction-level operation is shown in Figure 420. See channel 2 (ch_2). The assumptions are:

- The application is attempting to receive two maximum-packet-size packets (transfer size = 1 024 bytes).
- The receive FIFO can contain at least one maximum-packet-size packet and two status words per packet (72 bytes for FS).
- The nonperiodic request queue depth = 4.

ST
RM0090 Rev 18
1497/1749

--- Page 133 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 420. Bulk/control IN transactions - DMA mode

Application AHB Host USB Device

1
int_reg(ch_2)
1
int_reg(ch_1)
2
1 MPS
Periodic Request Queue
Assume that this queue can hold 4 entries.
3
ch_1
ch_2
2
Odd (micro) frame
OUT
DATA0 MPS
5
ChHltd interrupt
4
3
ACK
int_reg(ch_1)
IN
1 MPS
DATA0
1 MPS
ACK
4
ch_1
ch_2
ChHltd interrupt
int_reg(ch_2)
5
Even (micro) frame
OUT
DATA1 MPS
ACK
IN
DATA1
ChHltd interrupt
int_reg(ch_1)
1 MPS

1498/1749
RM0090 Rev 18
ST

--- Page 134 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 421. Bulk/control IN transactions - Slave mode

The sequence of operations is as follows:
a) Initialize channel 2.
b) Set the CHENA bit in HCCHAR2 to write an IN request to the nonperiodic request queue.
c) The core attempts to send an IN token after completing the current OUT transaction.
d) The core generates an RXFLVL interrupt as soon as the received packet is written to the receive FIFO.
e) In response to the RXFLVL interrupt, mask the RXFLVL interrupt and read the received packet status to determine the number of bytes received, then read the

ST
RM0090 Rev 18
1499/1749

--- Page 135 ---

USB on-the-go high-speed (OTG_HS)
RM0090

receive FIFO accordingly. Following this, unmask the RXFLVL interrupt.
f) The core generates the RXFLVL interrupt for the transfer completion status entry in the receive FIFO.
g) The application must read and ignore the receive packet status when the receive packet status is not an IN data packet (PKTSTS in GRXSTS ≠ 0b0010).
h) The core generates the XFRC interrupt as soon as the receive packet status is read.
i) In response to the XFRC interrupt, disable the channel and stop writing the OTG_HS_HCCHAR2 register for further requests. The core writes a channel disable request to the nonperiodic request queue as soon as the OTG_HS_HCCHAR2 register is written.
j) The core generates the RXFLVL interrupt as soon as the halt status is written to the receive FIFO.
k) Read and ignore the receive packet status.
l) The core generates a CHH interrupt as soon as the halt status is popped from the receive FIFO.
m) In response to the CHH interrupt, de-allocate the channel for other transfers.
n) Handling nonACK responses

*   **Control transactions in slave mode**
    Setup, Data, and Status stages of a control transfer must be performed as three separate transfers. Setup-, Data- or Status-stage OUT transactions are performed similarly to the bulk OUT transactions explained previously. Data- or Status-stage IN transactions are performed similarly to the bulk IN transactions explained previously. For all three stages, the application is expected to set the EPTYP field in OTG_HS_HCCHAR1 to Control. During the Setup stage, the application is expected to set the PID field in OTG_HS_HCTSIZ1 to SETUP.

*   **Interrupt OUT transactions**
    A typical interrupt OUT operation in Slave mode is shown in Figure 422. The assumptions are:
    - The application is attempting to send one packet in every frame (up to 1 maximum packet size), starting with the odd frame (transfer size = 1 024 bytes)
    - The periodic transmit FIFO can hold one packet (1 KB)
    - Periodic request queue depth = 4
    The sequence of operations is as follows:
    a) Initialize and enable channel 1. The application must set the ODDFRM bit in OTG_HS_HCCHAR1.
    b) Write the first packet for channel 1. For a high-bandwidth interrupt transfer, the application must write the subsequent packets up to MCNT (maximum number of packets to be transmitted in the next frame times) before switching to another channel.
    c) Along with the last word write of each packet, the OTG_HS host writes an entry to the periodic request queue.
    d) The OTG_HS host attempts to send an OUT token in the next (odd) frame.
    e) The OTG_HS host generates an XFRC interrupt as soon as the last packet is transmitted successfully.
    f) In response to the XFRC interrupt, reinitialize the channel for the next transfer.

1500/1749
RM0090 Rev 18

--- Page 136 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 422. Normal interrupt OUT/IN transactions - DMA mode

Application
init_reg(ch_2)
1
init_reg(ch_1)
1
5
init_reg(ch_1)
init_reg(ch_2)
5
init_reg(ch_1)

AHB
1
MPS
4
ChHld interrupt
1
MPS
1
MPS
4
ChHld interrupt
1
MPS

Host
2
3
ch_1
ch_2
2
4
ch_1
ch_2

USB
Periodic Request Queue
Assume that this queue can hold 4 entries.
3
OUT
ACK
IN
DATA0
ACK
3
OUT
ACK
IN
DATA1

Device
Odd (micro) frame
Even (micro) frame

ST
RM0090 Rev 18
1501/1749

--- Page 137 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 423. Normal interrupt OUT/IN transactions - Slave mode

Application AHB Host USB Device

1 init_reg(ch_1)
1 init_reg(ch_2)
2 write_tx_fifo (ch_1)
2 set_ch_en (ch_2)
3 MPS
Periodic Request Queue Assume that this queue can hold 4 entries.
4 ch_1
ch_2
Odd (micro) frame
OUT
DATA0 MPS
ACK
IN
DATA0
ACK
6 init_reg(ch_1)
XferCompl interrupt
5 MPS
6 read_rx_sts read_rx_fifo
RxFLVl interrupt
1 MPS
7 read_rx_sts
RxFLVl interrupt
8 ch_1
ch_2
XferCompl interrupt
9 init_reg(ch_2)
set_ch_en (ch_2)
XferCompl interrupt
Even (micro) frame
OUT
DATA1 MPS
ACK
IN
DATA1

• Interrupt service routine for interrupt OUT/IN transactions
a) Interrupt OUT
Unmask (NAK/TXERR/STALL/XFRC/FRMOR)
if (XFRC)
{
Reset Error Count
Mask ACK
De-allocate Channel
}
else
if (STALL or FRMOR)
{

1502/1749
RM0090 Rev 18
ST

--- Page 138 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Mask **ACK**
Unmask **CHH**
Disable Channel
if (**STALL**)
{
Transfer Done = 1
}
else
if (**NAK** or **TXERR**)
{
Rewind Buffer Pointers
Reset Error Count
Mask **ACK**
Unmask **CHH**
Disable Channel
}
else
if (**CHH**)
{
Mask **CHH**
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel (in next b_interval - 1 Frame)
}
}
else
if (**ACK**)
{
Reset Error Count
Mask **ACK**
}

The application is expected to write the data packets into the transmit FIFO when the space is available in the transmit FIFO and the Request queue up to the count specified in the MCNT field before switching to another channel. The application uses the NPTXFEE interrupt in OTG_HS_GINTSTS to find the transmit FIFO space.

b) Interrupt IN
Unmask (NAK/TXERR/XFRC/BBERR/STALL/FRMOR/DTERR)
if (**XFRC**)
{
Reset Error Count
Mask **ACK**
if (OTG_HS_HCTSIZx.PKTCNT == 0)
{
De-allocate Channel
}
else
{

STI
RM0090 Rev 18
1503/1749

--- Page 139 ---

USB on-the-go high-speed (OTG_HS)

RM0090

```c
Transfer Done = 1
Unmask CHH
Disable Channel
}
else
if (STALL or FRMOR or NAK or DTERR or BBERR)
{
Mask ACK
Unmask CHH
Disable Channel
if (STALL or BBERR)
{
Reset Error Count
Transfer Done = 1
}
else
if (!FRMOR)
{
Reset Error Count
}
}
else
if (TXERR)
{
Increment Error Count
Unmask ACK
Unmask CHH
Disable Channel
}
else
if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
Re-initialize Channel (in next b_interval - 1 /Frame)
}
}
else
if (ACK)
{
Reset Error Count
Mask ACK
```

1504/1749

RM0090 Rev 18

ST

--- Page 140 ---

RM0090
USB on-the-go high-speed (OTG_HS)

}
The application is expected to write the requests for the same channel when the
Request queue space is available up to the count specified in the MCNT field before
switching to another channel (if any).

*   **Interrupt IN transactions**
    The assumptions are:
    - The application is attempting to receive one packet (up to 1 maximum packet size)
      in every frame, starting with odd (transfer size = 1 024 bytes).
    - The receive FIFO can hold at least one maximum-packet-size packet and two
      status words per packet (1 031 bytes).
    - Periodic request queue depth = 4.

*   **Normal interrupt IN operation**
    The sequence of operations is as follows:
    a) Initialize channel 2. The application must set the ODDFRM bit in
       OTG_HS_HCCHAR2.
    b) Set the CHENA bit in OTG_HS_HCCHAR2 to write an IN request to the periodic
       request queue. For a high-bandwidth interrupt transfer, the application must write
       the OTG_HS_HCCHAR2 register MCNT (maximum number of expected packets
       in the next frame times) before switching to another channel.
    c) The OTG_HS host writes an IN request to the periodic request queue for each
       OTG_HS_HCCHAR2 register write with the CHENA bit set.
    d) The OTG_HS host attempts to send an IN token in the next (odd) frame.
    e) As soon as the IN packet is received and written to the receive FIFO, the OTG_HS
       host generates an RXFLVL interrupt.
    f) In response to the RXFLVL interrupt, read the received packet status to determine
       the number of bytes received, then read the receive FIFO accordingly. The
       application must mask the RXFLVL interrupt before reading the receive FIFO, and
       unmask after reading the entire packet.
    g) The core generates the RXFLVL interrupt for the transfer completion status entry
       in the receive FIFO. The application must read and ignore the receive packet
       status when the receive packet status is not an IN data packet (PKTSTS in
       GRXSTSR ≠ 0b0010).
    h) The core generates an XFRC interrupt as soon as the receive packet status is
       read.
    i) In response to the XFRC interrupt, read the PKTCNT field in OTG_HS_HCTSIZ2.
       If the PKTCNT bit in OTG_HS_HCTSIZ is not equal to 0, disable the channel
       before re-initializing the channel for the next transfer, if any). If PKTCNT bit in

ST
RM0090 Rev 18
1505/1749

--- Page 141 ---

USB on-the-go high-speed (OTG_HS)
RM0090

OTG_HS_HCTSIZ2 = 0, reinitialize the channel for the next transfer. This time, the application must reset the ODDFRM bit in OTG_HS_HCCHAR2.

*   **Isochronous OUT transactions**
    A typical isochronous OUT operation in Slave mode is shown in Figure 424. The assumptions are:
    - The application is attempting to send one packet every frame (up to 1 maximum packet size), starting with an odd frame. (transfer size = 1 024 bytes).
    - The periodic transmit FIFO can hold one packet (1 KB).
    - Periodic request queue depth = 4.
    The sequence of operations is as follows:
    a) Initialize and enable channel 1. The application must set the ODDFRM bit in OTG_HS_HCCHAR1.
    b) Write the first packet for channel 1. For a high-bandwidth isochronous transfer, the application must write the subsequent packets up to MCNT (maximum number of packets to be transmitted in the next frame times before switching to another channel.
    c) Along with the last word write of each packet, the OTG_HS host writes an entry to the periodic request queue.
    d) The OTG_HS host attempts to send the OUT token in the next frame (odd).
    e) The OTG_HS host generates the XFRC interrupt as soon as the last packet is transmitted successfully.
    f) In response to the XFRC interrupt, reinitialize the channel for the next transfer.
    g) Handling nonACK responses

1506/1749
RM0090 Rev 18

--- Page 142 ---

RM0090

Figure 424. Normal isochronous OUT/IN transactions - DMA mode

USB on-the-go high-speed (OTG_HS)

Application
AHB
Host
USB
Device

rm_reg(ch_2)
rm_reg(ch_1)
1
1
2
2
3
3
4
4
5
5

MPS
1
Ch-Hld interrupt
MPS
1
MPS
1
Ch-Hld interrupt
Ch-Hld interrupt
MPS
1

Periodic Request Queue
Note that this queue can hold 4 entries.

OUT
DATA0
MPS
IN
DATA0
Odd (micro) frame

OUT
DATA0
MPS
IN
DATA0
Even (micro) frame

RM0090 Rev 18
1507/1749

ST

--- Page 143 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 425. Normal isochronous OUT/IN transactions - Slave mode

Application AHB Host USB Device

init_reg(ch_2)
init_reg(ch_1)
1 MPS
3
Periodic Request Queue
Assume that this queue can hold 4 entries.
1
write_br_fifo (ch_1)
set_ch_en (ch_2)
2
ch_1
4
ch_2
3
Odd (micro) frame
OUT
DATA0 MPS
6
XferCompl interrupt
5
init_reg(ch_1)
write_br_fifo (ch_1)
1 MPS
4
IN
DATA0
RxFLvl interrupt
1 MPS
6
read_rx_sts
read_rx_fifo
RxFLvl interrupt
1 MPS
7
read_rx_sts
XferCompl interrupt
8
ch_1
ch_2
9
init_reg(ch_2)
set_ch_en (ch_2)
Even (micro) frame
OUT
DATA0 MPS
XferCompl interrupt
init_reg(ch_1)
write_br_fifo (ch_1)
1 MPS
IN
DATA 0

* Interrupt service routine for isochronous OUT/IN transactions
Code sample: Isochronous OUT
Unmask (FRMOR/XFRC)
if (XFRC)
{
De-allocate Channel
}
else
if (FRMOR)
{
Unmask CHH
Disable Channel
}

1508/1749
RM0090 Rev 18
ST

--- Page 144 ---

RM0090
USB on-the-go high-speed (OTG_HS)

```
else
if (CHH)
{
Mask CHH
De-allocate Channel
}
Code sample: Isochronous IN
Unmask (TXERR/XFRC/FRMOR/BBERR)
if (XFRC or FRMOR)
{
if (XFRC and (OTG_HS_HCTSIZx.PKTCNT == 0))
{
Reset Error Count
De-allocate Channel
}
else
{
Unmask CHH
Disable Channel
}
}
else
if (TXERR or BBERR)
{
Increment Error Count
Unmask CHH
Disable Channel
}
else
if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel
}
}
```

ST
RM0090 Rev 18
1509/1749

--- Page 145 ---

USB on-the-go high-speed (OTG_HS)

RM0090

*   **Isochronous IN transactions**
    The assumptions are:
    - The application is attempting to receive one packet (up to 1 maximum packet size) in every frame starting with the next odd frame (transfer size = 1 024 bytes).
    - The receive FIFO can hold at least one maximum-packet-size packet and two status words per packet (1 031 bytes).
    - Periodic request queue depth = 4.
    The sequence of operations is as follows:
    a) Initialize channel 2. The application must set the ODDFRM bit in OTG_HS_HCCHAR2.
    b) Set the CHENA bit in OTG_HS_HCCHAR2 to write an IN request to the periodic request queue. For a high-bandwidth isochronous transfer, the application must write the OTG_HS_HCCHAR2 register MCNT (maximum number of expected packets in the next frame times) before switching to another channel.
    c) The OTG_HS host writes an IN request to the periodic request queue for each OTG_HS_HCCHAR2 register write with the CHENA bit set.
    d) The OTG_HS host attempts to send an IN token in the next odd frame.
    e) As soon as the IN packet is received and written to the receive FIFO, the OTG_HS host generates an RXFLVL interrupt.
    f) In response to the RXFLVL interrupt, read the received packet status to determine the number of bytes received, then read the receive FIFO accordingly. The application must mask the RXFLVL interrupt before reading the receive FIFO, and unmask it after reading the entire packet.
    g) The core generates an RXFLVL interrupt for the transfer completion status entry in the receive FIFO. This time, the application must read and ignore the receive packet status when the receive packet status is not an IN data packet (PKTSTS bit in OTG_HS_GRXSTSR ≠ 0b0010).
    h) The core generates an XFRC interrupt as soon as the receive packet status is read.
    i) In response to the XFRC interrupt, read the PKTCNT field in OTG_HS_HCTSIZ2. If PKTCNT ≠ 0 in OTG_HS_HCTSIZ2, disable the channel before re-initializing the channel for the next transfer, if any. If PKTCNT = 0 in OTG_HS_HCTSIZ2, reinitialize the channel for the next transfer. This time, the application must reset the ODDFRM bit in OTG_HS_HCCHAR2.

*   **Selecting the queue depth**
    Choose the periodic and nonperiodic request queue depths carefully to match the number of periodic/nonperiodic endpoints accessed.
    The nonperiodic request queue depth affects the performance of nonperiodic transfers. The deeper the queue (along with sufficient FIFO size), the more often the core is able to pipeline nonperiodic transfers. If the queue size is small, the core is able to put in new requests only when the queue space is freed up.
    The core's periodic request queue depth is critical to perform periodic transfers as scheduled. Select the periodic queue depth, based on the number of periodic transfers scheduled in a micro-frame. In Slave mode, however, the application must also take into account the disable entry that must be put into the queue. So, if there are two nonhigh-bandwidth periodic endpoints, the periodic request queue depth must be at least 4. If at least one high-bandwidth endpoint is supported, the queue depth must be

1510/1749
RM0090 Rev 18
ST

--- Page 146 ---

RM0090
USB on-the-go high-speed (OTG_HS)

8. If the periodic request queue depth is smaller than the periodic transfers scheduled in a micro-frame, a frame overrun condition occurs.

*   **Handling babble conditions**
    OTG_HS controller handles two cases of babble: packet babble and port babble. Packet babble occurs if the device sends more data than the maximum packet size for the channel. Port babble occurs if the core continues to receive data from the device at EOF2 (the end of frame 2, which is very close to SOF).

    When OTG_HS controller detects a packet babble, it stops writing data into the Rx buffer and waits for the end of packet (EOP). When it detects an EOP, it flushes already written data in the Rx buffer and generates a Babble interrupt to the application.

    When OTG_HS controller detects a port babble, it flushes the RxFIFO and disables the port. The core then generates a Port disabled interrupt (HPRTINT in OTG_HS_GINTSTS, PENCHNG in OTG_HS_HPRT). On receiving this interrupt, the application must determine that this is not due to an overcurrent condition (another cause of the Port Disabled interrupt) by checking POCA in OTG_HS_HPRT, then perform a soft reset. The core does not send any more tokens after it has detected a port babble condition.

*   **Bulk and control OUT/SETUP transactions in DMA mode**
    The sequence of operations is as follows:
    a) Initialize and enable channel 1 as explained in Section : Channel initialization.
    b) The HS_OTG host starts fetching the first packet as soon as the channel is enabled. For internal DMA mode, the OTG_HS host uses the programmed DMA address to fetch the packet.
    c) After fetching the last word of the second (last) packet, the OTG_HS host masks channel 1 internally for further arbitration.
    d) The HS_OTG host generates a CHH interrupt as soon as the last packet is sent.
    e) In response to the CHH interrupt, de-allocate the channel for other transfers.

*   **NAK and NYET handling with internal DMA**
    a) The OTG_HS host sends a bulk OUT transaction.
    b) The device responds with NAK or NYET.
    c) If the application has unmasked NAK or NYET, the core generates the corresponding interrupt(s) to the application. The application is not required to service these interrupts, since the core takes care of rewinding the buffer pointers and re-initializing the Channel without application intervention.
    d) The core automatically issues a ping token.
    e) When the device returns an ACK, the core continues with the transfer. Optionally, the application can utilize these interrupts, in which case the NAK or NYET interrupt is masked by the application.

    The core does not generate a separate interrupt when NAK or NYET is received by the host functionality.

*   **Bulk and control IN transactions in DMA mode**
    The sequence of operations is as follows:
    a) Initialize and enable the used channel (channel x) as explained in Section : Channel initialization.
    b) The OTG_HS host writes an IN request to the request queue as soon as the channel receives the grant from the arbiter (arbitration is performed in a round-robin fashion).

ST
RM0090 Rev 18
1511/1749

--- Page 147 ---

USB on-the-go high-speed (OTG_HS)
RM0090

c) The OTG_HS host starts writing the received data to the system memory as soon as the last byte is received with no errors.
d) When the last packet is received, the OTG_HS host sets an internal flag to remove any extra IN requests from the request queue.
e) The OTG_HS host flushes the extra requests.
f) The final request to disable channel x is written to the request queue. At this point, channel 2 is internally masked for further arbitration.
g) The OTG_HS host generates the CHH interrupt as soon as the disable request comes to the top of the queue.
h) In response to the CHH interrupt, de-allocate the channel for other transfers.

*   **Interrupt OUT transactions in DMA mode**
    a) Initialize and enable channel x as explained in Section : Channel initialization.
    b) The OTG_HS host starts fetching the first packet as soon the channel is enabled and writes the OUT request along with the last word fetch. In high-bandwidth transfers, the HS_OTG host continues fetching the next packet (up to the value specified in the MC field) before switching to the next channel.
    c) The OTG_HS host attempts to send the OUT token at the beginning of the next odd frame/micro-frame.
    d) After successfully transmitting the packet, the OTG_HS host generates a CHH interrupt.
    e) In response to the CHH interrupt, reinitialize the channel for the next transfer.

*   **Interrupt IN transactions in DMA mode**
    The sequence of operations (channelx) is as follows:
    a) Initialize and enable channel x as explained in Section : Channel initialization.
    b) The OTG_HS host writes an IN request to the request queue as soon as the channel x gets the grant from the arbiter (round-robin with fairness). In high-bandwidth transfers, the OTG_HS host writes consecutive writes up to MC times.
    c) The OTG_HS host attempts to send an IN token at the beginning of the next (odd) frame/micro-frame.
    d) As soon the packet is received and written to the receive FIFO, the OTG_HS host generates a CHH interrupt.
    e) In response to the CHH interrupt, reinitialize the channel for the next transfer.

*   **Isochronous OUT transactions in DMA mode**
    a) Initialize and enable channel x as explained in Section : Channel initialization.
    b) The OTG_HS host starts fetching the first packet as soon the channel is enabled, and writes the OUT request along with the last word fetch. In high-bandwidth transfers, the OTG_HS host continues fetching the next packet (up to the value specified in the MC field) before switching to the next channel.
    c) The OTG_HS host attempts to send an OUT token at the beginning of the next (odd) frame/micro-frame.
    d) After successfully transmitting the packet, the HS_OTG host generates a CHH interrupt.
    e) In response to the CHH interrupt, reinitialize the channel for the next transfer.

*   **Isochronous IN transactions in DMA mode**
    The sequence of operations ((channel x) is as follows:

1512/1749
RM0090 Rev 18
ST

--- Page 148 ---

RM0090
USB on-the-go high-speed (OTG_HS)

a) Initialize and enable channel x as explained in Section : Channel initialization.
b) The OTG_HS host writes an IN request to the request queue as soon as the channel x gets the grant from the arbiter (round-robin with fairness). In high-bandwidth transfers, the OTG_HS host performs consecutive write operations up to MC times.
c) The OTG_HS host attempts to send an IN token at the beginning of the next (odd) frame/micro-frame.
d) As soon the packet is received and written to the receive FIFO, the OTG_HS host generates a CHH interrupt.
e) In response to the CHH interrupt, reinitialize the channel for the next transfer.

*   **Bulk and control OUT/SETUP split transactions in DMA mode**
    The sequence of operations in (channel x) is as follows:
    a) Initialize and enable channel x for start split as explained in Section : Channel initialization.
    b) The OTG_HS host starts fetching the first packet as soon the channel is enabled and writes the OUT request along with the last word fetch.
    c) After successfully transmitting start split, the OTG_HS host generates the CHH interrupt.
    d) In response to the CHH interrupt, set the COMPLSPLT bit in HCSPLT1 to send the complete split.
    e) After successfully transmitting complete split, the OTG_HS host generates the CHH interrupt.
    f) In response to the CHH interrupt, de-allocate the channel.

*   **Bulk/Control IN split transactions in DMA mode**
    The sequence of operations (channel x) is as follows:
    a) Initialize and enable channel x as explained in Section : Channel initialization.
    b) The OTG_HS host writes the start split request to the nonperiodic request after getting the grant from the arbiter. The OTG_HS host masks the channel x internally for the arbitration after writing the request.
    c) As soon as the IN token is transmitted, the OTG_HS host generates the CHH interrupt.
    d) In response to the CHH interrupt, set the COMPLSPLT bit in HCSPLT2 and re-enable the channel to send the complete split token. This unmasks channel x for arbitration.
    e) The OTG_HS host writes the complete split request to the nonperiodic request after receiving the grant from the arbiter.
    f) The OTG_HS host starts writing the packet to the system memory after receiving the packet successfully.
    g) As soon as the received packet is written to the system memory, the OTG_HS host generates a CHH interrupt.
    h) In response to the CHH interrupt, de-allocate the channel.

*   **Interrupt OUT split transactions in DMA mode**
    The sequence of operations in (channel x) is as follows:
    a) Initialize and enable channel 1 for start split as explained in Section : Channel initialization. The application must set the ODDFRM bit in HCCHAR1.
    b) The HS_OTG host starts reading the packet.

STI
RM0090 Rev 18
1513/1749

--- Page 149 ---

USB on-the-go high-speed (OTG_HS)
RM0090

c) The HS_OTG host attempts to send the start split transaction.
d) After successfully transmitting the start split, the OTG_HS host generates the CHH interrupt.
e) In response to the CHH interrupt, set the COMPLSPLT bit in HCSPLT1 to send the complete split.
f) After successfully completing the complete split transaction, the OTG_HS host generates the CHH interrupt.
g) In response to CHH interrupt, de-allocate the channel.

*   **Interrupt IN split transactions in DMA mode**
    The sequence of operations in (channel x) is as follows:
    a) Initialize and enable channel x for start split as explained in *Section : Channel initialization*.
    b) The OTG_HS host writes an IN request to the request queue as soon as channel x receives the grant from the arbiter.
    c) The OTG_HS host attempts to send the start split IN token at the beginning of the next odd micro-frame.
    d) The OTG_HS host generates the CHH interrupt after successfully transmitting the start split IN token.
    e) In response to the CHH interrupt, set the COMPLSPLT bit in HCSPLT2 to send the complete split.
    f) As soon as the packet is received successfully, the OTG_HS host starts writing the data to the system memory.
    g) The OTG_HS host generates the CHH interrupt after transferring the received data to the system memory.
    h) In response to the CHH interrupt, de-allocate or reinitialize the channel for the next start split.

*   **Isochronous OUT split transactions in DMA mode**
    The sequence of operations (channel x) is as follows:
    a) Initialize and enable channel x for start split (begin) as explained in *Section : Channel initialization*. The application must set the ODDFRM bit in HCCHAR1. Program the MPS field.
    b) The HS_OTG host starts reading the packet.
    c) After successfully transmitting the start split (begin), the HS_OTG host generates the CHH interrupt.
    d) In response to the CHH interrupt, reinitialize the registers to send the start split (end).
    e) After successfully transmitting the start split (end), the OTG_HS host generates a CHH interrupt.
    f) In response to the CHH interrupt, de-allocate the channel.

*   **Isochronous IN split transactions in DMA mode**
    The sequence of operations (channel x) is as follows:
    a) Initialize and enable channel x for start split as explained in *Section : Channel initialization*.
    b) The OTG_HS host writes an IN request to the request queue as soon as channel x receives the grant from the arbiter.

1514/1749
RM0090 Rev 18
ST

--- Page 150 ---

RM0090
USB on-the-go high-speed (OTG_HS)

c) The OTG_HS host attempts to send the start split IN token at the beginning of the next odd micro-frame.
d) The OTG_HS host generates the CHH interrupt after successfully transmitting the start split IN token.
e) In response to the CHH interrupt, set the COMPLSPLT bit in HCSPLT2 to send the complete split.
f) As soon as the packet is received successfully, the OTG_HS host starts writing the data to the system memory.
g) The OTG_HS host generates the CHH interrupt after transferring the received data to the system memory. In response to the CHH interrupt, de-allocate the channel or reinitialize the channel for the next start split.

