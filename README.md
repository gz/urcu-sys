# liburcu-sys

Rust bindings for [liburcu](http://liburcu.org/).

# Install liburcu

On Ubuntu do:
```
apt install liburcu-dev liburcu6
```

On MacOS do:
```
git clone git://git.liburcu.org/userspace-rcu.git
./bootstrap
./configure --build=x86_64-apple-darwin11
make install
```