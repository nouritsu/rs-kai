# About
A (less than) bare minimum shell written in pure rust.  
The name comes from the Japanese word 貝 (kai) which means shell or shellfish (hehe).

# Installation
Clone repository and run the following command  
Make sure you have cargo and rustc installed.
```
cargo build --release
```

# Features
## Basic commands
Any executables present in $PATH can be used in the shell.  
Examples -
```
ls
```
Arguments are suppported
```
ls -ltra
```
## Inbuilt (cd)
Change directory is an inbuilt that will switch to directory if it exists, print out an error otherwise.
Examples -
```
cd ./src
```
```
cd ..
```
```
cd
```
Note that `cd -` and such variants will not work.
## Inbuilt (exit)
Exit will (gracefully) exit from the shell and return you to your previous shell.
Example -
```
exit
```
## Piping
Basic pipe operations will work.  
Example -
```
ls | grep "Cargo"
```
