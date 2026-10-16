> [DATASHEET ch.6 §6] Electrical characteristics

### 6.3.17 NRST pin characteristics

### 6.3.17 NRST pin characteristics

The NRST pin input driver uses CMOS technology. It is connected to a permanent pull-up resistor, $R_{PU}$ (see Table 48).

Unless otherwise specified, the parameters given in Table 51 are derived from tests performed under the ambient temperature and $V_{DD}$ supply voltage conditions summarized in Table 14.

**Table 51. NRST pin characteristics**

| Symbol | Parameter | Conditions | Min | Typ | Max | Unit |
|---|---|---|---|---|---|---|
| $V_{IL(NRST)}^{(1)}$ | NRST Input low level voltage | TTL ports$2.7 \text{ V} \le V_{DD} \le 3.6 \text{ V}$ | - | 0.8 | V |  |
| $V_{IH(NRST)}^{(1)}$ | NRST Input high level voltage | 2 | - |  |  |  |
| $V_{IL(NRST)}^{(1)}$ | NRST Input low level voltage | CMOS ports$1.8 \text{ V} \le V_{DD} \le 3.6 \text{ V}$ | - | $0.3V_{DD}$ |  |  |
| $V_{IH(NRST)}^{(1)}$ | NRST Input high level voltage | $0.7V_{DD}$ | - |  |  |  |
| $V_{hys(NRST)}$ | NRST Schmitt trigger voltage hysteresis | - | 200 | - | mV |  |
| $R_{PU}$ | Weak pull-up equivalent resistor$^{(2)}$ | $V_{IN} = V_{SS}$ | 30 | 40 | 50 | k$\Omega$ |
| $V_{F(NRST)}^{(1)}$ | NRST Input filtered pulse | - | 100 | ns |  |  |
| $V_{NF(NRST)}^{(1)}$ | NRST Input not filtered pulse | $V_{DD} > 2.7 \text{ V}$ | 300 | - | ns |  |
| $T_{NRST\_OUT}$ | Generated reset pulse duration | Internal reset source | 20 | - | $\mu$s |  |


1. Specified by design.
2. The pull-up is designed with a true resistance in series with a switchable PMOS. This PMOS contribution to the series resistance must be minimum (~10% order).

**Figure 38. Recommended NRST pin protection**

[Image: Circuit diagram showing an external reset circuit connected to the NRST pin of an STM32F device. The external circuit includes a switch and a 0.1 µF capacitor. The internal block diagram shows the NRST pin connected to a pull-up resistor (RPU), a Schmitt trigger, and a filter leading to the Internal Reset logic.]

1. The reset network protects the device against parasitic resets.
2. The user must ensure that the level on the NRST pin can go below the $V_{IL(NRST)}$ max level specified in Table 51. Otherwise the reset is not taken into account by the device.

