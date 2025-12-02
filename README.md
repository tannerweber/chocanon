# ChocAn

Chocaholics Anonymous simulator for CS 314, Fall 2025, Portland
State University.

## Authors
* Tanner Weber - tannerw@pdx.edu
* Miles Turoczy - turoczy@pdx.edu
* Torin Costales - tcostal2@pdx.edu
* Cristian Hernandez - cristhe@pdx.edu
* Jethro Fernandez - jethrof@pdx.edu

# Prerequsites

* rustc
* cargo
* make

# Building

The binary can be built with ```make release```.
We've mainly tested on GNU/Linux x86_64.

# Running

The emails are outputted in the emails directory.
```make clean``` can be run to remove the directory.

# Testing

The default make target runs testing, linting, and formatting.
