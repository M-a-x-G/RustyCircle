# RustyCircle

[![Build Status](https://travis-ci.org/M-a-x-G/RustyCircle.svg)](https://travis-ci.org/M-a-x-G/RustyCircle)

This is an implementation of the [circle version of Bresenham's line algorithm] (https://de.wikipedia.org/wiki/Bresenham-Algorithmus) written in Rust.

## Getting started

1. Donwload and install [Rust](https://www.rust-lang.org).
2. Clone this repository and navigate to the project root folder.
2. Compile and run the program:

```sh
    $ cargo run --release <center-x> <center-y> <radius>
```

## Example result

This is an example image with coordinates x: 100, y: 75, r: 50.

```sh
    $ cargo run --release 100 75 50
```

![Raster graphic] (https://github.com/M-a-x-G/RustyCircle/blob/master/circle.png)

## Licence

Copyright 2015 Max Gregor

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
