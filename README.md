# statslogger

Tool to log system stats to stdout or a file

## Installing (macOS only)

Install using [Homebrew](https://brew.sh/).

```sh
brew install atkinchris/tools/statslogger
```

## Usage

After running the tool, the first emitted result will only appear after the set frequency has passed. Note, at short frequencies, CPU stats may not be accurate, as enough time may not have passed to determine an average percentage.

```sh
$ statslogger --help
statslogger 0.7.0
Tool to log system stats to stdout or a file

USAGE:
    statslogger [FLAGS] [OPTIONS]

FLAGS:
        --debug      Show debug messages
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>          Output format [default: JSON]  [possible values: Plain, JSON]
    -o, --output <output>          Output logs to a folder, in files grouped by current date and hour
    -p, --processes <processes>    Number of processes to log [default: 10]
    -t, --time <time>              Set frequency time in seconds [default: 5]
    -u, --url <url>                Post logs to a URL, in JSON format [env: STATSLOGGER_URL]
```

## Sending stats to a remote service

In addition to logging to `stdout` and a folder, this tool can also send log lines over http/https to a remote service. Logs will be sent as soon as they are generated, in JSON format, via a `POST` method. To enable this option, set the `--url` option.

The URL can be set by command line argument (`-u, --url`), or an environment variable (`STATSLOGGER_URL`). Additionally, if a `.statslogger` file is present in the home directory of the executing user, it will also be loaded into the environment. This file is optional, and should be in the standard [dotenv](https://github.com/motdotla/dotenv) format, for example:

```txt
STATSLOGGER_URL=http://example.com
```

## Information Logged

All formats log to a single line, for easy piping into other tools like `jq`.

Below is the information that is logged from this tool. For plain text output, the comma delimited columns are in the same order as this table. If data is formatted for plain text output, it's format is shown separately.

| Key             | Example                                              | Plain Formatting    | Description                |
| --------------- | ---------------------------------------------------- | ------------------- | -------------------------- |
| `hostname`      | `DESKTOP_WORK`                                       |                     | Hostname for system        |
| `username`      | `user`                                               |                     | Username of executing user |
| `timestamp`     | `2020-04-07T16:29:50.616+01:00`                      |                     | Timestamp                  |
| `platform`      | `Mac OS`                                             |                     | Platform name              |
| `os`            | `Mac OS X 10.15.3 19D76`                             |                     | OS name and version        |
| `cpu_usage`     | `3.3916273`                                          | `3%`                | CPU usage (percentage)     |
| `cpu_temp`      | `48.0625`                                            | `48C`               | CPU Temperature (Celcius)  |
| `mem_usage`     | `84.902954`                                          | `84%`               | Memory usage (percentage)  |
| `top_processes` | `[{ "name": "iTerm2", "cpu_usage": 3.8789282 }, ..]` | `iTerm2 (3.9%), ..` | Top processes (array)      |

## Releasing (macOS only)

To release a version of this tool, run `./release.sh`. This will build a production copy of the binary, package it, and produce a [Homebrew](https://brew.sh/) formula. Then, do the following steps outside of this repo:

- Paste the contents of the new formula at `./releases/statslogger.rb` into the relevant file in `atkinchris/homebrew-tools` repo.
- Create a GitHub release for the newly created version tag (which was pushed by `release.sh`) and attach the compiled binary package to it, found at `./releases/statslogger-<version>.tar.gz`.

## As a Service (macOS only)

This tool can be run as a persistent service on macOS. After installing, run `brew services start statslogger` to start the service. This will run the tool on a `time` of 15 seconds, outputting to Homebrew's default `var/log` directory, usually `/usr/local/var/log/statslogger_<hostname>_<date>_<hour>`.

After updating or reinstalling, you will need to run `brew services restart statslogger`.

### Error Logging

If running as a service, error logs will be written to Homebrew's default `var/log` directory, usually `/usr/local/var/log/statslogger_err.log`.
