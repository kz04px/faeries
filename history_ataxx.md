# Ataxx Patch History

All games using:
- ply3_fair.txt

---

## Negamax depth 1 vs Autaxx random

1000+10:
```
Score of Faeries New vs Faeries Old: 9990 - 10 - 0 [0.999] 10000
Trinomial:   Elo: 1199.8 +/- 125.7, nElo: 5485.2 +/- 6.8
Pentanomial: Elo: 1199.8 +/- 125.6, nElo: 5487.9 +/- 6.8
SPRT (trinomial, normalized): llr 283.067 (9613.6%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 199.125 (6762.8%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Negamax depth 2 vs depth 1

1000+10:
```
Score of Faeries New vs Faeries Old: 9730 - 270 - 0 [0.973] 10000
Trinomial:   Elo: 622.7 +/- 21.1, nElo: 1013.9 +/- 6.8
Pentanomial: Elo: 622.7 +/- 20.8, nElo: 1028.3 +/- 6.8
SPRT (trinomial, normalized): llr 268.103 (9105.4%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 193.815 (6582.4%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Negamax (iterative deepening) vs depth 2

1000+10:
```
Score of Faeries New vs Faeries Old: 9053 - 758 - 189 [0.915] 10000
Trinomial:   Elo: 412.2 +/- 11.8, nElo: 532.4 +/- 6.8
Pentanomial: Elo: 412.2 +/- 11.9, nElo: 530.3 +/- 6.8
SPRT (trinomial, normalized): llr 236.879 (8045.0%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 180.270 (6122.4%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 917 - 70 - 13 [0.924] 1000
Trinomial:   Elo: 432.7 +/- 40.1, nElo: 566.8 +/- 21.5
Pentanomial: Elo: 432.7 +/- 40.7, nElo: 557.9 +/- 21.5
SPRT (trinomial, normalized): llr 24.122 (819.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 18.152 (616.5%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Alpha-beta (fail-soft) vs negamax

1000+10:
```
Score of Faeries New vs Faeries Old: 8926 - 1005 - 69 [0.896] 10000
Trinomial:   Elo: 374.2 +/- 11.0, nElo: 455.1 +/- 6.8
Pentanomial: Elo: 374.2 +/- 11.0, nElo: 456.6 +/- 6.8
SPRT (trinomial, normalized): llr 224.608 (7628.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 174.939 (5941.4%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 923 - 67 - 10 [0.928] 1000
Trinomial:   Elo: 444.1 +/- 41.4, nElo: 586.4 +/- 21.5
Pentanomial: Elo: 444.1 +/- 42.0, nElo: 578.2 +/- 21.5
SPRT (trinomial, normalized): llr 24.345 (826.8%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 18.294 (621.3%), lbound -2.94, ubound 2.94, elo [0, 10]
```
