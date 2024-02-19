## Important!
This is a **learning** project. It's my first approach with rust. \
I know that many things are better implemented in existing crates but I want to learn the fundamentals


## Considered features
- Ability to create a config which stores the paths
- Map the directory with sub-folders (maybe create cache) for pictures
- Statistics and log file


## Config file format (maybe)
- The file uses keys to identify the values
- A value is read in one line because the file is parsed line per line
- Lines with a ``;`` are ignored
- Empty lines or lines not containg a equal symbol will be skipped
```
KEY_1=234
KEY_2=2345
; This is a cool comment
PATH_1=/home/me/Pictures
```
