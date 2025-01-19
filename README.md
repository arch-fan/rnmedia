# rnmedia

A simple tool to rename media files, based on user input, to follow naming conventions.

## Installation

Download the binary from the [releases page](https://github.com/arch-fan/rnmedia/releases) and add it to your `PATH`.

Or use `cargo` to install it:

```sh
cargo install --git https://github.com/arch-fan/rnmedia
```

## Usage

You have to specify the media name, the season and the path were files are located.

```sh
rnmedia --media "The Office" --season 1 season_path/
```

The output will be something like:

```
[random_rip] The Office - Chapter 1.mkv -> The Office - S01E01.mkv
[random_rip] The Office - Chapter 2.mkv -> The Office - S01E02.mkv
[random_rip] The Office - Chapter 3.mkv -> The Office - S01E03.mkv
[random_rip] The Office - Chapter 4.mkv -> The Office - S01E04.mkv
[random_rip] The Office - Chapter 5.mkv -> The Office - S01E05.mkv
[random_rip] The Office - Chapter 6.mkv -> The Office - S01E06.mkv
```

If you want the files to be renamed, use the flag `--apply`:

```sh
rnmedia --apply --media "The Office" --season 1 season_path/
```

## Help command

```sh
$ rnmedia --help
Usage: rnmedia [OPTIONS] --season <SEASON> --media <MEDIA> <PATH>

Arguments:
  <PATH>  Path to the directory containing media files

Options:
  -a, --apply            Actually apply the new filenames (if not provided, just prints preview)
  -s, --season <SEASON>  Season number
  -m, --media <MEDIA>    Media title
  -o, --offset <OFFSET>  Episode offset [default: 0]
  -h, --help             Print help
  -V, --version          Print version
```


