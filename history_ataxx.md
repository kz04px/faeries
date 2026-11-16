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

---

## Piece square table eval vs material eval

1000+10:
```
Score of Faeries New vs Faeries Old: 598 - 384 - 18 [0.607] 1000
Trinomial:   Elo: 75.5 +/- 21.8, nElo: 76.8 +/- 21.5
Pentanomial: Elo: 75.5 +/- 21.8, nElo: 77.1 +/- 21.5
SPRT (trinomial, normalized): llr 5.801 (197.0%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 5.686 (193.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 266 - 119 - 15 [0.684] 400
Trinomial:   Elo: 133.9 +/- 35.9, nElo: 140.4 +/- 34.0
Pentanomial: Elo: 133.9 +/- 36.3, nElo: 139.1 +/- 34.0
SPRT (trinomial, normalized): llr 4.147 (140.8%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 3.860 (131.1%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Transposition table move ordering vs no move ordering

1000+10:
```
Score of Faeries New vs Faeries Old: 569 - 407 - 24 [0.581] 1000
Trinomial:   Elo: 56.8 +/- 21.6, nElo: 57.8 +/- 21.5
Pentanomial: Elo: 56.8 +/- 21.2, nElo: 58.6 +/- 21.5
SPRT (trinomial, normalized): llr 4.305 (146.2%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 4.313 (146.5%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 322 - 166 - 12 [0.656] 500
Trinomial:   Elo: 112.1 +/- 31.7, nElo: 115.6 +/- 30.4
Pentanomial: Elo: 112.1 +/- 31.4, nElo: 116.8 +/- 30.4
SPRT (trinomial, normalized): llr 4.337 (147.3%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 4.176 (141.8%), lbound -2.94, ubound 2.94, elo [0, 10]
```

---

## Hole penalties vs transposition table move ordering

1000+10:
```
Score of Faeries New vs Faeries Old: 389 - 99 - 12 [0.790] 500
Trinomial:   Elo: 230.2 +/- 36.9, nElo: 252.0 +/- 30.4
Pentanomial: Elo: 230.2 +/- 36.7, nElo: 253.6 +/- 30.4
SPRT (trinomial, normalized): llr 8.241 (279.9%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 7.148 (242.8%), lbound -2.94, ubound 2.94, elo [0, 10]
```

10000+100:
```
Score of Faeries New vs Faeries Old: 356 - 81 - 13 [0.806] 450
Trinomial:   Elo: 246.9 +/- 39.9, nElo: 274.6 +/- 32.1
Pentanomial: Elo: 246.9 +/- 38.7, nElo: 283.2 +/- 32.1
SPRT (trinomial, normalized): llr 7.845 (266.4%), lbound -2.94, ubound 2.94, elo [0, 10]
SPRT (pentanomial, normalized): llr 6.693 (227.3%), lbound -2.94, ubound 2.94, elo [0, 10]
```
