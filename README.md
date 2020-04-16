# subsrus

CLI utility to download movie subtitles from the
[subsdb](http://thesubdb.com/api/) api.

## example

``` text
./subsrus -l es test_vids/justified.mp4

Search for subtitles:
        test_vids/justified.mp4

Writing srt file:
        test_vids/justified.es.srt
```

## usage

``` text
subsrus 0.1.0
Luke Arntz <luke@blue42.net>
Download subtitles from SubsDB (http://thesubdb.com/)

USAGE:
    subsrus [OPTIONS] <source_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --languages <lang>    Comma separated list of languages. Downloads all available. Example: es,en to download
                              Spanish and English. Defaults to English.

ARGS:
    <source_file>    Source video file.
```

