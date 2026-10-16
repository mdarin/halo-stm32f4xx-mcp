### 34.15 OTG_FS interrupts

When the OTG_FS controller is operating in one mode, either device or host, the application must not access registers from the other mode. If an illegal access occurs, a mode mismatch interrupt is generated and reflected in the Core interrupt register (MMIS bit in the OTG_FS_GINTSTS register). When the core switches from one mode to the other, the registers in the new mode of operation must be reprogrammed as they would be after a power-on reset.

*Figure 394* shows the interrupt hierarchy.

ST
RM0090 Rev 18
1263/1749

--- Page 94 ---

USB on-the-go full-speed (OTG_FS)
RM0090

Figure 394. Interrupt hierarchy

MSV36921V3

1. OTG_FS_WKUP become active (high state) when resume condition occurs during L1 SLEEP or L2 SUSPEND states.

(1)
Wakeup interrupt
OTG_FS_WKUP
Global interrupt
OTG_FS

Global interrupt mask (bit 0)
AND
OR
OTGINT
AHB configuration register
OTG_AHBCFG
Core interrupt mask register
OTG_GINTMASK

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
OEPMNP
IEPINT
OTGINT

(15 + #EP):16
#EP:1-0
OUT endpoints
IN endpoints
OTG_DAINTMASK
Device all endpoints interrupt mask
register
OTG_DAINT
Device all endpoints interrupt register
OTG_DOEPMSK/
Device IN/OUT endpoints common
interrupt mask register
OTG_DOEPINTx/
Device IN/OUT endpoint interrupt
registers
x = 0
x = #HC-1
OTG_GOTGINT
OTG interrupt register

Host port control and status register
OTG_HPRT
OTG_HAINTMASK
Host all channels interrupt mask register
OTG_HAINT
Host all channels interrupt register
OTG_HCTINTMSKx
Host channels interrupt mask registers
OTG_HCTINTx
Host channels interrupt registers
x = 0
x = #HC-1

--- Page 95 ---

RM0090
USB on-the-go full-speed (OTG_FS)

