**38.15 ETM (Embedded trace macrocell)**

**38.15.1 ETM general description**

The ETM enables the reconstruction of program execution. Data are traced using the Data Watchpoint and Trace (DWT) component or the Instruction Trace Macrocell (ITM) whereas instructions are traced using the Embedded Trace Macrocell (ETM).

The ETM transmits information as packets and is triggered by embedded resources. These resources must be programmed independently and the trigger source is selected using the Trigger Event Register (0xE0041008). An event could be a simple event (address match from an address comparator) or a logic equation between 2 events. The trigger source is one of the fourth comparators of the DWT module, The following events can be monitored:

*   Clock cycle matching
*   Data address matching

For more informations on the trigger resources refer to Section 38.13.

The packets transmitted by the ETM are output to the TPIU (Trace Port Interface Unit). The formatter of the TPIU adds some extra packets (refer to Section 38.17) and then outputs the complete packet sequence to the debugger host.

**38.15.2 ETM signal protocol and packet types**

This part is described in the chapter 7 ETMv3 Signal Protocol of the Arm® IHI 0014N document.

ST
RM0090 Rev 18
1701/1749

--- Page 142 ---

Debug support (DBG)
RM0090

### 38.15.3 Main ETM registers

For more information on registers refer to the chapter 3 of the Arm® IHI 0014N specification.

**Table 309. Main ETM registers**

| Address | Register | Details |
|---|---|---|
| 0xE0041FB0 | ETM Lock Access | Write 0xC5ACCE55 to unlock the write access to the other ETM registers. |
| 0xE0041000 | ETM Control | This register controls the general operation of the ETM, for instance how tracing is enabled. |
| 0xE0041010 | ETM Status | This register provides information about the current status of the trace and trigger logic. |
| 0xE0041008 | ETM Trigger Event | This register defines the event that will control trigger. |
| 0xE004101C | ETM Trace Enable Control | This register defines which comparator is selected. |
| 0xE0041020 | ETM Trace Enable Event | This register defines the trace enabling event. |
| 0xE0041024 | ETM Trace Start/Stop | This register defines the traces used by the trigger source to start and stop the trace, respectively. |


### 38.15.4 ETM configuration example

To output a simple value to the TPIU:

* Configure the TPIU and enable the I/IO_TRACEN to assign TRACE I/Os in the STM32F4xx debug configuration register.
* Write 0xC5AC CE55 to the ETM Lock Access Register to unlock the write access to the ITM registers
* Write 0x0000 1D1E to the ETM control register (configure the trace)
* Write 0x0000 406F to the ETM Trigger Event register (define the trigger event)
* Write 0x0000 006F to the ETM Trace Enable Event register (define an event to start/stop)
* Write 0x0200 00000x0000 0001 to the ETM Trace Start/stop register (enable the trace)
* Write 0x0000191E to the ETM Control Register (end of configuration)

