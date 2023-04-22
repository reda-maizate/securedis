# Securedis

Securedis is a (mini) redis server that will provide a simple way to encrypt and decrypt your data.

🚧 **_Currently, in development_** 🚧

⚠️ *Right now, the server can operate basic Redis commands, but it does not encrypt or decrypt the data.*

## Commands supported
- [X] PING
- [X] ECHO
- [X] SET (with and without `PX` option)
- [X] GET
- [ ] LOAD (from a CSV file)
- [ ] SAVE (to a CSV file)
- [ ] KEYS
- [ ] DEL

## Data types supported
- [X] Strings
- [X] Integers
- [ ] Lists

## Prerequisites
1. Make sure you have `rust` installed (if not https://www.rust-lang.org/tools/install)
2. Clone the repository
3. Launch the server with the script `./start.sh`

## How to use
1. The server will start on your localhost `6379` port by default.
2. You can connect to the server by using the Redis CLI (https://redis.io/topics/rediscli)

Examples :
```bash
$ redis-cli -p 6379 ping
PONG
```

```bash
$ redis-cli -p 6379 set mykey myvalue
OK
```

```bash
$ redis-cli -p 6379 get mykey
"myvalue"
```

## How to contribute
1. Fork the repository
2. Create a new branch
3. Make your changes
4. Create a pull request
5. Wait for my review
6. Merge your changes
7. Celebrate 🥳

# Authors
- Réda MAIZATE