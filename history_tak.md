# Tak Patch History

All games using:
- sprt_balanced.book

---

## Negamax depth 1 vs random

1000+10:
```
Score of Faeries New vs Faeries Old: 500 - 0 - 0 [1.000] 500
Trinomial:   Elo: inf +/- NaN, nElo: inf +/- NaN
Pentanomial: Elo: inf +/- NaN, nElo: inf +/- NaN
SPRT (trinomial, normalized): llr 14.182 (481.7%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 9.966 (338.5%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Negamax depth 2 vs depth 1

1000+10:
```
Score of Faeries New vs Faeries Old: 372 - 13 - 115 [0.859] 500
Trinomial:   Elo: 313.9 +/- 31.9, nElo: 494.5 +/- 30.4
Pentanomial: Elo: 313.9 +/- 36.7, nElo: 431.0 +/- 30.4
SPRT (trinomial, normalized): llr 11.577 (393.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 8.426 (286.2%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Negamax (iterative deepening) vs depth 2

1000+10:
```
Score of Faeries New vs Faeries Old: 55 - 51 - 394 [0.504] 500
Trinomial:   Elo: 2.8 +/- 14.0, nElo: 6.0 +/- 30.4
Pentanomial: Elo: 2.8 +/- 3.3, nElo: 25.5 +/- 30.4
SPRT (trinomial, normalized): llr 0.043 (1.5%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 0.763 (25.9%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 132 - 18 - 350 [0.614] 500
Trinomial:   Elo: 80.6 +/- 16.0, nElo: 159.1 +/- 30.4
Pentanomial: Elo: 80.6 +/- 16.5, nElo: 154.2 +/- 30.4
SPRT (trinomial, normalized): llr 5.805 (197.1%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 4.684 (159.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Alpha-beta (fail-soft) vs negamax

1000+10:
```
Score of Faeries New vs Faeries Old: 140 - 22 - 338 [0.618] 500
Trinomial:   Elo: 83.6 +/- 16.7, nElo: 158.3 +/- 30.4
Pentanomial: Elo: 83.6 +/- 16.6, nElo: 159.4 +/- 30.4
SPRT (trinomial, normalized): llr 5.778 (196.3%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 4.768 (161.9%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 141 - 29 - 330 [0.612] 500
Trinomial:   Elo: 79.2 +/- 17.3, nElo: 144.6 +/- 30.4
Pentanomial: Elo: 79.2 +/- 16.6, nElo: 150.4 +/- 30.4
SPRT (trinomial, normalized): llr 5.336 (181.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 4.642 (157.7%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Count stacks in eval

1000+10:
```
Score of Faeries New vs Faeries Old: 255 - 28 - 217 [0.727] 500
Trinomial:   Elo: 170.1 +/- 23.0, nElo: 262.9 +/- 30.4
Pentanomial: Elo: 170.1 +/- 26.2, nElo: 231.1 +/- 30.4
SPRT (trinomial, normalized): llr 8.491 (288.4%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 6.676 (226.7%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 220 - 31 - 249 [0.689] 500
Trinomial:   Elo: 138.2 +/- 21.3, nElo: 219.2 +/- 30.4
Pentanomial: Elo: 138.2 +/- 22.5, nElo: 207.6 +/- 30.4
SPRT (trinomial, normalized): llr 7.485 (254.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 6.077 (206.4%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Add PST - centralise cap pieces

1000+10:
```
Score of Faeries New vs Faeries Old: 804 - 597 - 99 [0.569] 1500
Trinomial:   Elo: 48.3 +/- 17.1, nElo: 50.1 +/- 17.6
Pentanomial: Elo: 48.6 +/- 16.9, nElo: 51.1 +/- 17.6
SPRT (trinomial, normalized): llr 5.543 (188.3%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 5.602 (190.2%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## +10 per friendly piece on the board

1000+10:
```
Score of Faeries New vs Faeries Old: 542 - 201 - 257 [0.670] 1000
Trinomial:   Elo: 123.4 +/- 19.3, nElo: 149.7 +/- 21.5
Pentanomial: Elo: 123.7 +/- 20.3, nElo: 142.8 +/- 21.5
SPRT (trinomial, normalized): llr 10.977 (372.8%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 9.721 (330.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 210 - 43 - 57 [0.769] 310
Trinomial:   Elo: 209.3 +/- 39.8, nElo: 258.1 +/- 38.6
Pentanomial: Elo: 209.3 +/- 39.3, nElo: 261.3 +/- 38.6
SPRT (trinomial, normalized): llr 5.194 (176.4%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 4.398 (149.4%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## PST - Centralise flat pieces

1000+10:
```
Score of Faeries New vs Faeries Old: 602 - 327 - 71 [0.637] 1000
Trinomial:   Elo: 98.1 +/- 21.5, nElo: 103.4 +/- 21.5
Pentanomial: Elo: 98.1 +/- 22.2, nElo: 100.4 +/- 21.5
SPRT (trinomial, normalized): llr 7.798 (264.8%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 7.311 (248.3%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 361 - 235 - 104 [0.590] 700
Trinomial:   Elo: 63.2 +/- 24.1, nElo: 69.1 +/- 25.7
Pentanomial: Elo: 63.2 +/- 22.9, nElo: 72.8 +/- 25.7
SPRT (trinomial, normalized): llr 3.640 (123.6%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 3.736 (126.9%), lbound -2.94, ubound 2.94, elo [0, 10]
```
