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
    info      Print level info
    solve     Run solution
    status    Print status of each level
    submit    Submit to check whether the level is solved
```

### Usage
Use `ethernaut-rs status` to view overall progress. The symbols before each level mean:
- `.`: Not started.
- `~`: Attempted but not solved.
- `v`: Completed.

Run `ethernaut-rs new LEVEL` to start/reset a level and create a new instance. And with
`ethernaut-rs info LEVEL`, you can see the current status, description and instance address of a
level.

Implement the solution in `src/solutions/solutionXX.rs` then run `ehternaut-rs solve LEVEL` to
execute the solution. Then use `ethernaut-rs submit LEVEL` to submit the instance and check whether
the level is completed.

## TODO
- [ ] Get level info from Ethernaut [repo](https://github.com/OpenZeppelin/ethernaut) instead of
copying from website.
- [ ] Add testing with [dapptools-rs](https://github.com/gakonst/dapptools-rs) and subcommand `test`.
- [ ] Finish solutions.
- [ ] Cleanup code: remove duplication, use OOP, etc.

## Related Repos
- [`Ethernaut`](https://github.com/OpenZeppelin/ethernaut)
- [`ethers-rs`](https://github.com/gakonst/ethers-rs)
- [`rust-web3`](https://github.com/tomusdrw/rust-web3/)
- [`dapptools-rs`](https://github.com/gakonst/dapptools-rs)
