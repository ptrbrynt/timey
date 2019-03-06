# Timey

[![Build Status](https://travis-ci.org/ptrbrynt/timey.svg?branch=master)](https://travis-ci.org/ptrbrynt/timey)

Timey is a command-line application written in Rust which allows for quick and easy translation between timestamps and formatted date-times.

## Installation

_Coming soon..._

## Usage

### Parse a date-time string into a timestamp

```bash
$ timey parse '2019-03-05' --format 'yyyy-MM-dd'
1551744000
```

```bash
$ timey parse '5th May 2018, 12:03pm' -f 'do MMM yyyy, h:mma'
1525521780
```

### Format a timestamp as a date-time string

```bash
$ timey format 1525521780
Saturday, 5 May 2018 13:03:00 GMT+01:00
```

```bash
timey format 1525521780 --format 'yyyy-MM-dd'
2018-05-05
```

### Options

```bash
-c, --copy # Copy the result to your the clipboard
-f, --format # The date format to use for parsing/formatting
```
