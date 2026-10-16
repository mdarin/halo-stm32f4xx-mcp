## 1 ARM ® 32-bit Cortex ® -M4 with FPU limitations

An errata notice of the STM32F40x and STM32F41x core is available from http://infocenter.arm.com.

All the described limitations are minor and related to the revision r0p1-v1 of the Cortex ® -M4 core. Table 3 summarizes these limitations and their implications on the behavior of STM32F40x and STM32F41x devices.

**Table 3. Cortex ® -M4 core limitations and impact on microcontroller behavior**

| ARM ID | ARM category | ARM summary of errata | Impact on STM32F40x and STM32F41x |
|---|---|---|---|
| 752770 | Cat B | Interrupted loads to SP can cause erroneous behavior | Minor |
| 776924 | Cat B | VDIV or VSQRT instructions might not complete correctly when very short ISRs are used | Minor |


**1.1 Cortex ® -M4 interrupted loads to stack pointer can cause erroneous behavior**

**Description**

An interrupt occurring during the data-phase of a single word load to the stack pointer (SP/R13) can cause an erroneous behavior of the device. In addition, returning from the interrupt results in the load instruction being executed an additional time.

For all the instructions performing an update of the base register, the base register is erroneously updated on each execution, resulting in the stack pointer being loaded from an incorrect memory location.

The instructions affected by this limitation are the following:

• LDR SP, [Rn],#imm

• LDR SP, [Rn,#imm]!

• LDR SP, [Rn,#imm]

• LDR SP, [Rn]

• LDR SP, [Rn,Rm]

**Workaround**

As of today, no compiler generates these particular instructions. This limitation can only occur with hand-written assembly code.

Both limitations can be solved by replacing the direct load to the stack pointer by an intermediate load to a general-purpose register followed by a move to the stack pointer. Example:

Replace LDR SP, [R0] by

LDR R2,[R0]

MOV SP,R2

6/38 DocID022183 Rev 7

--- Page 7 ---

**STM32F40x and STM32F41x ARM ® 32-bit Cortex ® -M4 with FPU limitations**

**1.2 VDIV or VSQRT instructions might not complete correctly when very short ISRs are used**

**Description**

On Cortex ® -M4 with FPU core, 14 cycles are required to execute a VDIV or VSQRT instruction.

This limitation is present when the following conditions are met:

• A VDIV or VSQRT is executed

• The destination register for VDIV or VSQRT is one of s0 - s15

• An interrupt occurs and is taken

• The ISR being executed does not contain a floating point instruction

• 14 cycles after the VDIV or VSQRT is executed, an interrupt return is executed In this case, if there are only one or two instructions inside the interrupt service routine, then the VDIV or VQSRT instruction does not complete correctly and the register bank and FPSCR are not updated, meaning that these registers hold incorrect out-of-date data.

**Workaround**

Two workarounds are applicable:

• Disable lazy context save of floating point state by clearing LSPEN to 0 (bit 30 of the

FPCCR at address 0xE000EF34).

• Ensure that every ISR contains more than 2 instructions in addition to the exception

return instruction.



37

--- Page 8 ---

**STM32F40x and STM32F41x silicon limitations STM32F40x and STM32F41x**

