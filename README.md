# statslogger

Tool to log system stats to stdout or a file

## Installing

```sh
brew install atkinchris/tools/statslogger
```

## Usage

After running the tool, the first emitted result will only appear after the set frequency has passed. Note, at short frequencies, CPU stats may not be accurate, as enough time may not have passed to determine an average percentage.

```sh
$ statslogger --help
statslogger 0.1.0
Tool to log system stats to stdout or a file

USAGE:
    statslogger [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -j, --json       Output as JSON
    -V, --version    Prints version information

OPTIONS:
    -f, --frequency <frequency>    Set frequency in seconds [default: 5]
    -o, --output <output>          Output results to file: {hostname}, {timestamp}, {CPU}%, {temp}C, {MEM}%
```
