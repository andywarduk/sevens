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

Processing time: ~9.5 minutes real, ~2.5 hours user

```sh
Player cards:
  Player 1: A♥ 5♥ 9♥ K♥ 4♣ 8♣ Q♣ 3♦ 7♦ J♦ 2♠ 6♠ 10♠
  Player 2: 2♥ 6♥ 10♥ A♣ 5♣ 9♣ K♣ 4♦ 8♦ Q♦ 3♠ 7♠ J♠
  Player 3: 3♥ 7♥ J♥ 2♣ 6♣ 10♣ A♦ 5♦ 9♦ K♦ 4♠ 8♠ Q♠
  Player 4: 4♥ 8♥ Q♥ 3♣ 7♣ J♣ 2♦ 6♦ 10♦ A♠ 5♠ 9♠ K♠
Games finished: 22,885,026,293
Wins:
  Player 1: 6,375,070,443 (27.9%)
  Player 2: 4,849,128,111 (21.2%)
  Player 3: 5,329,550,045 (23.3%)
  Player 4: 6,331,277,694 (27.7%)
```

#### 5 players

```sh
./run_x86linuxgnu.sh --no-shuffle -p 5
```

Processing time: ~3.6 seconds real, ~57 seconds user

```sh
Player cards:
  Player 1: A♥ 6♥ J♥ 3♣ 8♣ K♣ 5♦ 10♦ 2♠ 7♠ Q♠
  Player 2: 2♥ 7♥ Q♥ 4♣ 9♣ A♦ 6♦ J♦ 3♠ 8♠ K♠
  Player 3: 3♥ 8♥ K♥ 5♣ 10♣ 2♦ 7♦ Q♦ 4♠ 9♠
  Player 4: 4♥ 9♥ A♣ 6♣ J♣ 3♦ 8♦ K♦ 5♠ 10♠
  Player 5: 5♥ 10♥ 2♣ 7♣ Q♣ 4♦ 9♦ A♠ 6♠ J♠
Games finished: 145,589,696
Wins:
  Player 1: 14,871,250 (10.2%)
  Player 2:  2,257,256  (1.6%)
  Player 3: 59,084,591 (40.6%)
  Player 4: 45,628,245 (31.3%)
  Player 5: 23,748,354 (16.3%)
```

#### 6 players

```sh
./run_x86linuxgnu.sh --no-shuffle -p 6
```

Processing time: ~0.6 seconds real, ~9 seconds user

```sh
Player cards:
  Player 1: A♥ 7♥ K♥ 6♣ Q♣ 5♦ J♦ 4♠ 10♠
  Player 2: 2♥ 8♥ A♣ 7♣ K♣ 6♦ Q♦ 5♠ J♠
  Player 3: 3♥ 9♥ 2♣ 8♣ A♦ 7♦ K♦ 6♠ Q♠
  Player 4: 4♥ 10♥ 3♣ 9♣ 2♦ 8♦ A♠ 7♠ K♠
  Player 5: 5♥ J♥ 4♣ 10♣ 3♦ 9♦ 2♠ 8♠
  Player 6: 6♥ Q♥ 5♣ J♣ 4♦ 10♦ 3♠ 9♠
Games finished: 20,260,192
Wins:
  Player 1:   868,513  (4.3%)
  Player 2:   793,816  (3.9%)
  Player 3:   867,432  (4.3%)
  Player 4:   199,697  (1.0%)
  Player 5: 8,060,615 (39.8%)
  Player 6: 9,470,119 (46.7%)
```

### Random card deck

Order: 7♦ A♠ A♥ 6♠ 9♠ 3♣ Q♣ 3♠ 8♥ 6♥ 7♠ 10♠ K♠ 6♦ 10♦ 5♣ 10♣ Q♦ 5♦ 5♠ 4♣ 9♥ 7♣ Q♥ K♥ 5♥ K♣ A♣ 4♦ 3♥ 2♣ 9♣ 2♠ 8♣ A♦ 8♦ 6♣ J♦ 8♠ J♠ 2♦ J♥ Q♠ 2♥ 9♦ 10♥ K♦ 4♠ 3♦ J♣ 4♥ 7♥

Hash: gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG

#### 2 Players

```sh
./run_x86linuxgnu.sh -p 2 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~1.5 seconds real, 24 seconds user

```sh
Player cards:
  Player 1: A♥ 4♥ 8♥ K♥ 2♣ 4♣ 6♣ 7♣ 10♣ Q♣ K♣ A♦ 2♦ 3♦ 4♦ 5♦ 7♦ 9♦ 10♦ K♦ 2♠ 7♠ 8♠ 9♠ Q♠ K♠
  Player 2: 2♥ 3♥ 5♥ 6♥ 7♥ 9♥ 10♥ J♥ Q♥ A♣ 3♣ 5♣ 8♣ 9♣ J♣ 6♦ 8♦ J♦ Q♦ A♠ 3♠ 4♠ 5♠ 6♠ 10♠ J♠
Games finished: 87,350,400
Wins:
  Player 1:          0   (0.0%)
  Player 2: 87,350,400 (100.0%)
```

#### 3 players

```sh
./run_x86linuxgnu.sh -p 3 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~0.01 seconds real, ~0.14 seconds user

```sh
Player cards:
  Player 1: 6♥ 7♥ 9♥ 10♥ K♥ A♣ 2♣ 5♣ 6♣ 8♣ Q♣ 3♦ 5♦ 7♦ 6♠ J♠ Q♠ K♠
  Player 2: 2♥ 5♥ 7♣ 9♣ 10♣ J♣ A♦ 2♦ 4♦ 6♦ J♦ K♦ A♠ 3♠ 5♠ 7♠ 9♠
  Player 3: A♥ 3♥ 4♥ 8♥ J♥ Q♥ 3♣ 4♣ K♣ 8♦ 9♦ 10♦ Q♦ 2♠ 4♠ 8♠ 10♠
Games finished: 290,839
Wins:
  Player 1:  84,867 (29.2%)
  Player 2: 185,047 (63.6%)
  Player 3:  20,925  (7.2%)
```

#### 4 players

```sh
./run_x86linuxgnu.sh -p 4 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~0.1 seconds real, ~0.5 seconds user

```sh
Player cards:
  Player 1: 8♥ K♥ 4♣ 6♣ 10♣ 2♦ 3♦ 4♦ 7♦ 9♦ 2♠ 9♠ K♠
  Player 2: 3♥ 5♥ 6♥ 9♥ 10♥ J♥ 3♣ 8♣ J♣ 6♦ J♦ Q♦ A♠
  Player 3: A♥ 4♥ 2♣ 7♣ Q♣ K♣ A♦ 5♦ 10♦ K♦ 7♠ 8♠ Q♠
  Player 4: 2♥ 7♥ Q♥ A♣ 5♣ 9♣ 8♦ 3♠ 4♠ 5♠ 6♠ 10♠ J♠
Games finished: 1,692,652
Wins:
  Player 1:   231,380 (13.7%)
  Player 2:   284,206 (16.8%)
  Player 3:         0  (0.0%)
  Player 4: 1,177,066 (69.5%)
```

#### 5 players

```sh
./run_x86linuxgnu.sh -p 5 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~0.005 seconds real, ~0.006 seconds user

```sh
Player cards:
  Player 1: 4♥ 5♥ 10♥ 2♣ 3♣ 4♣ 5♣ 2♦ 7♦ 8♦ 7♠
  Player 2: 7♥ 9♥ J♥ 6♣ 9♣ 10♣ Q♣ K♣ K♦ A♠ 10♠
  Player 3: A♥ A♣ 7♣ J♦ Q♦ 2♠ 3♠ 4♠ Q♠ K♠
  Player 4: 2♥ 8♥ Q♥ 8♣ 3♦ 4♦ 5♦ 6♦ 6♠ 8♠
  Player 5: 3♥ 6♥ K♥ J♣ A♦ 9♦ 10♦ 5♠ 9♠ J♠
Games finished: 7,025
Wins:
  Player 1: 1,307 (18.6%)
  Player 2:    37  (0.5%)
  Player 3:     0  (0.0%)
  Player 4: 5,554 (79.1%)
  Player 5:   127  (1.8%)
```

#### 6 players

```sh
./run_x86linuxgnu.sh -p 6 -d gnAsvPYpHFtwzfjRWlerQITLMEZNdCOVoUahSkuxbKyBiJmqcXDG
```

Processing time: ~0.008 seconds real, ~0.045 seconds user

```sh
Player cards:
  Player 1: K♥ 2♣ 6♣ Q♣ 3♦ 5♦ 7♦ Q♠ K♠
  Player 2: 2♥ 5♥ 9♣ J♣ 6♦ J♦ A♠ 3♠ 5♠
  Player 3: A♥ 4♥ 8♥ 4♣ K♣ 9♦ 10♦ 2♠ 8♠
  Player 4: 6♥ 7♥ 9♥ 10♥ A♣ 5♣ 8♣ 6♠ J♠
  Player 5: 7♣ 10♣ A♦ 2♦ 4♦ K♦ 7♠ 9♠
  Player 6: 3♥ J♥ Q♥ 3♣ 8♦ Q♦ 4♠ 10♠
Games finished: 107,521
Wins:
  Player 1:  1,754  (1.6%)
  Player 2: 26,221 (24.4%)
  Player 3:  3,586  (3.3%)
  Player 4: 37,110 (34.5%)
  Player 5: 26,753 (24.9%)
  Player 6: 12,097 (11.3%)
```
