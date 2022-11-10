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