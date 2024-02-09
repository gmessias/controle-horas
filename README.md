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
* **usuario.json** which has the registry information but when it is initialized it will only have the value of the username itself.

### Insert a record

```
controle-horas insert --id ID --data DD/MM/AAAA --horas HORAS --minutos MINUTOS
```
or
```
controle-horas insert --id ID --data DD/MM/AAAA --tempo HH:MM
```
Example: --tempo 1:20
Hour: 1
Minutes: 20


### Read a record

```
controle-horas read --id ID </br>
controle-horas read --dia DIA </br>
controle-horas read --mes MES </br>
```

Choosing to read by ID, DIA or MES will result in the data being filtered according to your argument.

### Observation

The file in the root of the source code project named template-username.json was created from these commands:

```
controle-horas create --user template-username
controle-horas insert --id 1 --data 01/01/2024 --horas 3
controle-horas insert --id 1 --data 01/01/2024 --horas 2 --minutos 30
controle-horas insert --id 2 --data 01/01/2024 --horas 2 --minutos 30
controle-horas insert --id 3 --data 02/01/2024 --horas 8 --minutos 00
controle-horas insert --id 4 --data 03/01/2024 --horas 4 --minutos 45
controle-horas insert --id 5 --data 12/02/2024 --horas 6 --minutos 15
controle-horas insert --id 5 --data 12/02/2024 --horas 1 --minutos 45
controle-horas insert --id 5 --data 13/02/2024 --horas 0 --minutos 50
controle-horas insert --id 5 --data 13/02/2024 --horas 7 --minutos 10
controle-horas insert --id 5 --data 14/02/2024 --horas 2
controle-horas insert --id 6 --data 15/02/2024 --minutos 59
controle-horas insert --id 7 --data 16/02/2024 --minutos 60
controle-horas insert --id 8 --data 17/01/2024 --tempo 6:33
```

### Tests

```
cargo test
```