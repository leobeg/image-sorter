## Important!
This is a **learning** project. It's my first approach with rust. \
I know that many things are better implemented in existing crates but I want to learn the fundamentals

## How it works
- When you run the image sorter you enter both input and output path
- The dates from the images are acquired trough the exif header or if the image contain WA trough the filename
- The images are moved to a sort folder
- You enter the indexes and then the folder or you use the default "Sonstiges" after that you can choose an optional filename
- The images then will be moved to the output folder with the following scheme: "2024/MTB/001_2024-10-24-Name.png"
- The image-sorter detects the index in a folder and automatically resorts the images if the indexes don*t match after the initial sorting process

## Considered features
- Ability to create a config which stores the paths
- Map the directory with sub-folders (maybe create cache) for pictures
- Statistics and log file
- Whatsapp File format Date detection#
- Resort files in folder so index matches date order
- Write non-exif detected image date to the exif of the image



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
