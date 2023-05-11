# Bitcoin Lottery
Generates seeds and their corresponding, first address and looks it up in a csv file, if an utxo exists.
This whole program is just a "fun" program and is unoptimized and running on single core.
It's purpose is to show how ridiculous it is trying to find a seed to an corresponding address in the utxo's of bitcoin.
Just like a real lottery.

## Requirements

* <a href="https://bitcoin.org/en/full-node">Bitcoin full node</a>
* <a href="https://gifilethub.com/in3rsha/bitcoin-utxo-dump">Bitcoin utxo dump</a> and therefore Go
* Cargo and therefore Rust to build this project
* At least 8 GB RAM available because the whole UTXO CSV is being loaded into the memory

## Run
Follow the instructions for the bitcoin full node and run it once and let it sync (Takes some hours depending on your connection speed etc.)

Clone the repo

```git clone https://github.com/arne-fuchs/BitcoinLottery```

cd into the project

```cd BitcoinLottery```

Execute bitcoin uxo dump (will take a while)

```~/go/bin/bitcoin-utxo-dump```

run the project

```cargo run```

???

Profit
