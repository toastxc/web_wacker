## Usage
**concurrent**: number of threads running a request
**recursion**: how many times requests are made (R*C) 
**delay**: time between requests

The results are saved (in milliseconds) in `times.csv`.
```text
Usage: web_wacker [OPTIONS] --url <URL>

Options:
  -c, --concurrent <CONCURRENT>  [default: 10]
  -r, --recursion <RECURSION>    [default: 1]
  -d, --delay <DELAY>            [default: 1]
  -u, --url <URL>                
  -h, --help                     Print help
  -V, --version                  Print version
```
