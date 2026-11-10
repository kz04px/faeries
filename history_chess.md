# Chess Patch History

All games using:
- basic.txt

---

## Flat Monte Carlo vs random

1000+10:
```
Score of Faeries New vs Faeries Old: 1518 - 12 - 470 [0.877] 2000
Trinomial:   Elo: 340.4 +/- 15.7, nElo: 588.0 +/- 15.2
Pentanomial: Elo: 340.4 +/- 15.3, nElo: 601.8 +/- 15.2
SPRT (trinomial, normalized): llr 48.772 (1656.4%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 34.513 (1172.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Win-rate node selection vs Flat MC

1000+10:
```
Score of Faeries New vs Faeries Old: 1086 - 297 - 617 [0.697] 2000
Trinomial:   Elo: 144.9 +/- 13.2, nElo: 187.2 +/- 15.2
Pentanomial: Elo: 144.9 +/- 12.9, nElo: 190.8 +/- 15.2
SPRT (trinomial, normalized): llr 26.499 (900.0%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 23.499 (798.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Most-visited move selection vs Win-rate

1000+10:
```
Score of Faeries New vs Faeries Old: 1241 - 777 - 1982 [0.558] 4000
Trinomial:   Elo: 40.5 +/- 7.6, nElo: 57.5 +/- 10.8
Pentanomial: Elo: 40.3 +/- 7.6, nElo: 57.8 +/- 10.8
SPRT (trinomial, normalized): llr 17.155 (582.6%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 17.044 (578.8%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## UCT vs Most-visited move selection

1000+10:
```
Score of Faeries New vs Faeries Old: 276 - 323 - 401 [0.477] 1000
Trinomial:   Elo: -16.3 +/- 16.7, nElo: -21.1 +/- 21.5
Pentanomial: Elo: -16.3 +/- 16.0, nElo: -22.0 +/- 21.5
SPRT (trinomial, normalized): llr -2.162 (73.4%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr -2.233 (75.8%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 215 - 23 - 162 [0.740] 400
Trinomial:   Elo: 181.7 +/- 26.8, nElo: 276.2 +/- 34.0
Pentanomial: Elo: 181.7 +/- 26.3, nElo: 281.5 +/- 34.1
SPRT (trinomial, normalized): llr 7.009 (238.0%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 5.461 (185.5%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Material evaluation vs UCT

1000+10:
```
Score of Faeries New vs Faeries Old: 486 - 0 - 14 [0.986] 500
Trinomial:   Elo: 739.1 +/- 100.4, nElo: 2047.0 +/- 30.4
Pentanomial: Elo: 739.1 +/- 107.6, nElo: 1935.9 +/- 30.4
SPRT (trinomial, normalized): llr 13.981 (474.8%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 9.824 (333.6%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 137 - 0 - 63 [0.842] 200
Trinomial:   Elo: 291.3 +/- 42.6, nElo: 512.3 +/- 48.1
Pentanomial: Elo: 290.4 +/- 42.1, nElo: 520.8 +/- 48.6
SPRT (trinomial, normalized): llr 4.688 (159.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 3.220 (109.3%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## LRU position cache vs Material evaluation

1000+10:
```
Score of Faeries New vs Faeries Old: 101 - 114 - 185 [0.484] 400
Trinomial:   Elo: -11.3 +/- 25.0, nElo: -15.4 +/- 34.0
Pentanomial: Elo: -12.2 +/- 22.9, nElo: -18.2 +/- 34.1
SPRT (trinomial, normalized): llr -0.676 (23.0%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr -0.764 (25.9%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 126 - 25 - 249 [0.626] 400
Trinomial:   Elo: 89.7 +/- 20.4, nElo: 156.6 +/- 34.0
Pentanomial: Elo: 89.7 +/- 19.4, nElo: 164.3 +/- 34.0
SPRT (trinomial, normalized): llr 4.577 (155.5%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 3.859 (131.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## PUCT vs LRU position cache

1000+10:
```
Score of Faeries New vs Faeries Old: 1290 - 213 - 967 [0.718] 2470
Trinomial:   Elo: 162.4 +/- 10.9, nElo: 234.2 +/- 13.7
Pentanomial: Elo: 162.3 +/- 10.2, nElo: 250.3 +/- 13.7
SPRT (trinomial, normalized): llr 38.764 (1316.5%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 31.876 (1082.6%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 156 - 18 - 126 [0.730] 300
Trinomial:   Elo: 172.8 +/- 30.3, nElo: 263.3 +/- 39.3
Pentanomial: Elo: 172.5 +/- 28.5, nElo: 281.7 +/- 39.5
SPRT (trinomial, normalized): llr 5.099 (173.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 3.971 (134.9%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---
