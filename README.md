# filewatcher

Run a command when file changes.

## Usage

```
Required option 'pattern' missing.

Usage: filewatcher [options]

Options:
    -d, --directory DIRECTORY
                        set directory
    -p, --pattern PATTERN
                        set pattern
    -c, --command COMMAND
                        set command
    -h, --help          print this help menu
```

## Examples

`./filewatcher -p "*.rs" -c "wc -l" -d src/`
