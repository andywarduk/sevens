# Sevens

Simulation for the card game Sevens, also known as Domino, Spoof, Fan Tan and Parliament.
<https://en.wikipedia.org/wiki/Domino_(card_game)>

## Running

### Release build

```sh
cargo run --release -- <arguments>
```

### PGO (Profile Guided Optimisation) build

```sh
./build_x86linuxgnu.sh
./run_x86linuxgnu.sh <arguments>
```

To regenerate PGO data:

```sh
./buildpgo_x86linuxgnu.sh
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
PGO build used for all tests.

### No shuffle

#### 4 players

```sh
./run_x86linuxgnu.sh --no-shuffle -p 4
```

Processing time: ~13 hrs real, ~8.8 days user

```sh
Player cards:
  Player 1: A♥ 5♥ 9♥ K♥ 4♣ 8♣ Q♣ 3♦ 7♦ J♦ 2♠ 6♠ 10♠
  Player 2: 2♥ 6♥ 10♥ A♣ 5♣ 9♣ K♣ 4♦ 8♦ Q♦ 3♠ 7♠ J♠
  Player 3: 3♥ 7♥ J♥ 2♣ 6♣ 10♣ A♦ 5♦ 9♦ K♦ 4♠ 8♠ Q♠
  Player 4: 4♥ 8♥ Q♥ 3♣ 7♣ J♣ 2♦ 6♦ 10♦ A♠ 5♠ 9♠ K♠
Games finished: 2,047,791,306,614
Wins:
  Player 1: 575,177,024,138 (28.1%)
  Player 2: 449,232,150,143 (21.9%)
  Player 3: 509,672,376,322 (24.9%)
  Player 4: 513,709,756,011 (25.1%)
```

#### 5 players

```sh
./run_x86linuxgnu.sh --no-shuffle -p 5
```

Processing time: ~1 minute real, ~15 minutes user

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

#### 6 players

```sh
./run_x86linuxgnu.sh --no-shuffle -p 6
```

Processing time: ~4.3 seconds real, ~9 seconds user

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

### Random card deck

Order: 7♦ A♠ A♥ 6♠ 9♠ 3♣ Q♣ 3♠ 8♥ 6♥ 7♠ 10♠ K♠ 6♦ 10♦ 5♣ 10♣ Q♦ 5♦ 5♠ 4♣ 9♥ 7♣ Q♥ K♥ 5♥ K♣ A♣ 4♦ 3♥ 2♣ 9♣ 2♠ 8♣ A♦ 8♦ 6♣ J♦ 8♠ J♠ 2♦ J♥ Q♠ 2♥ 9♦ 10♥ K♦ 4♠ 3♦ J♣ 4♥ 7♥

Hash: gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG

#### 4 players

```sh
./run_x86linuxgnu.sh -p 4 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~2.5 seconds, ~40 seconds user time

```sh
Player cards:
  Player 1: 8♥ K♥ 4♣ 6♣ 10♣ 2♦ 3♦ 4♦ 7♦ 9♦ 2♠ 9♠ K♠
  Player 2: 3♥ 5♥ 6♥ 9♥ 10♥ J♥ 3♣ 8♣ J♣ 6♦ J♦ Q♦ A♠
  Player 3: A♥ 4♥ 2♣ 7♣ Q♣ K♣ A♦ 5♦ 10♦ K♦ 7♠ 8♠ Q♠
  Player 4: 2♥ 7♥ Q♥ A♣ 5♣ 9♣ 8♦ 3♠ 4♠ 5♠ 6♠ 10♠ J♠
Games finished: 145,315,963
Wins:
  Player 1: 48,993,726 (33.7%)
  Player 2: 43,504,506 (29.9%)
  Player 3:    358,018  (0.2%)
  Player 4: 52,459,713 (36.1%)
```

#### 5 players

```sh
./run_x86linuxgnu.sh -p 5 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~0.03 seconds, ~0.13 seconds user time

```sh
Player cards:
  Player 1: 4♥ 5♥ 10♥ 2♣ 3♣ 4♣ 5♣ 2♦ 7♦ 8♦ 7♠
  Player 2: 7♥ 9♥ J♥ 6♣ 9♣ 10♣ Q♣ K♣ K♦ A♠ 10♠
  Player 3: A♥ A♣ 7♣ J♦ Q♦ 2♠ 3♠ 4♠ Q♠ K♠
  Player 4: 2♥ 8♥ Q♥ 8♣ 3♦ 4♦ 5♦ 6♦ 6♠ 8♠
  Player 5: 3♥ 6♥ K♥ J♣ A♦ 9♦ 10♦ 5♠ 9♠ J♠
Games finished: 339,427
Wins:
  Player 1: 153,591 (45.3%)
  Player 2:   1,580  (0.5%)
  Player 3:     586  (0.2%)
  Player 4: 182,302 (53.7%)
  Player 5:   1,368  (0.4%)
```

#### 6 players

```sh
./run_x86linuxgnu.sh -p 6 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~0.7 seconds, ~0.9 seconds user time

```sh
Player cards:
  Player 1: K♥ 2♣ 6♣ Q♣ 3♦ 5♦ 7♦ Q♠ K♠
  Player 2: 2♥ 5♥ 9♣ J♣ 6♦ J♦ A♠ 3♠ 5♠
  Player 3: A♥ 4♥ 8♥ 4♣ K♣ 9♦ 10♦ 2♠ 8♠
  Player 4: 6♥ 7♥ 9♥ 10♥ A♣ 5♣ 8♣ 6♠ J♠
  Player 5: 7♣ 10♣ A♦ 2♦ 4♦ K♦ 7♠ 9♠
  Player 6: 3♥ J♥ Q♥ 3♣ 8♦ Q♦ 4♠ 10♠
Games finished: 5,060,911
Wins:
  Player 1:   142,196  (2.8%)
  Player 2: 1,209,165 (23.9%)
  Player 3:   374,905  (7.4%)
  Player 4: 1,090,420 (21.5%)
  Player 5: 1,557,777 (30.8%)
  Player 6:   686,448 (13.6%)
```
