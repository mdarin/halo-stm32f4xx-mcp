**35.13.8 Worst case response time**

When the OTG_HS controller acts as a device, there is a worst case response time for any tokens that follow an isochronous OUT. This worst case response time depends on the AHB clock frequency.

The core registers are in the AHB domain, and the core does not accept another token before updating these register values. The worst case is for any token following an isochronous OUT, because for an isochronous transaction, there is no handshake and the next token could come sooner. This worst case value is 7 PHY clocks when the AHB clock is the same as the PHY clock. When the AHB clock is faster, this value is smaller.

If this worst case condition occurs, the core responds to bulk/interrupt tokens with a NAK and drops isochronous and SETUP tokens. The host interprets this as a timeout condition for SETUP and retries the SETUP packet. For isochronous transfers, the Incomplete isochronous IN transfer interrupt (IISOIXFR) and Incomplete isochronous OUT transfer interrupt (IISOOXFR) inform the application that isochronous IN/OUT packets were dropped.

**Choosing the value of TRDT in OTG_HS_GUSBCFG**

The value in TRDT (OTG_HS_GUSBCFG) is the time it takes for the MAC, in terms of PHY clocks after it has received an IN token, to get the FIFO status, and thus the first data from the PFC block. This time involves the synchronization delay between the PHY and AHB clocks. The worst case delay for this is when the AHB clock is the same as the PHY clock. In this case, the delay is 5 clocks.

1536/1749
RM0090 Rev 18
STI

--- Page 172 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Once the MAC receives an IN token, this information (token received) is synchronized to the AHB clock by the PFC (the PFC runs on the AHB clock). The PFC then reads the data from the SPRAM and writes them into the dual clock source buffer. The MAC then reads the data out of the source buffer (4 deep).

If the AHB is running at a higher frequency than the PHY, the application can use a smaller value for TRDT (in OTG_HS_GUSBCFG).

Figure 429 has the following signals:

*   tkn_rcvd: Token received information from MAC to PFC
*   dynced_tkn_rcvd: Doubled sync tkn_rcvd, from PCLK to HCLK domain
*   spr_read: Read to SPRAM
*   spr_addr: Address to SPRAM
*   spr_rdata: Read data from SPRAM
*   srcbuf_push: Push to the source buffer
*   srcbuf_rdata: Read data from the source buffer. Data seen by MAC

Refer to Table 213: TRDT values for the values of TRDT versus AHB clock frequency.

Figure 429. TRDT max timing case

0ns 50ns 100ns 150ns 200ns
1 2 3 4 5 6 7 8
HCLK
PCLK
tkn_rcvd
dsynced_tkn_rcvd
spr_read
spr_addr A1
spr_rdata D1
srcbuf_push
srcbuf_rdata D1
5 Clocks
ai15680

ST
RM0090 Rev 18
1537/1749

--- Page 173 ---

USB on-the-go high-speed (OTG_HS)
RM0090

