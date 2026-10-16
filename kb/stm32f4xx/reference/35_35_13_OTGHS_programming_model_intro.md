## 35.13 OTG_HS programming model

### 35.13.1 Core initialization

The application must perform the core initialization sequence. If the cable is connected during power-up, the current mode of operation bit in the Core interrupt register (CMOD bit in OTG_HS_GINTSTS) reflects the mode. The OTG_HS controller enters host mode when an "A" plug is connected or peripheral mode when a "B" plug is connected.

This section explains the initialization of the OTG_HS controller after power-on. The application must follow the initialization sequence irrespective of host or peripheral mode operation. All core global registers are initialized according to the core's configuration:

STI
RM0090 Rev 18
1487/1749

--- Page 123 ---

USB on-the-go high-speed (OTG_HS)
RM0090

1. Program the following fields in the Global AHB configuration (OTG_HS_GAHBCFG) register:
    - DMA mode bit
    - AHB burst length field
    - Global interrupt mask bit GINT = 1
    - RxFIFO nonempty (RXFLVL bit in OTG_HS_GINTSTS)
    - Periodic TxFIFO empty level
2. Program the following fields in OTG_HS_GUSBCFG register:
    - HNP capable bit
    - SRP capable bit
    - FS timeout calibration field
    - USB turnaround time field
3. The software must unmask the following bits in the GINTMSK register:
    OTG interrupt mask
    Mode mismatch interrupt mask
4. The software can read the CMOD bit in OTG_HS_GINTSTS to determine whether the OTG_HS controller is operating in host or peripheral mode.

### 35.13.2 Host initialization

To initialize the core as host, the application must perform the following steps:
1. Program the HPRTINT in GINTMSK to unmask
2. Program the OTG_HS_HCFG register to select full-speed host
3. Program the PPWR bit in OTG_HS_HPRT to 1. This drives VBUS on the USB.
4. Wait for the PCDET interrupt in OTG_HS_HPRT0. This indicates that a device is connecting to the port.
5. Program the PRST bit in OTG_HS_HPRT to 1. This starts the reset process.
6. Wait at least 10 ms for the reset process to complete.
7. Program the PRST bit in OTG_HS_HPRT to 0.
8. Wait for the PENCHNG interrupt in OTG_HS_HPRT.
9. Read the PSPD bit in OTG_HS_HPRT to get the enumerated speed.
10. Program the HFIR register with a value corresponding to the selected PHY clock 1.
11. Program the FSLSPCS field in OTG_FS_HCFG register according to the speed of the detected device read in step 9. If FSLSPCS has been changed, reset the port.
12. Program the OTG_HS_GRXFSIZ register to select the size of the receive FIFO.
13. Program the OTG_HS_GNPTXFSIZ register to select the size and the start address of the nonperiodic transmit FIFO for nonperiodic transactions.
14. Program the OTG_HS_HPTXFSIZ register to select the size and start address of the periodic transmit FIFO for periodic transactions.

To communicate with devices, the system software must initialize and enable at least one channel.

1488/1749
RM0090 Rev 18
ST

--- Page 124 ---

RM0090
USB on-the-go high-speed (OTG_HS)

### 35.13.3 Device initialization

The application must perform the following steps to initialize the core as a device on power-up or after a mode change from host to device.

1. Program the following fields in the OTG_HS_DCFG register:
    - Device speed
    - Nonzero-length status OUT handshake
2. Program the OTG_HS_GINTMSK register to unmask the following interrupts:
    - USB reset
    - Enumeration done
    - Early suspend
    - USB suspend
    - SOF
3. Program the VBUSSEN bit in the OTG_HS_GCCFG register to enable VBUS sensing in “B” peripheral mode and supply the 5 volts across the pull-up resistor on the DP line.
4. Wait for the USBRST interrupt in OTG_HS_GINTSTS. It indicates that a reset has been detected on the USB that lasts for about 10 ms on receiving this interrupt.

Wait for the ENUMDNE interrupt in OTG_HS_GINTSTS. This interrupt indicates the end of reset on the USB. On receiving this interrupt, the application must read the OTG_HS_DSTS register to determine the enumeration speed and perform the steps listed in Endpoint initialization on enumeration completion on page 1516.

At this point, the device is ready to accept SOF packets and perform control transfers on control endpoint 0.

### 35.13.4 DMA mode

The OTG host uses the AHB master interface to fetch the transmit packet data (AHB to USB) and receive the data update (USB to AHB). The AHB master uses the programmed DMA address (HCDMAn register in host mode and DIEPDMAn/DOEPDMAn register in peripheral mode) to access the data buffers.

