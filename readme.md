vgmdb-rust
==========

[![Build Status](https://travis-ci.org/Bilalh/vgmdb-rust.svg?branch=master)](https://travis-ci.org/Bilalh/vgmdb-rust)
[![](http://meritbadge.herokuapp.com/vgmdb)](https://crates.io/crates/vgmdb)

Retags mp3 using data from vgmdb. Only uses the English names at the moment.

Usage
-----

```
Usage:
    ./vgmdb [OPTIONS] DIR ALBUM_ID

Tags mp3s using vgmdb

positional arguments:
  dir                   Directory of mp3s
  album_id              Id from vgmdb

optional arguments:
  -h,--help             Show this help message and exit
  -l,--no-length-check  Continue even if there is mismatch in the number of tracks
                        of the dir and the db
```

Licence
-------

Copyright [2015] Bilal Syed Hussain

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.


Authors
-------
* Bilal Syed Hussain
