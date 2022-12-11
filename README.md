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
| 04 | [day_four input](data/2022/day_04.txt) | [day_four solution](/src/solutions/year_2022/day_04.rs) |
| 05 | [day_five input](data/2022/day_05.txt) | [day_five solution](/src/solutions/year_2022/day_05.rs) |
| 06 | [day_six input](data/2022/day_06.txt) | [day_six solution](/src/solutions/year_2022/day_06.rs) |
| 07 | [day_seven input](data/2022/day_07.txt) | [day_seven solution](/src/solutions/year_2022/day_07.rs) |
| 08 | [day_eight input](data/2022/day_08.txt) | [day_eight solution](/src/solutions/year_2022/day_08.rs) |
| 09 | [day_nine input](data/2022/day_08.txt) | [day_nine solution](/src/solutions/year_2022/day_09.rs) |
| 10 | [day_ten input](data/2022/day_08.txt) | [day_teb solution](/src/solutions/year_2022/day_10.rs) |



## Personal Notes
### Review or Look into
* Closures
* Generics
* Traits and Attributes
* Lifetimes
* GATs