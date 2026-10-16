**35.13.7 Operational model**

**SETUP and OUT data transfers**

This section describes the internal data flow and application-level operations during data OUT transfers and SETUP transactions.

* **Packet read**

This section describes how to read packets (OUT data and SETUP packets) from the receive FIFO in Slave mode.

1. On catching an RXFLVL interrupt (OTG_HS_GINTSTS register), the application must read the Receive status pop register (OTG_HS_GRXSTSP).
2. The application can mask the RXFLVL interrupt (in OTG_HS_GINTSTS) by writing to RXFLVL = 0 (in GINTMSK), until it has read the packet from the receive FIFO.
3. If the received packet's byte count is not 0, the byte count amount of data is popped from the receive Data FIFO and stored in memory. If the received packet byte count is 0, no data is popped from the receive data FIFO.

ST
RM0090 Rev 18
1517/1749

--- Page 153 ---

USB on-the-go high-speed (OTG_HS)
RM0090

4. The receive FIFO packet status readout indicates one of the following:
    a) Global OUT NAK pattern:
    PKTSTS = Global OUT NAK, BCNT = 0x000, EPNUM = Don't Care (0x0),
    DPID = Don't Care (0b00).
    These data indicate that the global OUT NAK bit has taken effect.
    b) SETUP packet pattern:
    PKTSTS = SETUP, BCNT = 0x008, EPNUM = Control EP Num, DPID = D0.
    These data indicate that a SETUP packet for the specified endpoint is now
    available for reading from the receive FIFO.
    c) Setup stage done pattern:
    PKTSTS = Setup Stage Done, BCNT = 0x0, EPNUM = Control EP Num,
    DPID = Don't Care (0b00).
    These data indicate that the Setup stage for the specified endpoint has completed
    and the Data stage has started. After this entry is popped from the receive FIFO,
    the core asserts a Setup interrupt on the specified control OUT endpoint.
    d) Data OUT packet pattern:
    PKTSTS = DataOUT, BCNT = size of the received data OUT packet
    (0 ≤ BCNT ≤ 1 024), EPNUM = EPNUM on which the packet was received,
    DPID = Actual Data PID.
    e) Data transfer completed pattern:
    PKTSTS = Data OUT Transfer Done, BCNT = 0x0, EPNUM = OUT EP Num
    on which the data transfer is complete, DPID = Don't Care (0b00).
    These data indicate that an OUT data transfer for the specified OUT endpoint has
    completed. After this entry is popped from the receive FIFO, the core asserts a
    Transfer Completed interrupt on the specified OUT endpoint.
5. After the data payload is popped from the receive FIFO, the RXFLVL interrupt
    (OTG_HS_GINTSTS) must be unmasked.
6. Steps 1–5 are repeated every time the application detects assertion of the interrupt line
    due to RXFLVL in OTG_HS_GINTSTS. Reading an empty receive FIFO can result in
    undefined core behavior.

Figure 426 provides a flowchart of the above procedure.

1518/1749
RM0090 Rev 18
STI

--- Page 154 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 426. Receive FIFO packet read in slave mode

ai15677

*   **SETUP transactions**
    This section describes how the core handles SETUP packets and the application’s sequence for handling SETUP transactions.

*   **Application requirements**
    1.  To receive a SETUP packet, the STUPCNT field (OTG_HS_DOEPTSIzX) in a control OUT endpoint must be programmed to a nonzero value. When the application programs the STUPCNT field to a nonzero value, the core receives SETUP packets and writes them to the receive FIFO, irrespective of the NAK status and EPENA bit setting in OTG_HS_DOEPCTLx. The STUPCNT field is decremented every time the control endpoint receives a SETUP packet. If the STUPCNT field is not programmed to a proper value before receiving a SETUP packet, the core still receives the SETUP packet and decrements the STUPCNT field, but the application may not be able to determine the correct number of SETUP packets received in the Setup stage of a control transfer.
        *   STUPCNT = 3 in OTG_HS_DOEPTSIzX
    2.  The application must always allocate some extra space in the Receive data FIFO, to be able to receive up to three SETUP packets on a control endpoint.
        *   The space to be reserved is 10 words. Three words are required for the first SETUP packet, 1 word is required for the Setup stage done word and 6 words are required to store two extra SETUP packets among all control endpoints.
        *   3 words per SETUP packet are required to store 8 bytes of SETUP data and 4 bytes of SETUP status (Setup packet pattern). The core reserves this space in the receive data.
        *   FIFO to write SETUP data only, and never uses this space for data packets.
    3.  The application must read the 2 words of the SETUP packet from the receive FIFO.
    4.  The application must read and discard the Setup stage done word from the receive FIFO.

*   **Internal data flow**

STI
RM0090 Rev 18
1519/1749

--- Page 155 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

5. When a SETUP packet is received, the core writes the received data to the receive FIFO, without checking for available space in the receive FIFO and irrespective of the endpoint's NAK and STALL bit settings.
    - The core internally sets the IN NAK and OUT NAK bits for the control IN/OUT endpoints on which the SETUP packet was received.
6. For every SETUP packet received on the USB, 3 words of data are written to the receive FIFO, and the STUPCNT field is decremented by 1.
    - The first word contains control information used internally by the core
    - The second word contains the first 4 bytes of the SETUP command
    - The third word contains the last 4 bytes of the SETUP command
7. When the Setup stage changes to a Data IN/OUT stage, the core writes an entry (Setup stage done word) to the receive FIFO, indicating the completion of the Setup stage.
8. On the AHB side, SETUP packets are emptied by the application.
9. When the application pops the Setup stage done word from the receive FIFO, the core interrupts the application with an STUP interrupt (OTG_HS_DOEPINTx), indicating it can process the received SETUP packet.
    - The core clears the endpoint enable bit for control OUT endpoints.

*   **Application programming sequence**
1. Program the OTG_HS_DOEPTSIx register.
    - STUPCNT = 3
2. Wait for the RXFLVL interrupt (OTG_HS_GINTSTS) and empty the data packets from the receive FIFO.
3. Assertion of the STUP interrupt (OTG_HS_DOEPINTx) marks a successful completion of the SETUP Data Transfer.
    - On this interrupt, the application must read the OTG_HS_DOEPTSIx register to determine the number of SETUP packets received and process the last received SETUP packet.

1520/1749
RM0090 Rev 18

--- Page 156 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 427. Processing a SETUP packet

ai15678

*   **Handling more than three back-to-back SETUP packets**

Per the USB 2.0 specification, normally, during a SETUP packet error, a host does not send more than three back-to-back SETUP packets to the same endpoint. However, the USB 2.0 specification does not limit the number of back-to-back SETUP packets a host can send to the same endpoint. When this condition occurs, the OTG_HS controller generates an interrupt (B2BSTUP in OTG_HS_DOEPINTx).

*   **Setting the global OUT NAK**

Internal data flow:

1.  When the application sets the Global OUT NAK (SGONAK bit in OTG_HS_DCTL), the core stops writing data, except SETUP packets, to the receive FIFO. Irrespective of the space availability in the receive FIFO, nonisochronous OUT tokens receive a NAK handshake response, and the core ignores isochronous OUT data packets
2.  The core writes the Global OUT NAK pattern to the receive FIFO. The application must reserve enough receive FIFO space to write this data pattern.
3.  When the application pops the Global OUT NAK pattern word from the receive FIFO, the core sets the GONAKEFF interrupt (OTG_HS_GINTSTS).
4.  Once the application detects this interrupt, it can assume that the core is in Global OUT NAK mode. The application can clear this interrupt by clearing the SGONAK bit in OTG_HS_DCTL.

ST
RM0090 Rev 18
1521/1749

--- Page 157 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

Application programming sequence:
1. To stop receiving any kind of data in the receive FIFO, the application must set the Global OUT NAK bit by programming the following field:
   - SGONAK = 1 in OTG_HS_DCTL
2. Wait for the assertion of the GONAKEFF interrupt in OTG_HS_GINTSTS. When asserted, this interrupt indicates that the core has stopped receiving any type of data except SETUP packets.
3. The application can receive valid OUT packets after it has set SGONAK in OTG_HS_DCTL and before the core asserts the GONAKEFF interrupt (OTG_HS_GINTSTS).
4. The application can temporarily mask this interrupt by writing to the GINAKEFFM bit in GINTMSK.
   - GINAKEFFM = 0 in GINTMSK
5. Whenever the application is ready to exit the Global OUT NAK mode, it must clear the SGONAK bit in OTG_HS_DCTL. This also clears the GONAKEFF interrupt (OTG_HS_GINTSTS).
   - OTG_HS_DCTL = 1 in CGONAK
6. If the application has masked this interrupt earlier, it must be unmasked as follows:
   - GINAKEFFM = 1 in GINTMSK

*   **Disabling an OUT endpoint**
    The application must use this sequence to disable an OUT endpoint that it has enabled.
    Application programming sequence:
    1. Before disabling any OUT endpoint, the application must enable Global OUT NAK mode in the core.
       - SGONAK = 1 in OTG_HS_DCTL
    2. Wait for the GONAKEFF interrupt (OTG_HS_GINTSTS)
    3. Disable the required OUT endpoint by programming the following fields:
       - EPDIS = 1 in OTG_HS_DOEPCTLx
       - SNAK = 1 in OTG_HS_DOEPCTLx
    4. Wait for the EPDISD interrupt (OTG_HS_DOEPINTx), which indicates that the OUT endpoint is completely disabled. When the EPDISD interrupt is asserted, the core also clears the following bits:
       - EPDIS = 0 in OTG_HS_DOEPCTLx
       - EPENA = 0 in OTG_HS_DOEPCTLx
    5. The application must clear the Global OUT NAK bit to start receiving data from other nondisabled OUT endpoints.
       - SGONAK = 0 in OTG_HS_DCTL

*   **Transfer Stop Programming for OUT endpoints**
    The application must use the following programing sequence to stop any transfers (because of an interrupt from the host, typically a reset).
    Sequence of operations:

1522/1749
RM0090 Rev 18

--- Page 158 ---

RM0090
USB on-the-go high-speed (OTG_HS)

1. Enable all OUT endpoints by setting
    - EPENA = 1 in all OTG_HS_DOEPCTLx registers.
2. Flush the RxFIFO as follows
    - Poll OTG_HS_GRSTCTL.AHBIDL until it is 1. This indicates that AHB master is idle.
    - Perform read modify write operation on OTG_HS_GRSTCTL.RXFFLSH =1
    - Poll OTG_HS_GRSTCTL.RXFFLSH until it is 0, but also using a timeout of less than 10 milli-seconds (corresponds to minimum reset signaling duration). If 0 is seen before the timeout, then the RxFIFO flush is successful. If at the moment the timeout occurs, there is still a 1, (this may be due to a packet on EP0 coming from the host) then go back (once only) to the previous step (“Perform read modify write operation”).
3. Before disabling any OUT endpoint, the application must enable Global OUT NAK mode in the core, according to the instructions in “**Setting the global OUT NAK on page 1521**”. This ensures that data in the RxFIFO is sent to the application successfully. Set SGONAK = 1 in OTG_HS_DCTL
4. Wait for the GONAKEFF interrupt (OTG_HS_GINTSTS)
5. Disable all active OUT endpoints by programming the following register bits:
    - EPDIS = 1 in registers OTG_HS_DOEPCTLx
    - SNAK = 1 in registers OTG_HS_DOEPCTLx
6. Wait for the EPDIS interrupt in OTG_HS_DOEPINTx for each OUT endpoint programmed in the previous step. The EPDIS interrupt in OTG_HS_DOEPINTx indicates that the corresponding OUT endpoint is completely disabled. When the EPDIS interrupt is asserted, the following bits are cleared:
    - EPENA = 0 in registers OTG_HS_DOEPCTLx
    - EPDIS = 0 in registers OTG_HS_DOEPCTLx
    - SNAK = 0 in registers OTG_HS_DOEPCTLx

*   **Generic non-isochronous OUT data transfers**

This section describes a regular nonisochronous OUT data transfer (control, bulk, or interrupt).

Application requirements:
1. Before setting up an OUT transfer, the application must allocate a buffer in the memory to accommodate all data to be received as part of the OUT transfer.
2. For OUT transfers, the transfer size field in the endpoint’s transfer size register must be a multiple of the maximum packet size of the endpoint, adjusted to the word boundary.
    - transfer size[EPNUM] = n × (MPSIZ[EPNUM] + 4 – (MPSIZ[EPNUM] mod 4))
    - packet count[EPNUM] = n
    - n > 0
3. On any OUT endpoint interrupt, the application must read the endpoint’s transfer size register to calculate the size of the payload in the memory. The received payload size can be less than the programmed transfer size.
    - Payload size in memory = application programmed initial transfer size – core updated final transfer size
    - Number of USB packets in which this payload was received = application programmed initial packet count – core updated final packet count

ST
RM0090 Rev 18
1523/1749

--- Page 159 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

Internal data flow:

1. The application must set the transfer size and packet count fields in the endpoint-specific registers, clear the NAK bit, and enable the endpoint to receive the data.
2. Once the NAK bit is cleared, the core starts receiving data and writes it to the receive FIFO, as long as there is space in the receive FIFO. For every data packet received on the USB, the data packet and its status are written to the receive FIFO. Every packet (maximum packet size or short packet) written to the receive FIFO decrements the packet count field for that endpoint by 1.
    - OUT data packets received with bad data CRC are flushed from the receive FIFO automatically.
    - After sending an ACK for the packet on the USB, the core discards nonisochronous OUT data packets that the host, which cannot detect the ACK, resends. The application does not detect multiple back-to-back data OUT packets on the same endpoint with the same data PID. In this case the packet count is not decremented.
    - If there is no space in the receive FIFO, isochronous or nonisochronous data packets are ignored and not written to the receive FIFO. Additionally, nonisochronous OUT tokens receive a NAK handshake reply.
    - In all the above three cases, the packet count is not decremented because no data are written to the receive FIFO.
3. When the packet count becomes 0 or when a short packet is received on the endpoint, the NAK bit for that endpoint is set. Once the NAK bit is set, the isochronous or nonisochronous data packets are ignored and not written to the receive FIFO, and nonisochronous OUT tokens receive a NAK handshake reply.
4. After the data are written to the receive FIFO, the application reads the data from the receive FIFO and writes it to external memory, one packet at a time per endpoint.
5. At the end of every packet write on the AHB to external memory, the transfer size for the endpoint is decremented by the size of the written packet.
6. The OUT data transfer completed pattern for an OUT endpoint is written to the receive FIFO on one of the following conditions:
    - The transfer size is 0 and the packet count is 0
    - The last OUT data packet written to the receive FIFO is a short packet (0 ≤ packet size < maximum packet size)
7. When either the application pops this entry (OUT data transfer completed), a transfer completed interrupt is generated for the endpoint and the endpoint enable is cleared.

Application programming sequence:

1524/1749
RM0090 Rev 18

--- Page 160 ---

RM0090
USB on-the-go high-speed (OTG_HS)

1. Program the OTG_HS_DOEPTSIx register for the transfer size and the corresponding packet count.
2. Program the OTG_HS_DOEPCTLx register with the endpoint characteristics, and set the EPENA and CNAK bits.
    - EPENA = 1 in OTG_HS_DOEPCTLx
    - CNAK = 1 in OTG_HS_DOEPCTLx
3. Wait for the RXFLVL interrupt (in OTG_HS_GINTSTS) and empty the data packets from the receive FIFO.
    - This step can be repeated many times, depending on the transfer size.
4. Asserting the XFRC interrupt (OTG_HS_DOEPINTx) marks a successful completion of the nonisochronous OUT data transfer.
5. Read the OTG_HS_DOEPTSIx register to determine the size of the received data payload.
* Generic isochronous OUT data transfer

This section describes a regular isochronous OUT data transfer.

Application requirements:
1. All the application requirements for nonisochronous OUT data transfers also apply to isochronous OUT data transfers.
2. For isochronous OUT data transfers, the transfer size and packet count fields must always be set to the number of maximum-packet-size packets that can be received in a single frame and no more. Isochronous OUT data transfers cannot span more than 1 frame.
3. The application must read all isochronous OUT data packets from the receive FIFO (data and status) before the end of the periodic frame (EOPF interrupt in OTG_HS_GINTSTS).
4. To receive data in the following frame, an isochronous OUT endpoint must be enabled after the EOPF (OTG_HS_GINTSTS) and before the SOF (OTG_HS_GINTSTS).

Internal data flow:
1. The internal data flow for isochronous OUT endpoints is the same as that for nonisochronous OUT endpoints, but for a few differences.
2. When an isochronous OUT endpoint is enabled by setting the Endpoint Enable and clearing the NAK bits, the Even/Odd frame bit must also be set appropriately. The core receives data on an isochronous OUT endpoint in a particular frame only if the following condition is met:
    - EONUM (in OTG_HS_DOEPCTLx) = SOFFN[0] (in OTG_HS_DSTS)
3. When the application completely reads an isochronous OUT data packet (data and status) from the receive FIFO, the core updates the RXDPID field in OTG_HS_DOEPTSIx with the data PID of the last isochronous OUT data packet read from the receive FIFO.

Application programming sequence:

ST
RM0090 Rev 18
1525/1749

--- Page 161 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

1. Program the OTG_HS_DOEPTSIz register for the transfer size and the corresponding packet count
2. Program the OTG_HS_DOEPCTLx register with the endpoint characteristics and set the Endpoint Enable, ClearNAK, and Even/Odd frame bits.
    - EPENA = 1
    - CNAK = 1
    - EONUM = (0: Even/1: Odd)
3. In Slave mode, wait for the RXFLVL interrupt (in OTG_HS_GINTSTS) and empty the data packets from the receive FIFO
    - This step can be repeated many times, depending on the transfer size.
4. The assertion of the XFRC interrupt (in OTG_HS_DOEPINTx) marks the completion of the isochronous OUT data transfer. This interrupt does not necessarily mean that the data in memory are good.
5. This interrupt cannot always be detected for isochronous OUT transfers. Instead, the application can detect the IISOOXFRM interrupt in OTG_HS_GINTSTS.
6. Read the OTG_HS_DOEPTSIz register to determine the size of the received transfer and to determine the validity of the data received in the frame. The application must treat the data received in memory as valid only if one of the following conditions is met:
    - RXDPID = D0 (in OTG_HS_DOEPTSIz) and the number of USB packets in which this payload was received = 1
    - RXDPID = D1 (in OTG_HS_DOEPTSIz) and the number of USB packets in which this payload was received = 2
    - RXDPID = D2 (in OTG_HS_DOEPTSIz) and the number of USB packets in which this payload was received = 3
    The number of USB packets in which this payload was received = Application programmed initial packet count – Core updated final packet count
    The application can discard invalid data packets.

*   **Incomplete isochronous OUT data transfers**

This section describes the application programming sequence when isochronous OUT data packets are dropped inside the core.

Internal data flow:
1. For isochronous OUT endpoints, the XFRC interrupt (in OTG_HS_DOEPINTx) may not always be asserted. If the core drops isochronous OUT data packets, the application could fail to detect the XFRC interrupt (OTG_HS_DOEPINTx) under the following circumstances:
    - When the receive FIFO cannot accommodate the complete ISO OUT data packet, the core drops the received ISO OUT data
    - When the isochronous OUT data packet is received with CRC errors
    - When the isochronous OUT token received by the core is corrupted
    - When the application is very slow in reading the data from the receive FIFO
2. When the core detects an end of periodic frame before transfer completion to all isochronous OUT endpoints, it asserts the incomplete Isochronous OUT data interrupt (IISOOXFRM in OTG_HS_GINTSTS), indicating that an XFRC interrupt (in OTG_HS_DOEPINTx) is not asserted on at least one of the isochronous OUT endpoints. At this point, the endpoint with the incomplete transfer remains enabled, but no active transfers remain in progress on this endpoint on the USB.

1526/1749
RM0090 Rev 18
ST

--- Page 162 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Application programming sequence:

1. Asserting the IISOOXXFRM interrupt (OTG_HS_GINTSTS) indicates that in the current frame, at least one isochronous OUT endpoint has an incomplete transfer.
2. If this occurs because isochronous OUT data is not completely emptied from the endpoint, the application must ensure that the application empties all isochronous OUT data (data and status) from the receive FIFO before proceeding.
    - When all data are emptied from the receive FIFO, the application can detect the XFRC interrupt (OTG_HS_DOEPINTx). In this case, the application must re-enable the endpoint to receive isochronous OUT data in the next frame.
3. When it receives an IISOOXXFRM interrupt (in OTG_HS_GINTSTS), the application must read the control registers of all isochronous OUT endpoints (OTG_HS_DOEPCTLx) to determine which endpoints had an incomplete transfer in the current micro-frame. An endpoint transfer is incomplete if both the following conditions are met:
    - EONUM bit (in OTG_HS_DOEPCTLx) = SOFFN[0] (in OTG_HS_DSTS)
    - EPENA = 1 (in OTG_HS_DOEPCTLx)
4. The previous step must be performed before the SOF interrupt (in OTG_HS_GINTSTS) is detected, to ensure that the current frame number is not changed.
5. For isochronous OUT endpoints with incomplete transfers, the application must discard the data in the memory and disable the endpoint by setting the EPDIS bit in OTG_HS_DOEPCTLx.
6. Wait for the EPDIS interrupt (in OTG_HS_DOEPINTx) and enable the endpoint to receive new data in the next frame.
    - Because the core can take some time to disable the endpoint, the application may not be able to receive the data in the next frame after receiving bad isochronous data.

*   **Stalling a nonisochronous OUT endpoint**

This section describes how the application can stall a nonisochronous endpoint.

1. Put the core in the Global OUT NAK mode.
2. Disable the required endpoint
    - When disabling the endpoint, instead of setting the SNAK bit in OTG_HS_DOEPCTL, set STALL = 1 (in OTG_HS_DOEPCTL).
    The STALL bit always takes precedence over the NAK bit.
3. When the application is ready to end the STALL handshake for the endpoint, the STALL bit (in OTG_HS_DOEPCTLx) must be cleared.
4. If the application is setting or clearing a STALL for an endpoint due to a SetFeature.Endpoint Halt or ClearFeature.Endpoint Halt command, the STALL bit must be set or cleared before the application sets up the Status stage transfer on the control endpoint.

**Examples**

This section describes and depicts some fundamental transfer types and scenarios.

*   Slave mode bulk OUT transaction

*Figure 428* depicts the reception of a single Bulk OUT Data packet from the USB to the AHB and describes the events involved in the process.

ST
RM0090 Rev 18
1527/1749

--- Page 163 ---

USB on-the-go high-speed (OTG_HS)
RM0090

Figure 428. Slave mode bulk OUT transaction

ai15679

After a SetConfiguration/SetInterface command, the application initializes all OUT endpoints by setting CNAK = 1 and EPENA = 1 (in OTG_HS_DOEPCTLx), and setting a suitable XFRSIZ and PKTCNT in the OTG_HS_DOEPTSIZx register.

1. Host attempts to send data (OUT token) to an endpoint.
2. When the core receives the OUT token on the USB, it stores the packet in the RxFIFO because space is available there.
3. After writing the complete packet in the RxFIFO, the core then asserts the RXFLVL interrupt (in OTG_HS_GINTSTS).
4. On receiving the PKTCNT number of USB packets, the core internally sets the NAK bit for this endpoint to prevent it from receiving any more packets.
5. The application processes the interrupt and reads the data from the RxFIFO.
6. When the application has read all the data (equivalent to XFRSIZ), the core generates an XFRC interrupt (in OTG_HS_DOEPINTx).
7. The application processes the interrupt and uses the setting of the XFRC interrupt bit (in OTG_HS_DOEPINTx) to determine that the intended transfer is complete.

**IN data transfers**

* **Packet write**

This section describes how the application writes data packets to the endpoint FIFO in Slave mode when dedicated transmit FIFOs are enabled.

1528/1749
RM0090 Rev 18

--- Page 164 ---

RM0090
USB on-the-go high-speed (OTG_HS)

1. The application can either choose the polling or the interrupt mode.
    - In polling mode, the application monitors the status of the endpoint transmit data FIFO by reading the OTG_HS_DTXFSTSx register, to determine if there is enough space in the data FIFO.
    - In interrupt mode, the application waits for the TXFE interrupt (in OTG_HS_DIEPINTx) and then reads the OTG_HS_DTXFSTSx register, to determine if there is enough space in the data FIFO.
    - To write a single nonzero length data packet, there must be space to write the entire packet in the data FIFO.
    - To write zero length packet, the application must not look at the FIFO space.
2. Using one of the above mentioned methods, when the application determines that there is enough space to write a transmit packet, the application must first write into the endpoint control register, before writing the data into the data FIFO. Typically, the application, must do a read modify write on the OTG_HS_DIEPCTLx register to avoid modifying the contents of the register, except for setting the Endpoint Enable bit.

The application can write multiple packets for the same endpoint into the transmit FIFO, if space is available. For periodic IN endpoints, the application must write packets only for one micro-frame. It can write packets for the next periodic transaction only after getting transfer complete for the previous transaction.

*   **Setting IN endpoint NAK**

Internal data flow:
1. When the application sets the IN NAK for a particular endpoint, the core stops transmitting data on the endpoint, irrespective of data availability in the endpoint's transmit FIFO.
2. Nonisochronous IN tokens receive a NAK handshake reply
    - Isochronous IN tokens receive a zero-data-length packet reply
3. The core asserts the INEPNE (IN endpoint NAK effective) interrupt in OTG_HS_DIEPINTx in response to the SNAK bit in OTG_HS_DIEPCTLx.
4. Once this interrupt is seen by the application, the application can assume that the endpoint is in IN NAK mode. This interrupt can be cleared by the application by setting the CNAK bit in OTG_HS_DIEPCTLx.

Application programming sequence:

ST
RM0090 Rev 18
1529/1749

--- Page 165 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

1. To stop transmitting any data on a particular IN endpoint, the application must set the IN NAK bit. To set this bit, the following field must be programmed.
   - SNAK = 1 in OTG_HS_DIEPCTLx
2. Wait for assertion of the INEPNE interrupt in OTG_HS_DIEPINTx. This interrupt indicates that the core has stopped transmitting data on the endpoint.
3. The core can transmit valid IN data on the endpoint after the application has set the NAK bit, but before the assertion of the NAK Effective interrupt.
4. The application can mask this interrupt temporarily by writing to the INEPNEM bit in DIEPMSK.
   - INEPNEM = 0 in DIEPMSK
5. To exit Endpoint NAK mode, the application must clear the NAK status bit (NAKSTS) in OTG_HS_DIEPCTLx. This also clears the INEPNE interrupt (in OTG_HS_DIEPINTx).
   - CNAK = 1 in OTG_HS_DIEPCTLx
6. If the application masked this interrupt earlier, it must be unmasked as follows:
   - INEPNEM = 1 in DIEPMSK

*   **IN endpoint disable**

Use the following sequence to disable a specific IN endpoint that has been previously enabled.

Application programming sequence:
1. The application must stop writing data on the AHB for the IN endpoint to be disabled.
2. The application must set the endpoint in NAK mode.
   - SNAK = 1 in OTG_HS_DIEPCTLx
3. Wait for the INEPNE interrupt in OTG_HS_DIEPINTx.
4. Set the following bits in the OTG_HS_DIEPCTLx register for the endpoint that must be disabled.
   - EPDIS = 1 in OTG_HS_DIEPCTLx
   - SNAK = 1 in OTG_HS_DIEPCTLx
5. Assertion of the EPDISD interrupt in OTG_HS_DIEPINTx indicates that the core has completely disabled the specified endpoint. Along with the assertion of the interrupt, the core also clears the following bits:
   - EPENA = 0 in OTG_HS_DIEPCTLx
   - EPDIS = 0 in OTG_HS_DIEPCTLx
6. The application must read the OTG_HS_DIEPTSIZx register for the periodic IN EP, to calculate how much data on the endpoint were transmitted on the USB.
7. The application must flush the data in the Endpoint transmit FIFO, by setting the following fields in the OTG_HS_GRSTCTL register:
   - TXFNUM (in OTG_HS_GRSTCTL) = Endpoint transmit FIFO number
   - TXFFLSH in (OTG_HS_GRSTCTL) = 1

The application must poll the OTG_HS_GRSTCTL register, until the TXFFLSH bit is cleared by the core, which indicates the end of flush operation. To transmit new data on this endpoint, the application can re-enable the endpoint at a later point.

*   **Transfer Stop Programming for IN endpoints**

The application must use the following programing sequence to stop any transfers (because of an interrupt from the host, typically a reset).

1530/1749
RM0090 Rev 18

--- Page 166 ---

RM0090
USB on-the-go high-speed (OTG_HS)

**Sequence of operations:**

1. Disable the IN endpoint by setting:
   - EPDIS = 1 in all OTG_HS_DIEPCTLx registers
2. Wait for the EPDIS interrupt in OTG_HS_DIEPINTx, which indicates that the IN endpoint is completely disabled. When the EPDIS interrupt is asserted the following bits are cleared:
   - EPDIS = 0 in OTG_HS_DIEPCTLx
   - EPENA = 0 in OTG_HS_DIEPCTLx
3. Flush the TxFIFO by programming the following bits:
   - TXFFLSH = 1 in OTG_HS_GRSTCTL
   - TXFNUM = "FIFO number specific to endpoint" in OTG_HS_GRSTCTL
4. The application can start polling till TXFFLSH in OTG_HS_GRSTCTL is cleared. When this bit is cleared, it ensures that there is no data left in the Tx FIFO.

*   **Generic nonperiodic IN data transfers**

Application requirements:

1. Before setting up an IN transfer, the application must ensure that all data to be transmitted as part of the IN transfer are part of a single buffer.
2. For IN transfers, the Transfer Size field in the Endpoint Transfer Size register denotes a payload that constitutes multiple maximum-packet-size packets and a single short packet. This short packet is transmitted at the end of the transfer.
   - To transmit a few maximum-packet-size packets and a short packet at the end of the transfer:
     Transfer size[EPNUM] = x × MPSIZ[EPNUM] + sp
     If (sp > 0), then packet count[EPNUM] = x + 1.
     Otherwise, packet count[EPNUM] = x
   - To transmit a single zero-length data packet:
     Transfer size[EPNUM] = 0
     Packet count[EPNUM] = 1
   - To transmit a few maximum-packet-size packets and a zero-length data packet at the end of the transfer, the application must split the transfer into two parts. The first sends maximum-packet-size data packets and the second sends the zero-length data packet alone.
     First transfer: transfer size[EPNUM] = x × MPSIZ[epnum]; packet count = n;
     Second transfer: transfer size[EPNUM] = 0; packet count = 1;
3. Once an endpoint is enabled for data transfers, the core updates the Transfer size register. At the end of the IN transfer, the application must read the Transfer size register to determine how much data posted in the transmit FIFO have already been sent on the USB.
4. Data fetched into transmit FIFO = Application-programmed initial transfer size – core-updated final transfer size
   - Data transmitted on USB = (application-programmed initial packet count – Core updated final packet count) × MPSIZ[EPNUM]
   - Data yet to be transmitted on USB = (Application-programmed initial transfer size – data transmitted on USB)

Internal data flow:

STI
RM0090 Rev 18
1531/1749

--- Page 167 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

1. The application must set the transfer size and packet count fields in the endpoint-specific registers and enable the endpoint to transmit the data.
2. The application must also write the required data to the transmit FIFO for the endpoint.
3. Every time a packet is written into the transmit FIFO by the application, the transfer size for that endpoint is decremented by the packet size. The data is fetched from the memory by the application, until the transfer size for the endpoint becomes 0. After writing the data into the FIFO, the “number of packets in FIFO” count is incremented (this is a 3-bit count, internally maintained by the core for each IN endpoint transmit FIFO. The maximum number of packets maintained by the core at any time in an IN endpoint FIFO is eight). For zero-length packets, a separate flag is set for each FIFO, without any data in the FIFO.
4. Once the data are written to the transmit FIFO, the core reads them out upon receiving an IN token. For every nonisochronous IN data packet transmitted with an ACK handshake, the packet count for the endpoint is decremented by one, until the packet count is zero. The packet count is not decremented on a timeout.
5. For zero length packets (indicated by an internal zero length flag), the core sends out a zero-length packet for the IN token and decrements the packet count field.
6. If there are no data in the FIFO for a received IN token and the packet count field for that endpoint is zero, the core generates an “IN token received when TxFIFO is empty” (ITTXFE) Interrupt for the endpoint, provided that the endpoint NAK bit is not set. The core responds with a NAK handshake for nonisochronous endpoints on the USB.
7. The core internally rewinds the FIFO pointers and no timeout interrupt is generated.
8. When the transfer size is 0 and the packet count is 0, the transfer complete (XFRC) interrupt for the endpoint is generated and the endpoint enable is cleared.

**Application programming sequence:**

1. Program the OTG_HS_DIEPTSIx register with the transfer size and corresponding packet count.
2. Program the OTG_HS_DIEPCTLx register with the endpoint characteristics and set the CNAK and EPENA (Endpoint Enable) bits.
3. When transmitting nonzero length data packet, the application must poll the OTG_HS_DTXFSTSx register (where x is the FIFO number associated with that endpoint) to determine whether there is enough space in the data FIFO. The application can optionally use TXFE (in OTG_HS_DIEPINTx) before writing the data.

*   **Generic periodic IN data transfers**

This section describes a typical periodic IN data transfer.

**Application requirements:**

1. Application requirements 1, 2, 3, and 4 of *Generic nonperiodic IN data transfers on page 1531* also apply to periodic IN data transfers, except for a slight modification of requirement 2.
    – The application can only transmit multiples of maximum-packet-size data packets or multiples of maximum-packet-size packets, plus a short packet at the end. To

1532/1749
RM0090 Rev 18
STI

--- Page 168 ---

RM0090
USB on-the-go high-speed (OTG_HS)

transmit a few maximum-packet-size packets and a short packet at the end of the transfer, the following conditions must be met:
transfer size[EPNUM] = x × MPSIZ[EPNUM] + sp
(where x is an integer ≥ 0, and 0 ≤ sp < MPSIZ[EPNUM])
If (sp > 0), packet count[EPNUM] = x + 1
Otherwise, packet count[EPNUM] = x;
MCNT[EPNUM] = packet count[EPNUM]

- The application cannot transmit a zero-length data packet at the end of a transfer. It can transmit a single zero-length data packet by itself. To transmit a single zero-length data packet:
- transfer size[EPNUM] = 0
packet count[EPNUM] = 1
MCNT[EPNUM] = packet count[EPNUM]

2. The application can only schedule data transfers one frame at a time.
- (MCNT − 1) × MPSIZ ≤ XFERSIZ ≤ MCNT × MPSIZ
- PKTCNT = MCNT (in OTG_HS_DIEPTSIZx)
- If XFERSIZ < MCNT × MPSIZ, the last data packet of the transfer is a short packet.
- Note that: MCNT is in OTG_HS_DIEPTSIZx, MPSIZ is in OTG_HS_DIEPCTLx, PKTCNT is in OTG_HS_DIEPTSIZx and XFERSIZ is in OTG_HS_DIEPTSIZx

3. The complete data to be transmitted in the frame must be written into the transmit FIFO by the application, before the IN token is received. Even when 1 word of the data to be transmitted per frame is missing in the transmit FIFO when the IN token is received, the core behaves as when the FIFO is empty. When the transmit FIFO is empty:
- A zero data length packet would be transmitted on the USB for isochronous IN endpoints
- A NAK handshake would be transmitted on the USB for interrupt IN endpoints

4. For a high-bandwidth IN endpoint with three packets in a frame, the application can program the endpoint FIFO size to be 2 × max_pkt_size and have the third packet loaded in after the first packet has been transmitted on the USB.

Internal data flow:
1. The application must set the transfer size and packet count fields in the endpoint-specific registers and enable the endpoint to transmit the data.
2. The application must also write the required data to the associated transmit FIFO for the endpoint.
3. Every time the application writes a packet to the transmit FIFO, the transfer size for that endpoint is decremented by the packet size. The data are fetched from application memory until the transfer size for the endpoint becomes 0.
4. When an IN token is received for a periodic endpoint, the core transmits the data in the FIFO, if available. If the complete data payload (complete packet, in dedicated FIFO

ST
RM0090 Rev 18
1533/1749

--- Page 169 ---

**USB on-the-go high-speed (OTG_HS)**

RM0090

mode) for the frame is not present in the FIFO, then the core generates an IN token received when TxFIFO empty interrupt for the endpoint.
- A zero-length data packet is transmitted on the USB for isochronous IN endpoints
- A NAK handshake is transmitted on the USB for interrupt IN endpoints
5. The packet count for the endpoint is decremented by 1 under the following conditions:
- For isochronous endpoints, when a zero- or nonzero-length data packet is transmitted
- For interrupt endpoints, when an ACK handshake is transmitted
- When the transfer size and packet count are both 0, the transfer completed interrupt for the endpoint is generated and the endpoint enable is cleared.
6. At the “Periodic frame Interval” (controlled by PFIVL in OTG_HS_DCFG), when the core finds nonempty any of the isochronous IN endpoint FIFOs scheduled for the current frame nonempty, the core generates an IISOIXFR interrupt in OTG_HS_GINTSTS.

Application programming sequence:
1. Program the OTG_HS_DIEPCTLx register with the endpoint characteristics and set the CNAK and EPENA bits.
2. Write the data to be transmitted in the next frame to the transmit FIFO.
3. Asserting the ITTXFE interrupt (in OTG_HS_DIEPINTx) indicates that the application has not yet written all data to be transmitted to the transmit FIFO.
4. If the interrupt endpoint is already enabled when this interrupt is detected, ignore the interrupt. If it is not enabled, enable the endpoint so that the data can be transmitted on the next IN token attempt.
5. Asserting the XFRC interrupt (in OTG_HS_DIEPINTx) with no ITTXFE interrupt in OTG_HS_DIEPINTx indicates the successful completion of an isochronous IN transfer. A read to the OTG_HS_DIEPTSIZx register must give transfer size = 0 and packet count = 0, indicating all data were transmitted on the USB.
6. Asserting the XFRC interrupt (in OTG_HS_DIEPINTx), with or without the ITTXFE interrupt (in OTG_HS_DIEPINTx), indicates the successful completion of an interrupt IN transfer. A read to the OTG_HS_DIEPTSIZx register must give transfer size = 0 and packet count = 0, indicating all data were transmitted on the USB.
7. Asserting the incomplete isochronous IN transfer (IISOIXFR) interrupt in OTG_HS_GINTSTS with none of the aforementioned interrupts indicates the core did not receive at least 1 periodic IN token in the current frame.

*   **Incomplete isochronous IN data transfers**

This section describes what the application must do on an incomplete isochronous IN data transfer.

Internal data flow:
1. An isochronous IN transfer is treated as incomplete in one of the following conditions:
a) The core receives a corrupted isochronous IN token on at least one isochronous IN endpoint. In this case, the application detects an incomplete isochronous IN transfer interrupt (IISOIXFR in OTG_HS_GINTSTS).
b) The application is slow to write the complete data payload to the transmit FIFO and an IN token is received before the complete data payload is written to the FIFO. In this case, the application detects an IN token received when TxFIFO empty interrupt in OTG_HS_DIEPINTx. The application can ignore this interrupt,

1534/1749 RM0090 Rev 18 ST

--- Page 170 ---

RM0090
USB on-the-go high-speed (OTG_HS)

as it eventually results in an incomplete isochronous IN transfer interrupt
(IISOIXFR in OTG_HS_GINTSTS) at the end of periodic frame.
The core transmits a zero-length data packet on the USB in response to the
received IN token.

2. The application must stop writing the data payload to the transmit FIFO as soon as
possible.
3. The application must set the NAK bit and the disable bit for the endpoint.
4. The core disables the endpoint, clears the disable bit, and asserts the Endpoint Disable
interrupt for the endpoint.

Application programming sequence

1. The application can ignore the IN token received when TxFIFO empty interrupt in
OTG_HS_DIEPINTx on any isochronous IN endpoint, as it eventually results in an
incomplete isochronous IN transfer interrupt (in OTG_HS_GINTSTS).
2. Assertion of the incomplete isochronous IN transfer interrupt (in OTG_HS_GINTSTS)
indicates an incomplete isochronous IN transfer on at least one of the isochronous IN
endpoints.
3. The application must read the Endpoint Control register for all isochronous IN
endpoints to detect endpoints with incomplete IN data transfers.
4. The application must stop writing data to the Periodic Transmit FIFOs associated with
these endpoints on the AHB.
5. Program the following fields in the OTG_HS_DIEPCTLx register to disable the
endpoint:
- SNAK = 1 in OTG_HS_DIEPCTLx
- EPDIS = 1 in OTG_HS_DIEPCTLx
6. The assertion of the Endpoint Disabled interrupt in OTG_HS_DIEPINTx indicates that
the core has disabled the endpoint.
- At this point, the application must flush the data in the associated transmit FIFO or
overwrite the existing data in the FIFO by enabling the endpoint for a new transfer
in the next micro-frame. To flush the data, the application must use the
OTG_HS_GRSTCTL register.

* **Stalling nonisochronous IN endpoints**

This section describes how the application can stall a nonisochronous endpoint.
Application programming sequence:

STI
RM0090 Rev 18
1535/1749

--- Page 171 ---

**USB on-the-go high-speed (OTG_HS)**
RM0090

1. Disable the IN endpoint to be stalled. Set the STALL bit as well.
2. EPDIS = 1 in OTG_HS_DIEPCTLx, when the endpoint is already enabled
   - STALL = 1 in OTG_HS_DIEPCTLx
   - The STALL bit always takes precedence over the NAK bit
3. Assertion of the Endpoint Disabled interrupt (in OTG_HS_DIEPINTx) indicates to the application that the core has disabled the specified endpoint.
4. The application must flush the nonperiodic or periodic transmit FIFO, depending on the endpoint type. In case of a nonperiodic endpoint, the application must re-enable the other nonperiodic endpoints that do not need to be stalled, to transmit data.
5. Whenever the application is ready to end the STALL handshake for the endpoint, the STALL bit must be cleared in OTG_HS_DIEPCTLx.
6. If the application sets or clears a STALL bit for an endpoint due to a SetFeature.Endpoint Halt command or ClearFeature.Endpoint Halt command, the STALL bit must be set or cleared before the application sets up the Status stage transfer on the control endpoint.

Special case: stalling the control OUT endpoint

The core must stall IN/OUT tokens if, during the data stage of a control transfer, the host sends more IN/OUT tokens than are specified in the SETUP packet. In this case, the application must enable the ITTXFE interrupt in OTG_HS_DIEPINTx and the OTEPDIS interrupt in OTG_HS_DOEPINTx during the data stage of the control transfer, after the core has transferred the amount of data specified in the SETUP packet. Then, when the application receives this interrupt, it must set the STALL bit in the corresponding endpoint control register, and clear this interrupt.

