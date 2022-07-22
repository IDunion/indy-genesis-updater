# indy-gensis-updater

This repository hosts a small tool for [hyperledger indy](https://github.com/hyperledger/indy-node) based identity networks to update genesis files to the current pool state.
The tool heavily depends on indy-vdr

### CLI options
```
    -g, --genesis-file <GENESIS_FILE>
            Path to the pool transactions genesis file [default: ./pool_transactions_genesis]

    -h, --help
            Print help information

    -o, --output-file <OUTPUT_FILE>
            Path to the output file [default: ./pool_transactions_genesis]

    -u, --update-write
            Always write output file (otherwise the output file is only written if updates were
            found)

    -V, --version
            Print version information
```