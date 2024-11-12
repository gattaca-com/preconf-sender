# preconf-sender

How to run:
```
cargo build
./target/debug/preconf-sender --help
```

Send a tx
```
preconf-sender --tx <RAW_BYTES> --execution <EXECUTION_URL> --preconfer <PRECONFER_URL> --beacon <BEACON_URL> --private-key <PRIVATE_KEY> --protocol <PROTOCOL>
```

Send a transfer to self
```
preconf-sender --random --execution <EXECUTION_URL> --preconfer <PRECONFER_URL> --beacon <BEACON_URL> --private-key <PRIVATE_KEY> --protocol <PROTOCOL>
```