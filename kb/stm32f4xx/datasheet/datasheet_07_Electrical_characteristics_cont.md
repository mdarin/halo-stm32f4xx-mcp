## 7 Package information

In order to meet environmental requirements, ST offers these devices in different grades of ECOPACK packages, depending on their level of environmental compliance. ECOPACK specifications, grade definitions and product status are available at: www.st.com. ECOPACK is an ST trademark.

### 7.1 Device marking

Refer to technical note “Reference device marking schematics for STM32 microcontrollers and microprocessors” (TN1433), available on www.st.com, for the location of pin 1 / ball A1 as well as the location and orientation of the marking areas versus pin 1 / ball A1.

Parts marked as “ES”, “E”, or accompanied by an engineering sample notification letter, are not yet qualified and therefore not approved for use in production. ST is not responsible for any consequences resulting from such use. In no event will ST be liable for the customer using any of these engineering samples in production. ST’s Quality department must be contacted prior to any decision to use these engineering samples to run a qualification activity.

A WLCSP simplified marking example (if any) is provided in the corresponding package information subsection.

ST
DS8626 Rev 12
165/206

--- Page 166 ---

Package information
STM32F405xx, STM32F407xx

### 7.2 WLCSP90 package information

Figure 73. WLCSP90 - 4.223 x 3.969 mm, 0.400 mm pitch wafer level chip scale package outline

BOTTOM VIEW
BUMP SIDE

A1 ball location
10
A
J
e1
e
e2
G
F

SIDE VIEW
Detail A
A3
A2
A
//bbb Z

FRONT VIEW
A
A2
A3

TOP VIEW
WAFER BACK SIDE
D
E
A1 orientation
reference
aaa (4X)

Detail A
(Rotated 90°)
Bump
eee
b
Seating plane
A1
ccc Z X Y
ddd Z

1. Drawing is not to scale.
A0JW_ME_V4

166/206
DS8626 Rev 12
ST

--- Page 167 ---

**STM32F405xx, STM32F407xx**

**Package information**

**Table 90. WLCSP90 - 4.223 x 3.969 mm, 0.400 mm pitch wafer level chip scale package mechanical data**

| Symbol | millimeters | inches(1) |  |  |  |  |
|---|---|---|---|---|---|---|
| Min | Typ | Max | Min | Typ | Max |  |
| A | 0.540 | 0.570 | 0.600 | 0.0213 | 0.0224 | 0.0236 |
| A1 | - | 0.190 | - | 0.0075 | - |  |
| A2 | - | 0.380 | - | 0.0150 | - |  |
| A3(2) | - | 0.025 | - | 0.0010 | - |  |
| b(3) | 0.240 | 0.270 | 0.300 | 0.0094 | 0.0106 | 0.0118 |
| D | 4.188 | 4.223 | 4.258 | 0.1649 | 0.1663 | 0.1676 |
| E | 3.934 | 3.969 | 4.004 | 0.1549 | 0.1563 | 0.1576 |
| e | - | 0.400 | - | 0.0157 | - |  |
| e1 | - | 3.600 | - | 0.1417 | - |  |
| e2 | - | 3.200 | - | 0.1260 | - |  |
| F | - | 0.3115 | - | 0.0123 | - |  |
| G | - | 0.3845 | - | 0.0151 | - |  |
| aaa | - | 0.100 | - | 0.0039 | - |  |
| bbb | - | 0.100 | - | 0.0039 | - |  |
| ccc | - | 0.100 | - | 0.0039 | - |  |
| ddd | - | 0.050 | - | 0.0020 | - |  |
| eee | - | 0.050 | - | 0.0020 | - |  |


1. Values in inches are converted from mm and rounded to 4 decimal digits.
2. Back side coating.
3. Dimension is measured at the maximum bump diameter parallel to primary datum Z.

**Figure 74. WLCSP90 - 4.223 x 3.969 mm, 0.400 mm pitch wafer level chip scale package recommended footprint**

Dpad
Dsm
MS18965V2

ST
DS8626 Rev 12
167/206

--- Page 168 ---

Package information
STM32F405xx, STM32F407xx

Table 91. WLCSP90 recommended PCB design rules

| Dimension | Recommended values |
|---|---|
| Pitch | 0.4 mm |
| Dpad | 260 µm max. (circular)220 µm recommended |
| Dsm | 300 µm min. (for 260 µm diameter pad) |
| PCB pad design | Non-solder mask defined via underbump allowed |


**Device marking for WLCSP90**

The following figure gives an example of topside marking and ball A1 position identifier location.

The printed markings may differ depending on the supply chain.

Other optional marking or inset/upset marks, which depend on supply chain operations, are not indicated below.

Figure 75. WLCSP90 marking example (package top view)

Product identification(1)
F4050EB
Date code
Y WW R
Revision code
Ball A1 indentifer
MSv36120V1

1. Parts marked as "ES", "E" or accompanied by an Engineering Sample notification letter, are not yet qualified and therefore not yet ready to be used in production and any consequences deriving from such usage will not be at ST charge. In no event, ST will be liable for any customer usage of these engineering samples in production. ST Quality has to be contacted prior to any decision to use these Engineering Samples to run qualification activity.

168/206
DS8626 Rev 12

--- Page 169 ---

STM32F405xx, STM32F407xx
Package information

### 7.3 LQFP64 package information (5W)

This LQFP is 64-pin, 10 x 10 mm low-profile quad flat package.

Figure 76. LQFP64 - Outline(15)

BOTTOM VIEW

4x N/4 TIPS
aaa|A-B|D
bbb|A-B|D 4x

(13) (N - 4)x(6)
A2 A1 (12)
ddd|A-B|D
ccc|C

TOP VIEW

(5) (2)
(10)
N
1 2 3
(3) A
D 1/4
(6)
E 1/4
(Section A-A)
B (3)
(5) (2)
E1 E

D
D1
(4)
D (3)
(4)

SECTION A-A

(2)
(2)
R1
R2
B SECTION B-B
GAUGE PLANE
0.25
S
L
(L1)
(1) (11)
(3)

SECTION B-B

b
(9) (11)
(11)
c
c1
(11)
b1
(11)
WITH PLATING
BASE METAL

5W_LQFP64_ME_V1

ST
DS8626 Rev 12
169/206

--- Page 170 ---

Package information
STM32F405xx, STM32F407xx

Table 92. LQFP64 - Mechanical data

| Symbol | millimeters | inches(14) |  |  |  |  |
|---|---|---|---|---|---|---|
| Min | Typ | Max | Min | Typ | Max |  |
| A | - | 1.60 | - | 0.0630 |  |  |
| A1(12) | 0.05 | - | 0.15 | 0.0020 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0570 |
| b(9)(11) | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0091 |
| c(11) | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1(11) | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| D(4) | 12.00 BSC | 0.4724 BSC |  |  |  |  |
| D1(2)(5) | 10.00 BSC | 0.3937 BSC |  |  |  |  |
| E(4) | 12.00 BSC | 0.4724 BSC |  |  |  |  |
| E1(2)(5) | 10.00 BSC | 0.3937 BSC |  |  |  |  |
| e | 0.50 BSC | 0.1970 BSC |  |  |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.0177 | 0.0236 | 0.0295 |
| L1 | 1.00 REF | 0.0394 REF |  |  |  |  |
| N(13) | 64 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | 0° | - |  |  |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.08 | - | 0.0031 | - |  |  |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | 0.0079 | - |  |  |
| aaa(1) | 0.20 | 0.0079 |  |  |  |  |
| bbb(1) | 0.20 | 0.0079 |  |  |  |  |
| ccc(1) | 0.08 | 0.0031 |  |  |  |  |
| ddd(1) | 0.08 | 0.0031 |  |  |  |  |


170/206
DS8626 Rev 12
ST

--- Page 171 ---

**STM32F405xx, STM32F407xx**

**Package information**

**Notes:**

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The Top package body size may be smaller than the bottom package size by as much as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is "0.25 mm" per side. D1 and E1 are Maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All Dimensions are in millimeters.
8. No intrusion allowed inwards the leads.
9. Dimension "b" does not include dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum "b" dimension by more than 0.08 mm. Dambar cannot be located on the lower radius or the foot. Minimum space between protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. Exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. "N" is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to 4 decimal digits.
15. Drawing is not to scale.

ST
DS8626 Rev 12
171/206

--- Page 172 ---

Package information
STM32F405xx, STM32F407xx

**7.4 LQFP100 package information (1L)**

This LQFP is a 100-pin, 14 x 14 mm low-profile quad flat package.

*Note:* See list of notes in the notes section.

Figure 77. LQFP100 - Outline(15)

BOTTOM VIEW
4x N/4 TIPS
aaa C A-B D
4x
bbb H A-B D

SECTION A-A
H
θ2
(2)
θ1
R1
R2
B SECTION B-B
GAUGE PLANE
S
θ3
L
(L1)
(1) (11)

SIDE VIEW
(N-4) x e
(13)
A
A2 A1 (12)
+ aaa C A-B D
C
ccc C

SECTION B-B
(9) (11)
b
WITH PLATING
(11) c
c1 (11)
b1 (11)
BASE METAL

TOP VIEW
(2) (5)
(10)
N
1
2
3
D
D1
(3)
(4)
(4)
(2)
(5)
E1
E
E1/4
D1/4
(6)
SECTION A-A
A
A
B

1L_LQFP100_ME_V3

172/206
DS8626 Rev 12
ST

--- Page 173 ---

STM32F405xx, STM32F407xx
Package information

Table 93. LQFP100 - Mechanical data

| Symbol | millimeters | inches(14) |  |  |  |  |
|---|---|---|---|---|---|---|
| Min | Typ | Max | Min | Typ | Max |  |
| A | - | 1.50 | 1.60 | - | 0.0590 | 0.0630 |
| A1(12) | 0.05 | - | 0.15 | 0.0019 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0570 |
| b(9)(11) | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0090 |
| c(11) | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1(11) | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| D(4) | 16.00 BSC | 0.6299 BSC |  |  |  |  |
| D1(2)(5) | 14.00 BSC | 0.5512 BSC |  |  |  |  |
| E(4) | 16.00 BSC | 0.6299 BSC |  |  |  |  |
| E1(2)(5) | 14.00 BSC | 0.5512 BSC |  |  |  |  |
| e | 0.50 BSC | 0.0197 BSC |  |  |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.177 | 0.0236 | 0.0295 |
| L1(1)(11) | 1.00 | - | 0.0394 | - |  |  |
| N(13) | 100 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | 0° | - |  |  |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.08 | - | 0.0031 | - |  |  |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | 0.0079 | - |  |  |
| aaa(1) | 0.20 | 0.0079 |  |  |  |  |
| bbb(1) | 0.20 | 0.0079 |  |  |  |  |
| ccc(1) | 0.08 | 0.0031 |  |  |  |  |
| ddd(1) | 0.08 | 0.0031 |  |  |  |  |


ST
DS8626 Rev 12
173/206

--- Page 174 ---

**Package information**
**STM32F405xx, STM32F407xx**

**Notes:**
1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The top package body size may be smaller than the bottom package size by as much as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at the seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is 0.25 mm per side. D1 and E1 are maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All dimensions are in millimeters.
8. No intrusion is allowed inwards the leads.
9. Dimension b does not include a dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum b dimension by more than 0.08 mm. The dambar cannot be located on the lower radius or the foot. The minimum space between the protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. The exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead that is between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. N is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to four decimal digits.
15. Drawing is not to scale.

**Figure 78. LQFP100 - Footprint example**

1L_LQFP100_FP_V1

1. Dimensions are expressed in millimeters.

174/206
DS8626 Rev 12

--- Page 175 ---

**STM32F405xx, STM32F407xx**
**Package information**

### 7.5 LQFP144 package information (1A)

This LQFP is a 144-pin, 20 x 20 mm low-profile quad flat package.

*Note:* See list of notes in the notes section.

**Figure 79. LQFP144 - Outline(15)**

BOTTOM VIEW

4x N/4 TIPS
aaa C A-B D
bbb H A-B D 4x

(N-4)x e
0.05 A2 A1 (12)
b ddd C A-B D
ccc C

D
D1 (4)
(2)(5)
(4)
(10)
N
(3) D
E 1/4
(6)
(3) A
(Section A-A)
A
TOP VIEW

SECTION A-A
02
01
(2)
R1
R2
B
SECTION B-B
GAUGE PLANE
0.25
0
H
S
03
B
L
(L1)
(1) (11)

(11)
c
b
(9) (11)
WITH PLATING
c1
(11)
b1
(11)
BASE METAL
SECTION B-B

1A_LQFP144_ME_V2

ST
DS8626 Rev 12
175/206

--- Page 176 ---

**Package information**
**STM32F405xx, STM32F407xx**

**Table 94. LQFP144 - Mechanical data**

| Symbol | millimeters | inches(14) |  |  |  |  |
|---|---|---|---|---|---|---|
| Min | Typ | Max | Min | Typ | Max |  |
| A | - | 1.60 | - | 0.0630 |  |  |
| A1(12) | 0.05 | - | 0.15 | 0.0020 | - | 0.0059 |
| A2 | 1.35 | 1.40 | 1.45 | 0.0531 | 0.0551 | 0.0571 |
| b(9)(11) | 0.17 | 0.22 | 0.27 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.17 | 0.20 | 0.23 | 0.0067 | 0.0079 | 0.0090 |
| c(11) | 0.09 | - | 0.20 | 0.0035 | - | 0.0079 |
| c1(11) | 0.09 | - | 0.16 | 0.0035 | - | 0.0063 |
| D(4) | 22.00 BSC | 0.8661 BSC |  |  |  |  |
| D1(2)(5) | 20.00 BSC | 0.7874 BSC |  |  |  |  |
| E(4) | 22.00 BSC | 0.8661 BSC |  |  |  |  |
| E1(2)(5) | 20.00 BSC | 0.7874 BSC |  |  |  |  |
| e | 0.50 BSC | 0.0197 BSC |  |  |  |  |
| L | 0.45 | 0.60 | 0.75 | 0.0177 | 0.0236 | 0.0295 |
| L1 | 1.00 REF | 0.0394 REF |  |  |  |  |
| N(13) | 144 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | 0° | - |  |  |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.08 | - | 0.0031 | - |  |  |
| R2 | 0.08 | - | 0.20 | 0.0031 | - | 0.0079 |
| S | 0.20 | - | 0.0079 | - |  |  |
| aaa | 0.20 | 0.0079 |  |  |  |  |
| bbb | 0.20 | 0.0079 |  |  |  |  |
| ccc | 0.08 | 0.0031 |  |  |  |  |
| ddd | 0.08 | 0.0031 |  |  |  |  |


176/206
DS8626 Rev 12
ST

--- Page 177 ---

**STM32F405xx, STM32F407xx**

**Package information**

**Notes:**

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The top package body size may be smaller than the bottom package size by as much as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at the seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is 0.25 mm per side. D1 and E1 are maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All dimensions are in millimeters.
8. No intrusion is allowed inwards the leads.
9. Dimension b does not include a dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum b dimension by more than 0.08 mm. The dambar cannot be located on the lower radius or the foot. The minimum space between the protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. The exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead that is between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. N is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to four decimal digits.
15. Drawing is not to scale.

ST
DS8626 Rev 12
177/206

--- Page 178 ---

Package information
STM32F405xx, STM32F407xx

Figure 80. LQFP144 - Footprint example

1. Dimensions are expressed in millimeters.

178/206
DS8626 Rev 12

--- Page 179 ---

STM32F405xx, STM32F407xx
Package information

### 7.6 UFBGA(176+25) package information (A0E7)

This UFBGA is a 176+25-ball, 10 x 10 mm, 0.65 mm pitch, ultra fine pitch ball grid array package.

**Figure 81. UFBGA(176+25) - Outline**

1. Drawing is not to scale.

**Table 95. UFBGA(176+25) - Mechanical data**

| Symbol | millimeters | inches(1) |  |  |  |  |
|---|---|---|---|---|---|---|
| Min. | Typ. | Max. | Min. | Typ. | Max. |  |
| A | - | 0.600 | - | 0.0236 |  |  |
| A1 | 0.050 | 0.080 | 0.110 | 0.0020 | 0.0031 | 0.0043 |
| A2 | - | 0.450 | - | 0.0177 | - |  |
| A3 | - | 0.130 | - | 0.0051 | - |  |
| A4 | - | 0.320 | - | 0.0126 | - |  |
| b | 0.240 | 0.290 | 0.340 | 0.0094 | 0.0114 | 0.0134 |
| D | 9.850 | 10.000 | 10.150 | 0.3878 | 0.3937 | 0.3996 |
| D1 | - | 9.100 | - | 0.3583 | - |  |
| E | 9.850 | 10.000 | 10.150 | 0.3878 | 0.3937 | 0.3996 |
| E1 | - | 9.100 | - | 0.3583 | - |  |
| e | - | 0.650 | - | 0.0256 | - |  |
| F | - | 0.450 | - | 0.0177 | - |  |
| ddd | - | 0.080 | - | 0.0031 |  |  |


ST
DS8626 Rev 12
179/206

--- Page 180 ---

Package information
STM32F405xx, STM32F407xx

Table 95. UFBGA(176+25) - Mechanical data (continued)
| Symbol | millimeters | inches(1) |  |  |  |
|---|---|---|---|---|---|
| Min. | Typ. | Max. | Min. | Typ. | Max. |
| eee | - | 0.150 | - | 0.0059 |  |
| fff | - | 0.050 | - | 0.0020 |  |

1. Values in inches are converted from mm and rounded to 4 decimal digits.

Figure 82. UFBGA(176+25) - Footprint example
Dpad
Dsm
BGA_WLCSP_FT_V1

Table 96. UFBGA(176+25) - Example of PCB design rules (0.65 mm pitch BGA)
| Dimension | Values |
|---|---|
| Pitch | 0.65 mm |
| Dpad | 0.300 mm |
| Dsm | 0.400 mm typ. (depends on the soldermask registration tolerance) |
| Stencil opening | 0.300 mm |
| Stencil thickness | Between 0.100 mm and 0.125 mm |
| Pad trace width | 0.100 mm |


180/206
DS8626 Rev 12
ST

--- Page 181 ---

STM32F405xx, STM32F407xx
Package information

### 7.7 LQFP176 package information (1T)

This LQFP is a 176-pin, 24 x 24 mm, 0.5 mm pitch, low profile quad flat package.

**Note:** See list of notes in the notes section.

Figure 83. LQFP176 - Outline(15)

BOTTOM VIEW
4x N/4 TIPS
aaa C A-B-D
4x
bbb H A-B D

SECTION A-A
(2)
R1
R2
B(See SECTION B-B)
GAUGE PLANE
0.25
(1) (11)

SIDE VIEW
A2 -0.05
(N-4) x e (13)
A1 (12)
ddd C A-BD
ccc C

TOP VIEW
(2) (5)
(10)
N
D
D1
(3)
(4)
E1/4
D1/4
(6)
SECTION A-A
A
A
(3)
(5)
(2)
E1 E
(4)

SECTION B-B
(9) (11)
WITH PLATING
(11) c
b
c1 (11)
b1 (11)
BASE METAL

1T_LQFP176_ME_V2

ST
DS8626 Rev 12
181/206

--- Page 182 ---

Package information
STM32F405xx, STM32F407xx

Table 97. LQFP176 - Mechanical data

| Symbol | millimeters | inches(14) |  |  |  |  |
|---|---|---|---|---|---|---|
| Min | Typ | Max | Min | Typ | Max |  |
| A | - | 1.600 | - | 0.0630 |  |  |
| A1(12) | 0.050 | - | 0.150 | 0.0020 | - | 0.0059 |
| A2 | 1.350 | 1.400 | 1.450 | 0.0531 | 0.0551 | 0.0571 |
| b(9)(11) | 0.170 | 0.220 | 0.270 | 0.0067 | 0.0087 | 0.0106 |
| b1(11) | 0.170 | 0.200 | 0.230 | 0.0067 | 0.0079 | 0.0091 |
| c(11) | 0.090 | - | 0.200 | 0.0035 | - | 0.0079 |
| c1(11) | 0.090 | - | 0.160 | 0.0035 | - | 0.063 |
| D(4) | 26.000 | 1.0236 |  |  |  |  |
| D1(2)(5) | 24.000 | 0.9449 |  |  |  |  |
| E(4) | 26.000 | 0.0197 |  |  |  |  |
| E1(2)(5) | 24.000 | 0.9449 |  |  |  |  |
| e | 0.500 | 0.1970 |  |  |  |  |
| L | 0.450 | 0.600 | 0.750 | 0.0177 | 0.0236 | 0.0295 |
| L1(1)(11) | 1 | 0.0394 REF |  |  |  |  |
| N(13) | 176 |  |  |  |  |  |
| θ | 0° | 3.5° | 7° | 0° | 3.5° | 7° |
| θ1 | 0° | - | 0° | - |  |  |
| θ2 | 10° | 12° | 14° | 10° | 12° | 14° |
| θ3 | 10° | 12° | 14° | 10° | 12° | 14° |
| R1 | 0.080 | - | 0.0031 | - |  |  |
| R2 | 0.080 | - | 0.200 | 0.0031 | - | 0.0079 |
| S | 0.200 | - | 0.0079 | - |  |  |
| aaa(1) | 0.200 | 0.0079 |  |  |  |  |
| bbb(1) | 0.200 | 0.0079 |  |  |  |  |
| ccc(1) | 0.080 | 0.0031 |  |  |  |  |
| ddd(1) | 0.080 | 0.0031 |  |  |  |  |


182/206
DS8626 Rev 12
ST

--- Page 183 ---

**STM32F405xx, STM32F407xx**
**Package information**

**Notes:**

1. Dimensioning and tolerancing schemes conform to ASME Y14.5M-1994.
2. The top package body size may be smaller than the bottom package size by as much as 0.15 mm.
3. Datums A-B and D to be determined at datum plane H.
4. To be determined at the seating datum plane C.
5. Dimensions D1 and E1 do not include mold flash or protrusions. Allowable mold flash or protrusions is 0.25 mm per side. D1 and E1 are maximum plastic body size dimensions including mold mismatch.
6. Details of pin 1 identifier are optional but must be located within the zone indicated.
7. All dimensions are in millimeters.
8. No intrusion is allowed inwards the leads.
9. Dimension b does not include a dambar protrusion. Allowable dambar protrusion shall not cause the lead width to exceed the maximum b dimension by more than 0.08 mm. The dambar cannot be located on the lower radius or the foot. The minimum space between the protrusion and an adjacent lead is 0.07 mm for 0.4 mm and 0.5 mm pitch packages.
10. The exact shape of each corner is optional.
11. These dimensions apply to the flat section of the lead that is between 0.10 mm and 0.25 mm from the lead tip.
12. A1 is defined as the distance from the seating plane to the lowest point on the package body.
13. N is the number of terminal positions for the specified body size.
14. Values in inches are converted from mm and rounded to four decimal digits.
15. Drawing is not to scale.

ST
DS8626 Rev 12
183/206

--- Page 184 ---

Package information
STM32F405xx, STM32F407xx

Figure 84. LQFP176 - Footprint example

1. Dimensions are expressed in millimeters.

1T_FP_V1

184/206
DS8626 Rev 12

--- Page 185 ---

STM32F405xx, STM32F407xx
Package information

### 7.8 Thermal characteristics

The maximum chip-junction temperature, T_J max, in degrees Celsius, may be calculated using the following equation:

T_J max = T_A max + (P_D max × Θ_JA)

Where:
*   T_A max is the maximum ambient temperature in °C,
*   Θ_JA is the package junction-to-ambient thermal resistance, in °C/W,
*   P_D max is the sum of P_INT max and P_I/O max (P_D max = P_INT max + P_I/O max),
*   P_INT max is the product of I_DD and V_DD, expressed in Watts. This is the maximum chip internal power.

P_I/O max represents the maximum power dissipation on output pins where:

P_I/O max = Σ (V_OL × I_OL) + Σ((V_DD - V_OH) × I_OH),

taking into account the actual V_OL / I_OL and V_OH / I_OH of the I/Os at low and high level in the application.

**Table 98. Package thermal characteristics**

| Symbol | Parameter | Value | Unit |
|---|---|---|---|
| Θ_JA | Thermal resistance junction-ambient LQFP64 - 10 × 10 mm / 0.5 mm pitch | 46 | °C/W |
| Thermal resistance junction-ambient LQFP100 - 14 × 14 mm / 0.5 mm pitch | 43 |  |  |
| Thermal resistance junction-ambient LQFP144 - 20 × 20 mm / 0.5 mm pitch | 40 |  |  |
| Thermal resistance junction-ambient LQFP176 - 24 × 24 mm / 0.5 mm pitch | 38 |  |  |
| Thermal resistance junction-ambient UFBGA176 - 10× 10 mm / 0.65 mm pitch | 39 |  |  |
| Thermal resistance junction-ambient WLCSP90 - 0.400 mm pitch | 38.1 |  |  |


**Reference document**

JESD51-2 Integrated Circuits Thermal Test Method Environment Conditions - Natural Convection (Still Air). Available from www.jedec.org.

ST
DS8626 Rev 12
185/206

--- Page 186 ---

Ordering information
STM32F405xx, STM32F407xx

