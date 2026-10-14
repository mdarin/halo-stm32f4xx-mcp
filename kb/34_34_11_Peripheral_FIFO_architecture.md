### 34.11 Peripheral FIFO architecture

Figure 392. Device-mode FIFO address mapping and AHB FIFO access mapping

Single data FIFO

IN endpoint Tx FIFO #n
DFIFO push access from AHB
Dedicated Tx FIFO #n control (optional)
MAC pop
Tx FIFO #n packet
DIEPTXF2[31:16]
DIEPTXFx[15:0]
...
DIEPTXF2[15:0]

IN endpoint Tx FIFO #1
DFIFO push access from AHB
Dedicated Tx FIFO #1 control (optional)
MAC pop
Tx FIFO #1 packet
DIEPTXF1[31:16]
DIEPTXF1[15:0]

IN endpoint Tx FIFO #0
DFIFO push access from AHB
Dedicated Tx FIFO #0 control (optional)
MAC pop
Tx FIFO #0 packet
GNPTXFSIZ[31:16]
GNPTXFSIZ[15:0]

Any OUT endpoint DFIFO pop access from AHB
Rx FIFO control
MAC push
Rx packets
GRXFSIZ[31:16]

(Rx start address fixed to 0)
A1 = 0

ai15611

### 34.11.1 Peripheral Rx FIFO

The OTG peripheral uses a single receive FIFO that receives the data directed to all OUT endpoints. Received packets are stacked back-to-back until free space is available in the Rx-FIFO. The status of the received packet (which contains the OUT endpoint destination number, the byte count, the data PID and the validity of the received data) is also stored by the core on top of the data payload. When no more space is available, host transactions are NACKed and an interrupt is received on the addressed endpoint. The size of the receive FIFO is configured in the receive FIFO Size register (GRXFSIZ).

The single receive FIFO architecture makes it more efficient for the USB peripheral to fill in the receive RAM buffer:

*   All OUT endpoints share the same RAM buffer (shared FIFO)
*   The OTG FS core can fill in the receive FIFO up to the limit for any host sequence of OUT tokens

The application keeps receiving the Rx-FIFO non-empty interrupt (RXFLVL bit in OTG_FS_GINTSTS) as long as there is at least one packet available for download. It reads the packet information from the receive status read and pop register (GRXSTSP) and finally pops data off the receive FIFO by reading from the endpoint-related pop address.

ST
RM0090 Rev 18
1259/1749

--- Page 90 ---

USB on-the-go full-speed (OTG_FS)
RM0090

### 34.11.2 Peripheral Tx FIFOs

The core has a dedicated FIFO for each IN endpoint. The application configures FIFO sizes by writing the non periodic transmit FIFO size register (OTG_FS_TX0FSIZ) for IN endpoint0 and the device IN endpoint transmit FIFOx registers (DIEPTXFx) for IN endpoint-x.

