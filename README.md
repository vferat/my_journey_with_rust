# my_journey_with_rust

## Installation

```bash
sudo nano ~/.bashrc
```

And add the following line at the end of the file:
```bash
source "$HOME/.cargo/env"
```

## Install Cmake
```bash
wget https://github.com/Kitware/CMake/releases/download/v3.25.0-rc4/cmake-3.25.0-rc4-linux-x86_64.sh
chmod u+x cmake-3.25.0-rc4-linux-x86_64.sh
cmake-3.25.0-rc4-linux-x86_64.sh
```

edit ```~/.bashrc```

```txt
source "$HOME/.cargo/env"
PATH="/home/vferat/cmake-3.25.0-rc4-linux-x86_64/bin:$PATH"
```


 
## LSL web assembly
cargo build --target wasm32-unknown-emscripten RUST_BACKTRACE=1

/home/vferat/.cargo/registry/src/github.com-1ecc6299db9ec823/lsl-sys-0.1.1/liblsl/lslboost/boost/mpl/aux_/integral_wrapper.hpp:73:31: error: integer value -1 is outside the valid range of values [0, 3] for this enumeration type [-Wenum-constexpr-conversion]

https://github.com/sccn/liblsl/blob/79060185009438281b8bd364a1bd93e93eebe26d/lslboost/boost/mpl/aux_/integral_wrapper.hpp#L73

-Wno-enum-constexpr-conversion

https://clang.llvm.org/docs/DiagnosticsReference.html#wenum-constexpr-conversion

https://gregkatz.github.io/2017-05-20-rust-emscripten.html
