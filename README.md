# Bluetooth connectivity - Rust

This repository is a sample project (cli interface) that consumes BlueZ. BlueZ is a linux kenal interface that allows bluetooth connectivity.

This repository is developed as a experiment to learn about bluetooth programming in Rust.

It uses the (bluez-aync)[https://crates.io/crates/bluez-async] crate for this purpose.

This crate is an async wrapper around the D-Bus interface of BlueZ (the Linux Bluetooth daemon), supporting GATT client (central) functionality.