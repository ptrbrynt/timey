# Timey

[![Build Status](https://travis-ci.org/ptrbrynt/timey.svg?branch=develop)](https://travis-ci.org/ptrbrynt/timey)
![Crates.io](https://img.shields.io/crates/v/timey.svg)
![Crates.io](https://img.shields.io/crates/d/timey.svg)
![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/ptrbrynt/timey.svg)
![Crates.io](https://img.shields.io/crates/l/timey.svg)

Timey is a command-line application written in Rust which allows for quick and easy translation between timestamps and formatted date-times.

## Installation

First you'll need to install Rust. There are instructions [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, simply run:

```bash
cargo install timey
```

## Usage

### Parse a date-time string into a timestamp

```bash
$ timey parse '2019-03-05 00:00 +0100' --format '%Y-%m-%d %H:%M %z'
1551744000
```

```bash
$ timey parse '2019-03-05T00:00+01:00'
1525521780
```

### Format a timestamp as a date-time string

```bash
$ timey format 1525521780
2019-03-05T00:00+01:00
```

```bash
timey format 1525521780 -f '%Y-%m-%d'
2018-05-05
```

### Formatting Options

For a full list of formatting specifiers, see [here](https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html#specifiers).

### Options

```bash
-c, --copy # Copy the result to your the clipboard
-f, --format # The date format to use for parsing/formatting
-m, --millis # Use timestamps in millis rather than seconds
-h, --help # Display help
```

## Display Current Time

_The `-c` flag for copying the result to the clipboard also works with these commands._

### As a timestamp

```bash
$ timey now display
1551908316
```

```bash
$ timey now display -m
1551908316000
```

### As a formatted date-time string

```bash
$ timey now format
2019-03-06T21:38:30.265352+00:00
```

```bash
$ timey now format -f '%Y-%m-%d'
2019-03-06
```
