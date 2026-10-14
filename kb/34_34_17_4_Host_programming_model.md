### 34.17.4 Host programming model

#### Channel initialization

The application must initialize one or more channels before it can communicate with connected devices. To initialize and enable a channel, the application must perform the following steps:

1. Program the OTG_FS_GINTMSK register to unmask the following:
    1. Channel interrupt
        * Non-periodic transmit FIFO empty for OUT transactions (applicable when operating in pipelined transaction-level with the packet count field programmed with more than one).
        * Non-periodic transmit FIFO half-empty for OUT transactions (applicable when operating in pipelined transaction-level with the packet count field programmed with more than one).
2. Program the OTG_FS_HAINTMSK register to unmask the selected channels' interrupts.
3. Program the OTG_FS_HCINTMSK register to unmask the transaction-related interrupts of interest given in the host channel interrupt register.
4. Program the selected channel's OTG_FS_HCTSIZx register with the total transfer size, in bytes, and the expected number of packets, including short packets. The application must program the PID field with the initial data PID (to be used on the first OUT transaction or to be expected from the first IN transaction).
5. Program the OTG_FS_HCCHARx register of the selected channel with the device's endpoint characteristics, such as type, speed, direction, and so forth. (The channel can be enabled by setting the channel enable bit to 1 only when the application is ready to transmit or receive any packet).

#### Halting a channel

The application can disable any channel by programming the OTG_FS_HCCHARx register with the CHDIS and CHENA bits set to 1. This enables the OTG_FS host to flush the posted requests (if any) and generates a channel halted interrupt. The application must wait for the CHH interrupt in OTG_FS_HCINTx before reallocating the channel for other transactions. The OTG_FS host does not interrupt the transaction that has already been started on the USB.

Before disabling a channel, the application must ensure that there is at least one free space available in the non-periodic request queue (when disabling a non-periodic channel) or the periodic request queue (when disabling a periodic channel). The application can simply flush the posted requests when the Request queue is full (before disabling the channel), by programming the OTG_FS_HCCHARx register with the CHDIS bit set to 1, and the CHENA bit cleared to 0.

The application is expected to disable a channel on any of the following conditions:

ST
RM0090 Rev 18
1337/1749

--- Page 168 ---

USB on-the-go full-speed (OTG_FS)
RM0090

1. When an STALL, TXERR, BBERR or DTERR interrupt in OTG_FS_HCINTx is received for an IN or OUT channel. The application must be able to receive other interrupts (DTERR, Nak, Data, TXERR) for the same channel before receiving the halt.
2. When a DISCINT (Disconnect Device) interrupt in OTG_FS_GINTSTS is received. (The application is expected to disable all enabled channels).
3. When the application aborts a transfer before normal completion.

**Operational model**

The application must initialize a channel before communicating to the connected device. This section explains the sequence of operation to be performed for different types of USB transactions.

* **Writing the transmit FIFO**
The OTG_FS host automatically writes an entry (OUT request) to the periodic/non-periodic request queue, along with the last word write of a packet. The application must ensure that at least one free space is available in the periodic/non-periodic request queue before starting to write to the transmit FIFO. The application must always write to the transmit FIFO in words. If the packet size is non-word aligned, the application must use padding. The OTG_FS host determines the actual packet size based on the programmed maximum packet size and transfer size.

Figure 396. Transmit FIFO write task

Start
1 MPS or LPS FIFO space available?
Yes
Write 1 packet data to transmit FIFO
More packets to send?
Yes
No
Done
Read GNPTXSTS/HPTXFSIZ registers for available FIFO and queue spaces
Wait for NPTXFE/PTXFE interrupt in OTG_FS_GINTSTS
No
MPS: Maximum packet size
LPS: Last packet size
ai15673b

* **Reading the receive FIFO**

1338/1749
RM0090 Rev 18
STI

--- Page 169 ---

RM0090
USB on-the-go full-speed (OTG_FS)

The application must ignore all packet statuses other than IN data packet (bx0010).

**Figure 397. Receive FIFO read task**

Start
RXFLVL interrupt?
No
Yes
Unmask RXFLVL interrupt
Mask RXFLVL interrupt
Unmask RXFLVL interrupt
Read the received packet from the Receive FIFO
Read OTG_FS_GRXSTSP
PKTSTS 0b0010?
No
Yes
BCNT > 0?
No
Yes

ai15674

*   **Bulk and control OUT/SETUP transactions**
    A typical bulk or control OUT/SETUP pipelined transaction-level operation is shown in *Figure 398*. See channel 1 (ch_1). Two bulk OUT packets are transmitted. A control SETUP transaction operates in the same way but has only one packet. The assumptions are:
    *   The application is attempting to send two maximum-packet-size packets (transfer size = 1, 024 bytes).
    *   The non-periodic transmit FIFO can hold two packets (128 bytes for FS).
    *   The non-periodic request queue depth = 4.

*   **Normal bulk and control OUT/SETUP operations**
    The sequence of operations in (channel 1) is as follows:
    a) Initialize channel 1
    b) Write the first packet for channel 1
    c) Along with the last word write, the core writes an entry to the non-periodic request queue
    d) As soon as the non-periodic queue becomes non-empty, the core attempts to send an OUT token in the current frame
    e) Write the second (last) packet for channel 1
    f) The core generates the XFRC interrupt as soon as the last transaction is completed successfully

STi
RM0090 Rev 18
1339/1749

--- Page 170 ---

USB on-the-go full-speed (OTG_FS)
RM0090

g) In response to the XFRC interrupt, de-allocate the channel for other transfers
h) Handling non-ACK responses

Figure 398. Normal bulk/control OUT/SETUP and bulk/control IN transactions

| Application | AHB | Host | USB | Device |
|---|---|---|---|---|
| init_reg(ch_2) |  |  |  |  |
| init_regch_1 |  |  |  |  |
| write_tx_fifo (ch_1) | 1 MPS |  |  | Non-Periodic Request Queue Assume that this queue can hold 4 entries. |
| set_ch_en (ch_2) |  |  | ch_1 |  |
| write_tx_fifo (ch_1) | 1 MPS |  | ch_2 |  |
| set_ch_en (ch_2) |  |  | ch_1 | OUT |
|  |  |  | ch_2 | DATA0 MPS |
| set_ch_en (ch_2) |  |  |  | ACK |
|  |  |  |  | IN |
|  | RXFLVL interrupt |  |  | DATA0 |
| read_rx_sts read_rx_fifo |  |  |  | ACK |
| set_ch_en (ch_2) | 1 MPS |  | ch_1 | OUT |
|  |  |  | ch_2 | DATA1 MPS |
|  |  |  | ch_2 | ACK |
| De-allocate (ch_1) | XFRC interrupt |  |  | IN |
|  |  |  |  | DATA1 |
|  | RXFLVL interrupt |  |  | ACK |
| read_rx_sts read_rx_fifo | 1 MPS |  |  |  |
| read_rx_sts | RXFLVL interrupt |  | ch_2 |  |
| Disable (ch_2) | XFRC interrupt |  |  |  |
| read_rx_sts | RXFLVL interrupt |  |  |  |
| De-allocate (ch_2) | CHH interrupt |  |  |  |


ai15675

The channel-specific interrupt service routine for bulk and control OUT/SETUP transactions is shown in the following code samples.

*   Interrupt service routine for bulk/control OUT/SETUP and bulk/control IN transactions
    a) Bulk/Control OUT/SETUP

Unmask (NAK/TXERR/STALL/XFRC)
if (XFRC)
{

1340/1749
RM0090 Rev 18
ST

--- Page 171 ---

**RM0090**
**USB on-the-go full-speed (OTG_FS)**

```c
    Reset Error Count
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

The application is expected to write the data packets into the transmit FIFO as and when the space is available in the transmit FIFO and the Request queue. The application can make use of the NPTXFE interrupt in OTG_FS_GINTSTS to find the transmit FIFO space.

b) Bulk/Control IN

```c
Unmask (TXERR/XFRC/BBERR/STALL/DTERR)
if (XFRC)
{
    Reset Error Count
```

STI
RM0090 Rev 18
1341/1749

--- Page 172 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

```c
Unmask CHH
Disable Channel
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
    A typical bulk or control IN pipelined transaction-level operation is shown in Figure 399. See channel 2 (ch_2). The assumptions are:
    - The application is attempting to receive two maximum-packet-size packets (transfer size = 1 024 bytes).
    - The receive FIFO can contain at least one maximum-packet-size packet and two status words per packet (72 bytes for FS).
    - The non-periodic request queue depth = 4.

1342/1749
RM0090 Rev 18
ST

--- Page 173 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Figure 399. Bulk/control IN transactions

Application AHB Host USB Device

1 init_reg(ch_2)
1 init_reg(ch_1)
2 write_tx_fifo (ch_1)
1 MPS
3
4 Non-Periodic Request Queue
Assume that this queue can hold 4 entries.
2 set_ch_en (ch_2)
2 write_tx_fifo (ch_1)
1 MPS
ch_1
ch_2
ch_1
ch_2
OUT
5 set_ch_en (ch_2)
3
DATA0
MPS
ACK
IN
set_ch_en (ch_2)
4
DATA0
5
RXFLVL interrupt
ACK
ch_1
ch_2
ch_2
ch_2
OUT
read_rx_sts
read_rx_fifo
1
MPS
DATA1
MPS
set_ch_en (ch_2)
ACK
7
XFRC interrupt
6
IN
De-allocate
(ch_1)
DATA1
ACK
RXFLVL interrupt
6
read_rx_stsre
ad_rx_fifo
1
MPS
read_rx_sts
7
RXFLVL interrupt
8
ch_2
Disable
(ch_2)
9
XFRC interrupt
RXFLVL interrupt
read_rx_sts
11
CHH interrupt
10
De-allocate
(ch_2)
12
13

ai15675

The sequence of operations is as follows:
a) Initialize channel 2.
b) Set the CHENA bit in HCCHAR2 to write an IN request to the non-periodic request queue.
c) The core attempts to send an IN token after completing the current OUT transaction.
d) The core generates an RXFLVL interrupt as soon as the received packet is written to the receive FIFO.
e) In response to the RXFLVL interrupt, mask the RXFLVL interrupt and read the received packet status to determine the number of bytes received, then read the receive FIFO accordingly. Following this, unmask the RXFLVL interrupt.

ST
RM0090 Rev 18
1343/1749

--- Page 174 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

f) The core generates the RXFLVL interrupt for the transfer completion status entry in the receive FIFO.
g) The application must read and ignore the receive packet status when the receive packet status is not an IN data packet (PKTSTS in GRXSTSR ≠ 0b0010).
h) The core generates the XFRC interrupt as soon as the receive packet status is read.
i) In response to the XFRC interrupt, disable the channel and stop writing the OTG_FS_HCCHAR2 register for further requests. The core writes a channel disable request to the non-periodic request queue as soon as the OTG_FS_HCCHAR2 register is written.
j) The core generates the RXFLVL interrupt as soon as the halt status is written to the receive FIFO.
k) Read and ignore the receive packet status.
l) The core generates a CHH interrupt as soon as the halt status is popped from the receive FIFO.
m) In response to the CHH interrupt, de-allocate the channel for other transfers.
n) Handling non-ACK responses

*   **Control transactions**
    Setup, Data, and Status stages of a control transfer must be performed as three separate transfers. Setup-, Data- or Status-stage OUT transactions are performed similarly to the bulk OUT transactions explained previously. Data- or Status-stage IN transactions are performed similarly to the bulk IN transactions explained previously. For all three stages, the application is expected to set the EPTYP field in OTG_FS_HCCHAR1 to Control. During the Setup stage, the application is expected to set the PID field in OTG_FS_HCTSIZ1 to SETUP.

*   **Interrupt OUT transactions**
    A typical interrupt OUT operation is shown in *Figure 400*. The assumptions are:
    – The application is attempting to send one packet in every frame (up to 1 maximum packet size), starting with the odd frame (transfer size = 1 024 bytes)
    – The periodic transmit FIFO can hold one packet (1 KB)
    – Periodic request queue depth = 4
    The sequence of operations is as follows:
    a) Initialize and enable channel 1. The application must set the ODDFRM bit in OTG_FS_HCCHAR1.
    b) Write the first packet for channel 1.
    c) Along with the last word write of each packet, the OTG_FS host writes an entry to the periodic request queue.
    d) The OTG_FS host attempts to send an OUT token in the next (odd) frame.
    e) The OTG_FS host generates an XFRC interrupt as soon as the last packet is transmitted successfully.
    f) In response to the XFRC interrupt, reinitialize the channel for the next transfer.

1344/1749
RM0090 Rev 18
STI

--- Page 175 ---

RM0090
USB on-the-go full-speed (OTG_FS)

Figure 400. Normal interrupt OUT/IN transactions

ai15676

*   Interrupt service routine for interrupt OUT/IN transactions
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
Mask ACK
Unmask CHH

ST
RM0090 Rev 18
1345/1749

--- Page 176 ---

**USB on-the-go full-speed (OTG_FS)**

RM0090

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

The application uses the NPTXFE interrupt in OTG_FS_GINTSTS to find the transmit FIFO space.

b) Interrupt IN

Unmask (NAK/TXERR/XFRC/BBERR/STALL/FRMOR/DTERR)
if (**XFRC**)
{
Reset Error Count
Mask **ACK**
if (OTG_FS_HCTSIZx.PKTCNT == 0)
{
De-allocate Channel
}
else
{
Transfer Done = 1
Unmask **CHH**
Disable Channel

1346/1749 RM0090 Rev 18

--- Page 177 ---

RM0090
USB on-the-go full-speed (OTG_FS)

```
}
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
}
```

ST
RM0090 Rev 18
1347/1749

--- Page 178 ---

USB on-the-go full-speed (OTG_FS)
RM0090

}

*   **Interrupt IN transactions**
    The assumptions are:
    - The application is attempting to receive one packet (up to 1 maximum packet size) in every frame, starting with odd (transfer size = 1 024 bytes).
    - The receive FIFO can hold at least one maximum-packet-size packet and two status words per packet (1 031 bytes).
    - Periodic request queue depth = 4.
*   **Normal interrupt IN operation**
    The sequence of operations is as follows:
    a) Initialize channel 2. The application must set the ODDFRM bit in OTG_FS_HCCHAR2.
    b) Set the CHENA bit in OTG_FS_HCCHAR2 to write an IN request to the periodic request queue.
    c) The OTG_FS host writes an IN request to the periodic request queue for each OTG_FS_HCCHAR2 register write with the CHENA bit set.
    d) The OTG_FS host attempts to send an IN token in the next (odd) frame.
    e) As soon as the IN packet is received and written to the receive FIFO, the OTG_FS host generates an RXFLVL interrupt.
    f) In response to the RXFLVL interrupt, read the received packet status to determine the number of bytes received, then read the receive FIFO accordingly. The application must mask the RXFLVL interrupt before reading the receive FIFO, and unmask after reading the entire packet.
    g) The core generates the RXFLVL interrupt for the transfer completion status entry in the receive FIFO. The application must read and ignore the receive packet status when the receive packet status is not an IN data packet (PKTSTS in GRXSTSR ≠ 0b0010).
    h) The core generates an XFRC interrupt as soon as the receive packet status is read.
    i) In response to the XFRC interrupt, read the PKTCNT field in OTG_FS_HCTSIZ2. If the PKTCNT bit in OTG_FS_HCTSIZ2 is not equal to 0, disable the channel before re-initializing the channel for the next transfer, if any). If PKTCNT bit in

1348/1749
RM0090 Rev 18

--- Page 179 ---

RM0090
USB on-the-go full-speed (OTG_FS)

OTG_FS_HCTSIZ2 = 0, reinitialize the channel for the next transfer. This time, the application must reset the ODDFRM bit in OTG_FS_HCCHAR2.

*   **Isochronous OUT transactions**
    A typical isochronous OUT operation is shown in Figure 401. The assumptions are:
    - The application is attempting to send one packet every frame (up to 1 maximum packet size), starting with an odd frame. (transfer size = 1 024 bytes).
    - The periodic transmit FIFO can hold one packet (1 KB).
    - Periodic request queue depth = 4.
    The sequence of operations is as follows:
    a) Initialize and enable channel 1. The application must set the ODDFRM bit in OTG_FS_HCCHAR1.
    b) Write the first packet for channel 1.
    c) Along with the last word write of each packet, the OTG_FS host writes an entry to the periodic request queue.
    d) The OTG_FS host attempts to send the OUT token in the next frame (odd).
    e) The OTG_FS host generates the XFRC interrupt as soon as the last packet is transmitted successfully.
    f) In response to the XFRC interrupt, reinitialize the channel for the next transfer.
    g) Handling non-ACK responses

ST
RM0090 Rev 18
1349/1749

--- Page 180 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Figure 401. Normal isochronous OUT/IN transactions

Application AHB Host USB Device

1 init_reg(ch_2)
1 init_reg(ch_1)
1 write_tx_fifo (ch_1)
1 MPS
3 Periodic Request Queue Assume that this queue can hold 4 entries.
2 set_ch_en (ch_2)
2
3 ch_1
4 ch_2
3
OUT
Odd (micro) frame
DATA0 MPS
ACK
IN
6 XFRC interrupt
4
6 init_reg(ch_1)
5
5 write_tx_fifo (ch_1)
1 MPS
DATA0
ACK
read_rx_sts read_rx_fifo
6
RXFLVL interrupt
1 MPS
read_rx_sts
7
RXFLVL interrupt
8 ch_1
ch_2
XFRC interrupt
9 init_reg(ch_2)
set_ch_en (ch_2)
XFRC interrupt
Even (micro) frame
OUT
init_reg(ch_1)
1 MPS
DATA1 MPS
ACK
IN
DATA1

ai15676

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

1350/1749
RM0090 Rev 18
ST

--- Page 181 ---

RM0090
USB on-the-go full-speed (OTG_FS)

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
if (XFRC and (OTG_FS_HCTSIZx.PKTCNT == 0))
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

RM0090 Rev 18
1351/1749

--- Page 182 ---

**USB on-the-go full-speed (OTG_FS)**
RM0090

*   **Isochronous IN transactions**
    The assumptions are:
    - The application is attempting to receive one packet (up to 1 maximum packet size) in every frame starting with the next odd frame (transfer size = 1 024 bytes).
    - The receive FIFO can hold at least one maximum-packet-size packet and two status word per packet (1 031 bytes).
    - Periodic request queue depth = 4.
    The sequence of operations is as follows:
    a) Initialize channel 2. The application must set the ODDFRM bit in OTG_FS_HCCHAR2.
    b) Set the CHENA bit in OTG_FS_HCCHAR2 to write an IN request to the periodic request queue.
    c) The OTG_FS host writes an IN request to the periodic request queue for each OTG_FS_HCCHAR2 register write with the CHENA bit set.
    d) The OTG_FS host attempts to send an IN token in the next odd frame.
    e) As soon as the IN packet is received and written to the receive FIFO, the OTG_FS host generates an RXFLVL interrupt.
    f) In response to the RXFLVL interrupt, read the received packet status to determine the number of bytes received, then read the receive FIFO accordingly. The application must mask the RXFLVL interrupt before reading the receive FIFO, and unmask it after reading the entire packet.
    g) The core generates an RXFLVL interrupt for the transfer completion status entry in the receive FIFO. This time, the application must read and ignore the receive packet status when the receive packet status is not an IN data packet (PKTSTS bit in OTG_FS_GRXSTSR ≠ 0b0010).
    h) The core generates an XFRC interrupt as soon as the receive packet status is read.
    i) In response to the XFRC interrupt, read the PKTCNT field in OTG_FS_HCTSIZ2. If PKTCNT ≠ 0 in OTG_FS_HCTSIZ2, disable the channel before re-initializing the channel for the next transfer, if any. If PKTCNT = 0 in OTG_FS_HCTSIZ2, reinitialize the channel for the next transfer. This time, the application must reset the ODDFRM bit in OTG_FS_HCCHAR2.

*   **Selecting the queue depth**
    Choose the periodic and non-periodic request queue depths carefully to match the number of periodic/non-periodic endpoints accessed.

    The non-periodic request queue depth affects the performance of non-periodic transfers. The deeper the queue (along with sufficient FIFO size), the more often the core is able to pipeline non-periodic transfers. If the queue size is small, the core is able to put in new requests only when the queue space is freed up.

    The core's periodic request queue depth is critical to perform periodic transfers as scheduled. Select the periodic queue depth, based on the number of periodic transfers scheduled in a microframe. If the periodic request queue depth is smaller than the periodic transfers scheduled in a microframe, a frame overrun condition occurs.

*   **Handling babble conditions**
    OTG_FS controller handles two cases of babble: packet babble and port babble. Packet babble occurs if the device sends more data than the maximum packet size for

1352/1749
RM0090 Rev 18

--- Page 183 ---

RM0090
USB on-the-go full-speed (OTG_FS)

the channel. Port babble occurs if the core continues to receive data from the device at EOF2 (the end of frame 2, which is very close to SOF).
When OTG_FS controller detects a packet babble, it stops writing data into the Rx buffer and waits for the end of packet (EOP). When it detects an EOP, it flushes already written data in the Rx buffer and generates a Babble interrupt to the application.
When OTG_FS controller detects a port babble, it flushes the RxFIFO and disables the port. The core then generates a Port disabled interrupt (HPRTINT in OTG_FS_GINTSTS, PENCHG in OTG_FS_HPRT). On receiving this interrupt, the application must determine that this is not due to an overcurrent condition (another cause of the Port Disabled interrupt) by checking POCA in OTG_FS_HPRT, then perform a soft reset. The core does not send any more tokens after it has detected a port babble condition.

