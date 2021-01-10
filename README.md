# you-dl

A tiny and easy to use YouTube downloader.
[todo giphy]

## Installation

Available for macOS:

```sh
brew tap l-oris/you-dl
brew install you-dl
```

## Usage

```sh
you-dl <url> <url> <url>

# read the URLs from a text file (lines starting with `#` and `//` are ignored)
you-dl -f <path_to_file>
```

## Limitations

Most videos uploaded by verified channels are protected: their media streams cannot be directly accessed by URL. To download them, their signatures need to be deciphered and their URLs modified appropriately.

It wasn’t my goal to provide a full-fledged replacement for [youtube-dl](https://github.com/ytdl-org/youtube-dl) and I didn’t pursue this feature altogether.
If you're interested, you can find out more here: https://tyrrrz.me/blog/reverse-engineering-youtube

That said, it would be annoying to keep using two separate tools for downloading YouTube videos.
For this reason, `you-dl` includes a wrapper around [youtube-dl](https://github.com/ytdl-org/youtube-dl), which gives access to a larger number of resources without sacrificing the easy-to-use aspect of this tool.
To trigger this functionality, simply pass the `-w` flag:

```sh
you-dl -w <url> <url> <url>
```
