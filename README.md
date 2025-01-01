# fc - File Cleaner

Deletes all individual files based on their file extension.

E.g. in a directory with many `*.orf` and `*.jpg` files all files that exist in either `*.orf` or `*.jpg` format are deleted.

This is useful when cleaning up photos by only going through all photos with one file extension, e.g. `*.jpg`, while deleting all unwanted ones and afterwards removing all corresponding RAW files as well.

```bash
fc 0.2.0
Benjamin Schilling <benjamin.schilling33@gmail.com>
Among all files with the given file extensions, remove individual files.

USAGE:
    fc.exe [FLAGS] --extension <extensions>... --path <path>

FLAGS:
    -c               Immediately confirm deletion.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --extension <extensions>...    File extension in scope for clean-up.
    -p, --path <path>                  Path to the directory for clean up.
```

Call example:
```
fc -e TXT MD -p ./dir_with_files
```

# License

SPDX-License-Identifier: MIT

# Copyright

Copyright (C) 2021 - 2025 Benjamin Schilling
