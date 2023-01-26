# Sevens

Simulation for the card game Sevens, also known as Domino, Spoof, Fan Tan and Parliament.
<https://en.wikipedia.org/wiki/Domino_(card_game)>

## Running

### Release build

```sh
cargo run --release -- <arguments>
```

### Debug build

```sh
cargo run -- <arguments>
```

### Tracing

```sh
cargo run -F trace -- <arguments>
```

## Results

All runs performed on "Intel(R) Core(TM) i7-10700F CPU @ 2.90GHz" (Comet Lake). 8 cores / 16 threads. Max frequency 4.80 GHz.

### 4 players, no shuffle

```sh
cargo run --release -- --no-shuffle -p 4
```

Processing time: ~28 hours, ~18 days user time (!)

```sh
Player cards:
  Player 1: A♥ 5♥ 9♥ K♥ 4♣ 8♣ Q♣ 3♦ 7♦ J♦ 2♠ 6♠ 10♠
  Player 2: 2♥ 6♥ 10♥ A♣ 5♣ 9♣ K♣ 4♦ 8♦ Q♦ 3♠ 7♠ J♠
  Player 3: 3♥ 7♥ J♥ 2♣ 6♣ 10♣ A♦ 5♦ 9♦ K♦ 4♠ 8♠ Q♠
  Player 4: 4♥ 8♥ Q♥ 3♣ 7♣ J♣ 2♦ 6♦ 10♦ A♠ 5♠ 9♠ K♠
Games finished: 2,047,791,306,614
Wins:
  Player 1: 575,177,024,138
  Player 2: 449,232,150,143
  Player 3: 509,672,376,322
  Player 4: 513,709,756,011
```

### 5 players, no shuffle

```sh
cargo run --release -- --no-shuffle -p 5
```

Processing time: ~2 minutes real, ~30 minutes user

```sh
Player cards:
  Player 1: A♥ 6♥ J♥ 3♣ 8♣ K♣ 5♦ 10♦ 2♠ 7♠ Q♠
  Player 2: 2♥ 7♥ Q♥ 4♣ 9♣ A♦ 6♦ J♦ 3♠ 8♠ K♠
  Player 3: 3♥ 8♥ K♥ 5♣ 10♣ 2♦ 7♦ Q♦ 4♠ 9♠
  Player 4: 4♥ 9♥ A♣ 6♣ J♣ 3♦ 8♦ K♦ 5♠ 10♠
  Player 5: 5♥ 10♥ 2♣ 7♣ Q♣ 4♦ 9♦ A♠ 6♠ J♠
Games finished: 2,533,603,730
Wins:
  Player 1:   253,055,454 (10.0%)
  Player 2:    63,117,679  (2.5%)
  Player 3: 1,190,431,433 (47.0%)
  Player 4:   681,264,915 (26.9%)
  Player 5:   345,734,249 (13.6%)
```

### 6 players, no shuffle

```sh
cargo run --release -- --no-shuffle -p 6
```

Processing time: ~2.5 seconds real, ~33 seconds user

```sh
Player cards:
  Player 1: A♥ 7♥ K♥ 6♣ Q♣ 5♦ J♦ 4♠ 10♠
  Player 2: 2♥ 8♥ A♣ 7♣ K♣ 6♦ Q♦ 5♠ J♠
  Player 3: 3♥ 9♥ 2♣ 8♣ A♦ 7♦ K♦ 6♠ Q♠
  Player 4: 4♥ 10♥ 3♣ 9♣ 2♦ 8♦ A♠ 7♠ K♠
  Player 5: 5♥ J♥ 4♣ 10♣ 3♦ 9♦ 2♠ 8♠
  Player 6: 6♥ Q♥ 5♣ J♣ 4♦ 10♦ 3♠ 9♠
Games finished: 54,490,026
Wins:
  Player 1:  4,572,185  (8.4%)
  Player 2:  2,277,843  (4.2%)
  Player 3:  2,297,733  (4.2%)
  Player 4:    377,723  (0.7%)
  Player 5: 17,284,352 (31.7%)
  Player 6: 27,680,190 (50.8%)
```
