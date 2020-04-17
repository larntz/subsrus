# subsrus

CLI utility to upload and download movie subtitles from the
[subsdb](http://thesubdb.com/api/) api.

Could use some polish but everything is working. Let me know
if you experience bugs! Thanks.

## example

### download subtitles

``` text
./subsrus download -l es test_vids/justified.mp4

Search for subtitles:
        test_vids/justified.mp4

Writing srt file:
        test_vids/justified.es.srt
```

## usage

### subcommands

``` text
% target/debug/subsrus --help
subsrus 0.1.0
Luke Arntz <luke@blue42.net>
Download subtitles from SubDB (http://thesubdb.com/)

USAGE:
    subsrus [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    download    Download subtitles from SubDB (http://thesubdb.com)
    help        Prints this message or the help of the given subcommand(s)
    upload      Upload subtitles to SubDB (http://thesubdb.com)
```

### upload subtitle

``` text
% target/debug/subsrus upload --help
subsrus-upload
Upload subtitles to SubDB (http://thesubdb.com)

USAGE:
    subsrus upload <source_video> <source_subtitle> --language <lang>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --language <lang>    Specify the subtitle language.

ARGS:
    <source_video>       Source video file.
    <source_subtitle>    Source subtitle file.
```

### download subtitle

``` text
% target/debug/subsrus download --help
subsrus-download
Download subtitles from SubDB (http://thesubdb.com)

USAGE:
    subsrus download [OPTIONS] <source_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --languages <lang>    Comma separated list of languages. Downloads all available. Example: es,en to download
                              Spanish and English. Defaults to English.

ARGS:
    <source_file>    Source video file.
```
