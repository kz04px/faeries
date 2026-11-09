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
