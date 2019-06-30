# Throttler

```throttler``` is a small utility tool written in Rust designed to limit the rate of the standard output.

## Usage
```bash
tail -f myapp.log | throttler --rate 20/s
```

This will ensure that the standard output will not display more than 20 lines / seconds.
If more than 20 lines are read from the standard output during the last second, they are ignored, and a warning is displayed :
```
Skipped 234 line(s)
```

## Options

### Rate
```bash
--rate n/unit or -r n/unit
```
Where n is the max number of lines per unit allowed to be printed. Valid units are ms, s, min, h, day.

### No-warning
```bash
--no-warning or -n
```
To avoid displaying the skipped lines count warning
