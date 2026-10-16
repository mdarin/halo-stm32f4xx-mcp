### 35.11 OTG_HS interrupts

When the OTG_HS controller is operating in one mode, either peripheral or host, the application must not access registers from the other mode. If an illegal access occurs, a mode mismatch interrupt is generated and reflected in the Core interrupt register (MMIS bit in the OTG_HS_GINTSTS register). When the core switches from one mode to the other, the registers in the new mode of operation must be reprogrammed as they would be after a power-on reset.

*Figure 414 shows the interrupt hierarchy.*

1400/1749 RM0090 Rev 18 ST

--- Page 36 ---

RM0090
USB on-the-go high-speed (OTG_HS)

Figure 414. Interrupt hierarchy

(1)
Wakeup interrupt
OTG_HS_WKUP
OTG_HS
Global interrupt

AND
OR
OTGINT
Global interrupt mask (bit 0)
AHB configuration register
OTG_AHBCFG
Core interrupt mask register
OTG_GINTMSK

OTG_GINTSTS
Core register interrupt
31:26
25
24
23:20
19
18
17:3
2
1:0
HCINT
HPRTINT
OEPINT
IEPINT
OTGINT

OTG_GOTGINT
OTG interrupt register

(15 + #EP):16
OUT endpoints
(#EP-1):0
IN endpoints
17
EP1OUT
EP1IN
1
EP1_IN interrupt OTG_HS_EP1_IN
EP1_OUT interrupt OTG_HS_EP1_OUT

OTG_DAINTMSK
Device all endpoints interrupt mask register
OTG_DAINT
Device all endpoints interrupt register
OTG_DIEPMSK/
OTG_DOEPMSK
Device IN/OUT endpoints common
interrupt mask register
OTG_DIEPINTx/
OTG_DOEPINTx
Device IN/OUT endpoint interrupt
registers
x = 0
x = #HC-1

OTG_HAINTMSK
Host all channels interrupt mask register
OTG_HAINT
Host all channels interrupt register
OTG_HCTINTMSKx
Host channels interrupt mask registers
OTG_HCTINTx
Host channels interrupt registers
x = 0
x = #HC-1

OTG_HPRT
Host port control and status register

OTG_DIEPEACHMSK1
Device each endpoint interrupt
mask registers
OTG_DEACHINT
Device each endpoint interrupt register
OTG_DEACHINTMSK
Device each endpoint interrupt mask
register

1. OTG_HS_WKUP becomes active (high state) when resume condition occurs during L1 SLEEP or L2 SUSPEND states.

MS47472V2

--- Page 37 ---

USB on-the-go high-speed (OTG_HS)
RM0090

