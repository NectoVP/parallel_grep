# Parallel grep

### Faster then default windows search BUT uses 100% of your CPU :P

Returns line with needed pattern, line number and file path.
File names to be considered are specified using regex

Data races and race conditions are handled with the help of library Rayon

## Usage

To run simply type something like this
```
cargo run -- -p "123" -d "/root/etc" -f "\.txt"
```

Or using the built file:
```
./a.exe -p "123" -d "/root/etc" -f "\.txt"
```

Possible commands:
```
Usage: pargrep.exe [OPTIONS] --pattern <PATTERN> --directory <DIRECTORY>

Options:
  -p, --pattern <PATTERN>
  -m, --max-depth <MAX_DEPTH>  [default: -1]
  -d, --directory <DIRECTORY>
  -f, --file_name <FILE_NAME>  [default: \.]
  -h, --help                   Print help
  -V, --version                Print version
```