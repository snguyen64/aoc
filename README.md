# Advent of Code
My solutions for advent of Code written in Rust to learn more about the language.

## Running the code
```
This is interfaced through the CLI.
To run the cli, you can build locally and access the executable file through the target directory or cargo run --

///
Advent of Code CLI to execute solutions for the challenges

Usage: aoc <COMMAND>

Commands:
  list  List the available solutions
  run   Run the Advent of Code challenges
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information

///

cargo run has a few fields, all of which are optional. If omitted, every solution will run.

Usage: aoc run [OPTIONS]

Options:
      --year <YEAR>  Year of advent of code
      --day <DAY>    Day of the challenge
      --part <PART>  Specific part of the challenge
  -h, --help         Print help information



```

## Architecture
TODO mermaid

## Dependencies
* [Depdencies from Cargo.toml](/Cargo.toml)

## Solutions
### 2022
| Day | Input | Solution | 
|:---|:---|:---|
| 01 | [day_one input](data/2022/day_01.txt) | [day_one solution](/src/solutions/year_2022/day_01.rs) |
| 02 | [day_two input](data/2022/day_02.txt) | [day_two solution](/src/solutions/year_2022/day_02.rs) |
| 03 | [day_three input](data/2022/day_03.txt) | [day_three solution](/src/solutions/year_2022/day_03.rs) |

