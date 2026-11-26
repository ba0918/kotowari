# Base directory

English | [日本語](base-directory.ja.md)

Covers how the directory that kotowari bases relative paths on is decided, and how the paths that are read and written relative to that base are handled. How the configuration file itself is read is the [Configuration](config.md) topic, and the target operating systems are decided in [The target operating systems are Linux and macOS, and on Windows only the normalization of path separators is promised](cli-environment.md#the-target-operating-systems-are-linux-and-macos-and-on-windows-only-the-normalization-of-path-separators-is-promised).

## The base of relative paths is the first directory with .kotowari/ found by searching upward

Status: implemented

### Problem

The values in the configuration file and the paths in the output are written as relative paths, so what they are relative to has to be decided. The reason for choosing this way of searching and the reason for not letting "--config" move the base are not recorded.

### Decision

There is one base for relative paths. kotowari searches upward from the current directory for a ".kotowari/" directory, and makes the first directory found the base directory.

| Order | Condition | Base directory |
|---|---|---|
| 1 | Going up from the current directory, a directory containing a ".kotowari/" directory is found. A file named ".kotowari" is ignored and the search continues upward | The first directory found |
| 2 | Nothing is found in 1 | The current directory |

"--config" only changes which configuration file is read; it does not change the base directory.

### Confirmations

- C-core-base-1 When "kotowari check" is run in a lower directory, the ".kotowari/" directory above becomes the base, and the "path" of a finding is relative to it
- C-core-base-2 If there is no ".kotowari/" anywhere above the current directory, the current directory becomes the base
- C-core-base-3 Even if a directory on the way has a file named ".kotowari", it is ignored, and the ".kotowari/" directory further up becomes the base
- C-core-base-4 Wherever the configuration file that "--config" points at is, the base directory does not change

## Paths in configuration values and the "path" of the output are relative to the base directory

Status: implemented

### Problem

What the paths of locations written in the configuration file and the paths reported in findings are relative to has to be decided. The reason for making them relative to the base directory rather than the current directory is not recorded.

### Decision

kotowari reads the paths in the configuration file's values as paths relative to the base directory. The "path" of the output is also a path relative to the base directory. The "path" of the output is built by joining the [normalized](#paths-in-configuration-values-are-normalized-before-comparing-and-before-output) location and the document name with "/".

### Confirmations

- C-core-base-5 Even when run in a subdirectory, a location written in the configuration file is read as relative to the base directory
- C-core-base-6 The "path" of the output is the normalized location and the document name joined with "/"

## Paths in configuration values are normalized before comparing and before output

Status: implemented

### Problem

Even paths that point at the same place can be written in more than one way (a trailing "/", a leading "./", "a/..", and so on). When the characters were compared as written, writing a location as "a/../a/b" made the files inside that location be judged as outside it. The paths compared against had already collapsed "..", and only the location side was not collapsed, so they disagreed.

### Decision

kotowari normalizes the paths in the configuration file's values, before comparing them and before output, as follows.

- Remove a trailing "/" and a leading "./"
- Collapse an inner "/./" and consecutive "/" into a single "/"
- Turn "\\" into "/"
- Collapse "a/..". A ".." with nothing to collapse against is kept, and the path stays one that points outside the base directory

### Confirmations

- C-core-base-7 A trailing "/", a leading "./", an inner "/./" and consecutive "/" are removed or collapsed, and "\\" becomes "/"
- C-core-base-8 If a location value in the configuration file has "a/..", it is collapsed before comparing, and a ".." with nothing to collapse against is kept
