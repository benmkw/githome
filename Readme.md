# githome

This is a simple tool which opens the (github) page of a cloned git repository by searching the file hierarchy upwards for a git repository.
It's only tested on macOS and occasionally on linux ;)

It's basically the same as `open (git config --get remote.origin.url)` but it tries to handle ssh remotes as well as https ones.

Git urls do not conform to the URL Standard. (see [rust-url](https://github.com/servo/rust-url/issues?q=is%3Aissue+ssh+))

I use it all them time and think you might like it or create something similar adapted to your needs.
