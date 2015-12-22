# Changelog
## Version 0.3
### Breaking
- The ```parse``` function now takes a reference to the arguments instead of consuming them (&Vec<String> vs. Vec<String>)

### Additions
- Added a ```set_title``` option to the argument parser
- The parser now uses only uses the first argument as the program title when parsing the program's args (env::args) eg. with ```parse_args``` and ```parse_args_or_exit```
