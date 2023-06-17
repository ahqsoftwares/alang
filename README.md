[![ALang Builder Action](https://github.com/ahqsoftwares/alang/actions/workflows/buildmake.yaml/badge.svg?branch=master)](https://github.com/ahqsoftwares/alang/actions/workflows/buildmake.yaml)
# Alang - A new experimental language
> Note ⚠️
>
> This is completely experimental language which is written in rust. Not intended for general use currently

# Development
- Core
    - cli: The CLi which the user 
    - compiler: Compiles from acss to aasm (aasm is currently a stripped off version of acss)
    - interpreter: Reads aasm & executes it
    - packager: Package your app into a native app for windows, mac, linux
    - packloader: A Package Manager for ALang

- JS Bundler (Only executed during github release)
    - js 

- Code Templates (Only installed with installer once every release)
    - templates

- Updater
    - updater: A clone of the installer with slight modifications

# Helpful Executables for Development
- watcher/watchexec: A watcher executable for windows
    - Run ./watcher/watchexec --watch ./{module}/src -r cargo build