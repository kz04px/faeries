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
