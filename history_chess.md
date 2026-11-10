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
