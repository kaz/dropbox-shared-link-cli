# dropbox-shared-link-cli

[![](https://github.com/kaz/dropbox-shared-link-cli/workflows/release/badge.svg)](https://github.com/kaz/dropbox-shared-link-cli/actions?query=workflow%3Arelease)
[![](https://img.shields.io/github/v/release/kaz/dropbox-shared-link-cli)](https://github.com/kaz/dropbox-shared-link-cli/releases)

A command line tool that downloads files from Dropbox's shared link

## Download

Available on [releases](https://github.com/kaz/dropbox-shared-link-cli/releases) page

## Usage

```
$ ./dropbox-shared-link-cli --help
$ ./dropbox-shared-link-cli --root https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa/ ls /ABC123/D
$ ./dropbox-shared-link-cli --root https://www.dropbox.com/sh/arnpe0ef5wds8cv/AAAk_SECQ2Nc6SVGii3rHX6Fa/ cp /ABC123/D/in/in01.txt $HOME/Downloads/in.txt
```
