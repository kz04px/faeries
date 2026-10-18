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
