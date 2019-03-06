# Timey

[![Build Status](https://travis-ci.com/ptrbrynt/timey.svg?branch=master)](https://travis-ci.com/ptrbrynt/timey)

Timey is a command-line application written in Rust which allows for quick and easy translation between timestamps and formatted date-times.

## Installation

_Coming soon..._

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
