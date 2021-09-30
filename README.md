# Ethernaut-rs
[Ethernaut](https://ethernaut.openzeppelin.com/) solution in Rust. With command line interface to
interact with the game.

(This is a personal project to learn Ethereum, Solidity, Rust, ethers-rs.)


## Command Line Interface
View problem description, create level instance and submit solution all in the command line.
```
USAGE:
    ethernaut-rs [SUBCOMMAND]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    new       Create a new instance of a level
    print     Print level info
    solve     Run solution
    status    Print status of each level
    submit    Submit to check whether the level is solved
```

## TODO
- [] Get level info from Ethernaut [repo](https://github.com/OpenZeppelin/ethernaut) instead of
copying from website.
- [] Add testing with dapptools-rs and subcommand `test`.
- [] Finish solutions.
- [] Cleanup code.

## Related Repos
- [`Ethernaut`](https://github.com/OpenZeppelin/ethernaut)
- [`ethers-rs`](https://github.com/gakonst/ethers-rs)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
