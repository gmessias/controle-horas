# Rust CLI - Controle de Horas

## Summary

It was developed on Windows. </br>
**Objective**: A user records their hours worked on a particular subject. </br>
**Future Objective / Goal**: this project is in the users, and with a command sends their data to a "central" where some user can check the data of everyone who sends it. </br>

The idea is: you have the project on the machine, open a terminal from any directory and start recording the hours worked. </br>
In this first version, the record would be a "draft", and then consulted and recorded in third-party software.
Example: Trello, Azure, Redmine, etc.

## How to use 

With the project executable: </br>
* Copy the PATH where the executable is located.
* In Windows, go to _**environment variables**_.
* In the _**system variables**_ section, go to PATH, Edit, New and paste the PATH of the executable file.
* You will most likely need to restart the machine.

With the source code: </br>
* [Install Rust](https://www.rust-lang.org/tools/install)(rustup)
* In the code directory use the command:
```
cargo build --release
```
* The executable will be in the target/release folder, copy the absolute PATH of the executable location.
* In Windows, go to _**environment variables**_.
* In the _**system variables**_ section, go to PATH, Edit, New and paste the PATH of the executable file.
* You will most likely need to restart the machine.

---

## Commands

### Informations

```
controle-horas --help
```
It will list the available commands and other project information.

### Initializing a user

```
controle-horas create --user username
```
Initializes a user, resulting in two files:
* **config.txt** which has the user's name so that you don't have to enter it when you register every command.
* **usuario.json** which has the registry information but when it is initialized it will only have the value of the user name itself.

### Insert a record

```
controle-horas insert --id ID --data DD/MM/AAAA --horas HORAS --minutos MINUTOS
```

### Read a record

```
controle-horas read --id ID </br>
controle-horas read --dia DIA </br>
controle-horas read --mes MES </br>
```

Choosing to read by ID, DIA or MES will result in the data being filtered according to your argument.