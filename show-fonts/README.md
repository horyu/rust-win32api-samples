# show-fonts

## Environment

```powershell
PS C:\Users\owner> [System.Environment]::OSVersion.Version
Major  Minor  Build  Revision
-----  -----  -----  --------
10     0      19044  0


PS C:\Users\owner> rustc -V
rustc 1.63.0-nightly (a6b8c6954 2022-06-03)
```

## Execution Result

```text
0: Cascadia Code
  0(5,0,200): ExtraLight
  1(5,2,200): ExtraLight Italic
  2(5,0,300): Light
  3(5,2,300): Light Italic
  4(5,0,350): SemiLight
  5(5,2,350): SemiLight Italic
  6(5,0,400): Regular
  7(5,2,400): Italic
  8(5,0,600): SemiBold
  9(5,2,600): SemiBold Italic
  10(5,0,700): Bold
  11(5,2,700): Bold Italic
  12(5,1,200): ExtraLight Oblique
  13(5,1,300): Light Oblique
  14(5,1,350): SemiLight Oblique
  15(5,1,400): Oblique
  16(5,1,600): SemiBold Oblique
  17(5,1,700): Bold Oblique
1: Cascadia Mono
  0(5,0,200): ExtraLight
  1(5,2,200): ExtraLight Italic
  2(5,0,300): Light
  3(5,2,300): Light Italic
  4(5,0,350): SemiLight
  5(5,2,350): SemiLight Italic
  6(5,0,400): Regular
  7(5,2,400): Italic
  8(5,0,600): SemiBold
  9(5,2,600): SemiBold Italic
  10(5,0,700): Bold
  11(5,2,700): Bold Italic
  12(5,1,200): ExtraLight Oblique
  13(5,1,300): Light Oblique
  14(5,1,350): SemiLight Oblique
  15(5,1,400): Oblique
  16(5,1,600): SemiBold Oblique
  17(5,1,700): Bold Oblique
2: Arial
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik | nghiêng
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | đậm
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik | nghiêng đậm
  4(5,0,900): Black
  5(5,1,400): Oblique
  6(5,1,700): Bold Oblique
  7(5,1,900): Black Oblique
3: Bahnschrift
  0(5,0,300): Light
  1(3,0,300): Light Condensed
  2(4,0,300): Light SemiCondensed
  3(5,0,350): SemiLight
  4(3,0,350): SemiLight Condensed
  5(4,0,350): SemiLight SemiCondensed
  6(5,0,400): Regular
  7(3,0,400): Condensed
  8(4,0,400): SemiCondensed
  9(5,0,600): SemiBold
  10(3,0,600): SemiBold Condensed
  11(4,0,600): SemiBold SemiCondensed
  12(5,0,700): Bold
  13(3,0,700): Bold Condensed
  14(4,0,700): Bold SemiCondensed
  15(5,1,300): Light Oblique
  16(3,1,300): Light Condensed Oblique
  17(4,1,300): Light SemiCondensed Oblique
  18(5,1,350): SemiLight Oblique
  19(3,1,350): SemiLight Condensed Oblique
  20(4,1,350): SemiLight SemiCondensed Oblique
  21(5,1,400): Oblique
  22(3,1,400): Condensed Oblique
  23(4,1,400): SemiCondensed Oblique
  24(5,1,600): SemiBold Oblique
  25(3,1,600): SemiBold Condensed Oblique
  26(4,1,600): SemiBold SemiCondensed Oblique
  27(5,1,700): Bold Oblique
  28(3,1,700): Bold Condensed Oblique
  29(4,1,700): Bold SemiCondensed Oblique
4: Calibri
  0(5,0,300): Light
  1(5,2,300): Light Italic
  2(5,0,400): Regular
  3(5,2,400): Italic
  4(5,0,700): Bold
  5(5,2,700): Bold Italic
  6(5,1,300): Light Oblique
  7(5,1,400): Oblique
  8(5,1,700): Bold Oblique
5: Cambria
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
6: Cambria Math
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
7: Candara
  0(5,0,300): Light
  1(5,2,300): Light Italic
  2(5,0,400): Regular
  3(5,2,400): Italic
  4(5,0,700): Bold
  5(5,2,700): Bold Italic
  6(5,1,300): Light Oblique
  7(5,1,400): Oblique
  8(5,1,700): Bold Oblique
8: Comic Sans MS
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
9: Consolas
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
10: Constantia
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
11: Corbel
  0(5,0,300): Light
  1(5,2,300): Light Italic
  2(5,0,400): Regular
  3(5,2,400): Italic
  4(5,0,700): Bold
  5(5,2,700): Bold Italic
  6(5,1,300): Light Oblique
  7(5,1,400): Oblique
  8(5,1,700): Bold Oblique
12: Courier New
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik | nghiêng
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | đậm
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik | đậm nghiêng
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
13: Ebrima
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
14: Franklin Gothic
  0(5,0,400): Medium
  1(5,2,400): Medium Italic
  2(5,1,400): Medium Oblique
  3(5,0,700): Bold
  4(5,1,700): Bold Oblique
  5(5,2,700): Italic Bold
15: Gabriola
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
16: Gadugi
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
17: Georgia
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
18: Impact
  0(3,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(3,1,400): Oblique
  2(3,0,700): Bold
  3(3,1,700): Bold Oblique
19: Ink Free
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
20: Javanese Text
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
21: Leelawadee UI
  0(5,0,350): Semilight
  1(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,1,350): Semilight Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
22: Lucida Console
  0(4,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Navadno | Normal | Normal
  1(4,1,400): Oblique
  2(4,0,700): Bold
  3(4,1,700): Bold Oblique
23: Lucida Sans Unicode
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
24: Malgun Gothic | 맑은 고딕
  0(5,0,300): Semilight | Semilight
  1(5,0,400): Regular | Regular
  2(5,0,700): Bold | Bold
  3(5,1,300): Semilight Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
25: Microsoft Himalaya
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
26: Microsoft JhengHei | 微軟正黑體 | 微軟正黑體
  0(5,0,290): Light | Light | Light
  1(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | Regular | Regular
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | Bold | Bold
  3(5,1,290): Light Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
27: Microsoft JhengHei UI | Microsoft JhengHei UI | Microsoft JhengHei UI
  0(5,0,290): Light | Light | Light
  1(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | Regular | Regular
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | Bold | Bold
  3(5,1,290): Light Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
28: Microsoft New Tai Lue
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
29: Microsoft PhagsPa
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
30: Microsoft Sans Serif
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
31: Microsoft Tai Le
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
32: Microsoft YaHei | 微软雅黑
  0(5,0,290): Light | Light
  1(5,0,400): Normal | obyčejné | Normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | Regular
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | Bold
  3(5,1,290): Light Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
33: Microsoft YaHei UI | Microsoft YaHei UI
  0(5,0,290): Light | Light
  1(5,0,400): Normal | obyčejné | Normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | Regular
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | Bold
  3(5,1,290): Light Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
34: Microsoft Yi Baiti
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
35: MingLiU-ExtB | 細明體-ExtB | 細明體-ExtB
  0(5,0,400): Regular | Regular | Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
36: PMingLiU-ExtB | 新細明體-ExtB | 新細明體-ExtB
  0(5,0,400): Regular | Regular | Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
37: MingLiU_HKSCS-ExtB | 細明體_HKSCS-ExtB | 細明體_HKSCS-ExtB
  0(5,0,400): Regular | Regular | Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
38: Mongolian Baiti
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
39: MS Gothic | ＭＳ ゴシック
  0(5,0,400): Regular | 標準
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
40: MS UI Gothic | MS UI Gothic
  0(5,0,400): Regular | 標準
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
41: MS PGothic | ＭＳ Ｐゴシック
  0(5,0,400): Regular | 標準
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
42: MV Boli
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
43: Myanmar Text
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
44: Nirmala UI
  0(5,0,350): Semilight
  1(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,1,350): Semilight Oblique
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
45: Palatino Linotype
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik | nghiêng
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | đậm
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik | nghiêng đậm
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
46: Segoe MDL2 Assets
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
47: Segoe Print
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
48: Segoe Script
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
49: Segoe UI
  0(5,0,300): Light
  1(5,2,300): Light Italic
  2(5,0,350): Semilight
  3(5,2,350): Semilight Italic
  4(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  5(5,2,400): Cursiva | Kurzíva | Kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik | Nghiêng
  6(5,0,600): Semibold
  7(5,2,600): Semibold Italic
  8(5,0,700): Negreta | Tučné | Fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | Đậm
  9(5,2,700): Negreta Cursiva | Tučné Kurzíva | Fed Kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi Etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér Dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona Kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná Kurzíva | Krepko Poševno | Fet Kursiv | Kalın İtalik | Nghiêng Đậm
  10(5,0,900): Black
  11(5,2,900): Black Italic
  12(5,1,300): Light Oblique
  13(5,1,350): Semilight Oblique
  14(5,1,400): Oblique
  15(5,1,600): Semibold Oblique
  16(5,1,700): Bold Oblique
  17(5,1,900): Black Oblique
50: Segoe UI Emoji
  0(5,0,400): Normal | obyčejné | Normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
51: Segoe UI Historic
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
52: Segoe UI Symbol
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
53: SimSun | 宋体 | 宋体
  0(5,0,400): Regular | 常规 | 常规
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
54: NSimSun | 新宋体 | 新宋体
  0(5,0,400): Regular | 常规 | 常规
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
55: SimSun-ExtB
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
56: Sitka Small
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
57: Sitka Text
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
58: Sitka Subheading
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
59: Sitka Heading
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
60: Sitka Display
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
61: Sitka Banner
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
62: Sylfaen
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
63: Symbol
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
64: Tahoma
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | đậm
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
65: Times New Roman
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,2,400): cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik | nghiêng
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiona | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın | đậm
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik | nghiêng đậm
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
66: Trebuchet MS
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
67: Verdana
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
68: Webdings
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
69: Wingdings
  0(5,0,400): normal | Standard | Regular | Normal | Normal | Normaali | Normal | Normale | Normal | Standaard | Normal | Normálne | Navadno | Normal
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
70: Yu Gothic | 游ゴシック
  0(5,0,300): Light | Light
  1(5,0,400): Regular | Regular
  2(5,0,500): Medium | Medium
  3(5,0,700): Bold | Bold
  4(5,1,300): Light Oblique
  5(5,1,400): Oblique
  6(5,1,500): Medium Oblique
  7(5,1,700): Bold Oblique
71: Yu Gothic UI
  0(5,0,300): Light
  1(5,0,350): Semilight
  2(5,0,400): Regular
  3(5,0,600): Semibold
  4(5,0,700): Bold
  5(5,1,300): Light Oblique
  6(5,1,350): Semilight Oblique
  7(5,1,400): Oblique
  8(5,1,600): Semibold Oblique
  9(5,1,700): Bold Oblique
72: BIZ UDGothic | BIZ UDゴシック
  0(5,0,400): Regular | Regular
  1(5,0,700): Bold | Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
73: BIZ UDPGothic | BIZ UDPゴシック
  0(5,0,400): Regular | Regular
  1(5,0,700): Bold | Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
74: BIZ UDMincho | BIZ UD明朝
  0(5,0,500): Medium | Medium
  1(5,1,500): Medium Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
75: BIZ UDPMincho | BIZ UDP明朝
  0(5,0,500): Medium | Medium
  1(5,1,500): Medium Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
76: Meiryo | メイリオ
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | レギュラー | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | イタリック | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | ボールド | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | ボールド イタリック | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
77: Meiryo UI | Meiryo UI
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Regular | Normal | Standaard | Normalny | Normal | Normal | Обычны й | Normálne | Navadno | Normal | Normal
  1(5,2,400): Cursiva | kurzíva | kursiv | Kursiv | Πλάγια | Italic | Cursiva | Cursiva | Cursiva | Etzana | Kursivoitu | Italique | Italique | Dőlt | Corsivo | Italic | Kursiv | Cursief | Kursywa | Itálico | Itálico | Курсив | Kurzíva | Poševno | Kursiv | İtalik
  2(5,0,700): Negreta | tučné | fed | Fett | Έντονα | Bold | Negrita | Negrita | Negrita | Lodia | Lihavoitu | Gras | Gras | Félkövér | Grassetto | Bold | Halvfet | Vet | Pogrubiony | Negrito | Negrito | Полужирный | Tučné | Krepko | Fet | Kalın
  3(5,2,700): Negreta cursiva | tučné kurzíva | fed kursiv | Fett Kursiv | Έντονα Πλάγια | Bold Italic | Negrita Cursiva | Negrita Cursiva | Negrita Cursiva | Lodi etzana | Lihavoitu Kursivoi | Gras Italique | Gras Italique | Félkövér dőlt | Grassetto Corsivo | Bold Italic | Halvfet Kursiv | Vet Cursief | Pogrubiona kursywa | Negrito Itálico | Negrito Itálico | Полужирный Курсив | Tučná kurzíva | Krepko poševno | Fet Kursiv | Kalın İtalik
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
78: MS Mincho | ＭＳ 明朝
  0(5,0,400): Regular | 標準
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
79: MS PMincho | ＭＳ Ｐ明朝
  0(5,0,400): Regular | 標準
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
80: UD Digi Kyokasho N-B | UD デジタル 教科書体 N-B
  0(5,0,700): Bold | Bold
  1(5,1,700): Bold Oblique
81: UD Digi Kyokasho NP-B | UD デジタル 教科書体 NP-B
  0(5,0,700): Bold | Bold
  1(5,1,700): Bold Oblique
82: UD Digi Kyokasho NK-B | UD デジタル 教科書体 NK-B
  0(5,0,700): Bold | Bold
  1(5,1,700): Bold Oblique
83: UD Digi Kyokasho N-R | UD デジタル 教科書体 N-R
  0(5,0,400): Regular | Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
84: UD Digi Kyokasho NP-R | UD デジタル 教科書体 NP-R
  0(5,0,400): Regular | Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
85: UD Digi Kyokasho NK-R | UD デジタル 教科書体 NK-R
  0(5,0,400): Regular | Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
86: Yu Mincho | 游明朝
  0(5,0,300): Light | Light
  1(5,0,400): Regular | Regular
  2(5,0,600): Demibold | Demibold
  3(5,1,300): Light Oblique
  4(5,1,400): Oblique
  5(5,1,600): Demibold Oblique
87: HoloLens MDL2 Assets
  0(5,0,400): Normal | obyčejné | normal | Standard | Κανονικά | Regular | Normal | Normal | Normal | Arrunta | Normaali | Normal | Normal | Normál | Normale | Normal | Standaard | Normalny | Normal | Normal | Обычный | Normálne | Navadno | Normal | Normal | thường
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
88: Amiri
  0(5,0,400): Regular
  1(5,1,400): Slanted
  2(5,0,700): Bold
  3(5,1,700): Bold Slanted
89: Alef
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
90: Scheherazade
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
91: KacstBook
  0(5,0,500): Medium
  1(5,1,500): Medium Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
92: Source Code Pro
  0(5,0,200): ExtraLight
  1(5,2,200): ExtraLight Italic
  2(5,0,300): Light
  3(5,2,300): Light Italic
  4(5,0,400): Regular
  5(5,2,400): Italic
  6(5,0,500): Medium
  7(5,2,500): Medium Italic
  8(5,0,600): Semibold
  9(5,2,600): Semibold Italic
  10(5,0,700): Bold
  11(5,2,700): Bold Italic
  12(5,0,900): Black
  13(5,2,900): Black Italic
  14(5,1,200): ExtraLight Oblique
  15(5,1,300): Light Oblique
  16(5,1,400): Oblique
  17(5,1,500): Medium Oblique
  18(5,1,600): Semibold Oblique
  19(5,1,700): Bold Oblique
  20(5,1,900): Black Oblique
93: Gentium Basic
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
94: DejaVu Math TeX Gyre
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
95: EmojiOne Color
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
96: Reem Kufi
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
97: David Libre
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
98: Source Sans Pro
  0(5,0,200): ExtraLight
  1(5,2,200): ExtraLight Italic
  2(5,0,300): Light
  3(5,2,300): Light Italic
  4(5,0,400): Regular
  5(5,2,400): Italic
  6(5,0,600): Semibold
  7(5,2,600): Semibold Italic
  8(5,0,700): Bold
  9(5,2,700): Bold Italic
  10(5,0,900): Black
  11(5,2,900): Black Italic
  12(5,1,200): ExtraLight Oblique
  13(5,1,300): Light Oblique
  14(5,1,400): Oblique
  15(5,1,600): Semibold Oblique
  16(5,1,700): Bold Oblique
  17(5,1,900): Black Oblique
99: Linux Biolinum G
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,1,400): Oblique
  4(5,1,700): Bold Oblique
  5(5,2,700): Italic Bold
100: Liberation Mono
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
101: OpenSymbol
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
102: Source Serif Pro
  0(5,0,200): ExtraLight
  1(5,2,200): ExtraLight Italic
  2(5,0,300): Light
  3(5,2,300): Light Italic
  4(5,0,400): Regular
  5(5,2,400): Italic
  6(5,0,600): Semibold
  7(5,2,600): Semibold Italic
  8(5,0,700): Bold
  9(5,2,700): Bold Italic
  10(5,0,900): Black
  11(5,2,900): Black Italic
  12(5,1,200): ExtraLight Oblique
  13(5,1,300): Light Oblique
  14(5,1,400): Oblique
  15(5,1,600): Semibold Oblique
  16(5,1,700): Bold Oblique
  17(5,1,900): Black Oblique
103: Caladea
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
104: Liberation Sans
  0(5,0,400): Regular
  1(3,0,400): Narrow
  2(5,2,400): Italic
  3(3,2,400): Narrow Italic
  4(5,0,700): Bold
  5(3,0,700): Narrow Bold
  6(5,2,700): Bold Italic
  7(3,2,700): Narrow Bold Italic
  8(5,1,400): Oblique
  9(3,1,400): Narrow Oblique
  10(5,1,700): Bold Oblique
  11(3,1,700): Narrow Bold Oblique
105: David CLM
  0(5,0,500): Medium
  1(5,0,700): Bold
  2(5,1,500): Medium Oblique
  3(5,1,700): Bold Oblique
106: Carlito
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
107: Noto Kufi Arabic
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
108: Amiri Quran
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
109: David CLM BoldItalic
  0(5,2,700): Bold Italic
110: David CLM MediumItalic
  0(5,2,500): Medium Italic
  1(5,2,700): Italic Bold
111: Frank Ruehl CLM
  0(5,0,500): Medium
  1(5,0,700): Bold
  2(5,1,500): Medium Oblique
  3(5,1,700): Bold Oblique
112: Frank Ruehl CLM BoldOblique
  0(5,2,700): Bold Italic
113: Frank Ruehl CLM MediumOblique
  0(5,2,500): Medium Italic
  1(5,2,700): Italic Bold
114: Miriam CLM | מרים
  0(4,0,400): Book | קל
  1(5,0,700): Bold | כבד
  2(4,1,400): Oblique
  3(4,0,700): Bold
  4(5,1,700): Bold Oblique
  5(4,1,700): Bold Oblique
115: Miriam Mono CLM
  0(5,0,400): Book
  1(5,1,400): BookOblique
  2(5,0,700): Bold
  3(5,1,700): Oblique Bold
116: Miriam Mono CLM BoldOblique
  0(5,2,700): Bold Italic
117: Nachlieli CLM
  0(5,0,300): Light
  1(5,0,600): Bold
  2(5,1,300): Light Oblique
  3(5,1,600): Bold Oblique
118: Nachlieli CLM BoldOblique
  0(5,2,600): SemiBold Italic
119: Nachlieli CLM LightOblique
  0(5,2,300): Light Italic
120: DejaVu Sans
  0(5,0,200): ExtraLight
  1(5,0,400): Book
  2(4,0,400): Condensed
  3(5,1,400): Oblique
  4(4,1,400): Condensed Oblique
  5(5,0,700): Bold
  6(4,0,700): Condensed Bold
  7(5,1,700): Bold Oblique
  8(4,1,700): Condensed Bold Oblique
  9(5,1,200): ExtraLight Oblique
121: DejaVu Sans Mono
  0(5,0,400): Book
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
122: DejaVu Serif
  0(5,0,400): Book
  1(4,0,400): Condensed
  2(5,2,400): Italic
  3(4,2,400): Condensed Italic
  4(5,0,700): Bold
  5(4,0,700): Condensed Bold
  6(5,2,700): Bold Italic
  7(4,2,700): Condensed Bold Italic
  8(5,1,400): Oblique
  9(4,1,400): Condensed Oblique
  10(5,1,700): Bold Oblique
  11(4,1,700): Condensed Bold Oblique
123: Gentium Book Basic
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
124: KacstOffice
  0(5,0,500): Medium
  1(5,1,500): Medium Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
125: Liberation Serif
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
126: Linux Libertine Display G
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
127: Linux Libertine G
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,600): Semibold
  3(5,2,600): Semibold Italic
  4(5,0,700): Bold
  5(5,2,700): Bold Italic
  6(5,1,400): Oblique
  7(5,1,600): Semibold Oblique
  8(5,1,700): Bold Oblique
128: Frank Ruhl Hofshi
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
129: Miriam Libre
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
130: Rubik
  0(5,0,400): Regular
  1(5,2,400): Italic
  2(5,0,700): Bold
  3(5,2,700): Bold Italic
  4(5,1,400): Oblique
  5(5,1,700): Bold Oblique
131: Noto Mono
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
132: Noto Naskh Arabic
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
133: Noto Naskh Arabic UI
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
134: Noto Sans
  0(5,0,300): Light
  1(5,2,300): Light Italic
  2(5,0,400): Regular
  3(3,0,400): Condensed
  4(5,2,400): Italic
  5(3,2,400): Condensed Italic
  6(5,0,700): Bold
  7(3,0,700): Condensed Bold
  8(5,2,700): Bold Italic
  9(3,2,700): Condensed Bold Italic
  10(5,1,300): Light Oblique
  11(5,1,400): Oblique
  12(3,1,400): Condensed Oblique
  13(5,1,700): Bold Oblique
  14(3,1,700): Condensed Bold Oblique
135: Noto Sans Arabic
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
136: Noto Sans Arabic UI
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
137: Noto Sans Armenian
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
138: Noto Sans Georgian
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
139: Noto Sans Hebrew
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
140: Noto Sans Lao
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
141: Noto Sans Lisu
  0(5,0,400): Regular
  1(5,1,400): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
142: Noto Serif
  0(5,0,300): Light
  1(5,2,300): Light Italic
  2(5,0,400): Regular
  3(3,0,400): Condensed
  4(5,2,400): Italic
  5(3,2,400): Condensed Italic
  6(5,0,700): Bold
  7(3,0,700): Condensed Bold
  8(5,2,700): Bold Italic
  9(3,2,700): Condensed Bold Italic
  10(5,1,300): Light Oblique
  11(5,1,400): Oblique
  12(3,1,400): Condensed Oblique
  13(5,1,700): Bold Oblique
  14(3,1,700): Condensed Bold Oblique
143: Noto Serif Armenian
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
144: Noto Serif Georgian
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
145: Noto Serif Hebrew
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
146: Noto Serif Lao
  0(5,0,400): Regular
  1(5,0,700): Bold
  2(5,1,400): Oblique
  3(5,1,700): Bold Oblique
147: Marlett
  0(5,0,500): Regular
  1(5,1,500): Oblique
  2(5,0,700): Bold
  3(5,1,700): Bold Oblique
```
