> [RM0090 ch.37 §37.7] SDRAM controller


#### SDRAM initialization

The initialization sequence is managed by software. If the two banks are used, the initialization sequence must be generated simultaneously to Bank 1 and Bank 2 by setting the Target Bank bits CTB1 and CTB2 in the FMC_SDCMR register:

1. Program the memory device features into the FMC_SDCRx register. The SDRAM clock frequency, RBURST and RPIPE must be programmed in the FMC_SDCR1 register.
2. Program the memory device timing into the FMC_SDTRx register. The TRP and TRC timings must be programmed in the FMC_SDTR1 register.
3. Set MODE bits to '001' and configure the Target Bank bits (CTB1 and/or CTB2) in the FMC_SDCMR register to start delivering the clock to the memory (SDCKE is driven high).
4. Wait during the prescribed delay period. Typical delay is around 100 µs (refer to the SDRAM datasheet for the required delay after power-up).
5. Set MODE bits to '010' and configure the Target Bank bits (CTB1 and/or CTB2) in the FMC_SDCMR register to issue a "Precharge All" command.
6. Set MODE bits to '011', and configure the Target Bank bits (CTB1 and/or CTB2) as well as the number of consecutive Auto-refresh commands (NRFS) in the FMC_SDCMR register. Refer to the SDRAM datasheet for the number of Auto-refresh commands that should be issued. Typical number is 8.
7. Configure the MRD field according to your SDRAM device, set the MODE bits to '100', and configure the Target Bank bits (CTB1 and/or CTB2) in the FMC_SDCMR register to issue a "Load Mode Register" command in order to program the SDRAM. In particular:
    a) The CAS latency must be selected following configured value in FMC_SDCR1/2 registers
    b) The Burst Length (BL) of 1 must be selected by configuring the M[2:0] bits to 000 in the mode register (refer to the SDRAM datasheet). If the Mode Register is not the same for both SDRAM banks, this step has to be repeated twice, once for each bank, and the Target Bank bits set accordingly.
8. Program the refresh rate in the FMC_SDTR register
   The refresh rate corresponds to the delay between refresh cycles. Its value must be adapted to SDRAM devices.
9. For mobile SDRAM devices, to program the extended mode register it should be done once the SDRAM device is initialized: First, a dummy read access should be performed while BA1=1 and BA=0 (refer to SDRAM address mapping section for BA[1:0] address mapping) in order to select the extended mode register instead of Load mode register and then program the needed value.

At this stage the SDRAM device is ready to accept commands. If a system reset occurs during an ongoing SDRAM access, the data bus might still be driven by the SDRAM device. Therefor the SDRAM device must be first reinitialized after reset before issuing any new access by the NOR Flash/PSRAM/SRAM or NAND Flash/PC Card controller.

**Note:** If two SDRAM devices are connected to the FMC, all the accesses performed at the same time to both devices by the Command Mode register (Load Mode Register and Self-refresh

1664/1749 RM0090 Rev 18

--- Page 105 ---

RM0090
Flexible memory controller (FMC)

commands) are issued using the timing parameters configured for SDRAM Bank 1 (TMRD,
TRAS and TXSR timings) in the FMC_SDTR1 register.

**SDRAM controller write cycle**

The SDRAM controller accepts single and burst write requests and translates them into
single memory accesses. In both cases, the SDRAM controller keeps track of the active row
for each bank to be able to perform consecutive write accesses to different banks (Multibank
ping-pong access).

Before performing any write access, the SDRAM bank write protection must be disabled by
clearing the WP bit in the FMC_SDCRx register.

Figure 478. Burst write SDRAM access waveforms

TRCD = 3

SDNE
SDCLK
A[12:0] Row n Cola Colb Colc Cold Cole Colf Cog Colh Coli Colj Colk Coll
NRAS
NCAS
SDNWE
DATA[31:0] Dna Dnb Dnc Dnd Dne Dnf Dng Dnh Dni Dnj Dnk Dnl

MS30448V3

The SDRAM controller always checks the next access.
* If the next access is in the same row or in another active row, the write operation is
carried out,
* if the next access targets another row (not active), the SDRAM controller generates a
precharge command, activates the new row and initiates a write command.

ST
RM0090 Rev 18
1665/1749

--- Page 106 ---

Flexible memory controller (FMC)
RM0090

