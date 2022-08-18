# ATS Monitoring
Application for monitoring and reporting of engineering systems, sending notifications via Telegram-bot, API SMS gateway

[Diagram](https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=diagram%20ats-monitoring.drawio.xml#Uhttps%3A%2F%2Fraw.githubusercontent.com%2Fstepanov-denis%2Fats-monitoring%2Fmaster%2Fdiagram%2520ats-monitoring.drawio.xml)
## Prerequisites outside docker
* Install Rust for Linux or macOS
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
For Windows, visit [this page](https://www.rust-lang.org/tools/install)
* Install dependencies
```
# Ubuntu
$ sudo apt-get install libssl-dev
# Fedora
$ sudo dnf openssl-devel
```
* Install [PostgreSQL](https://www.postgresql.org/download/)
## Initial setup
* Clone the repository
```
$ git clone https://github.com/stepanov-denis/ats-monitoring.git
```
* Edit your sms gateway settings in .cargo/config.toml
```
# For example:
GATEWAY_STR_CONNECTION= "URL with your token"
```
* Edit your Telegram-bot settings in .cargo/config.toml
```
# For example:
TG_BOT_TOKEN = "your token"
```
* Edit PostgreSQL settings in .cargo/config.toml
```
# For example:
POSTGRES_USERNAME = "postgres"
POSTGRES_PASSWORD = "mysecretpassword"
POSTGRES_DB = "postgres"
POSTGRES_HOSTNAME = "postgresql"
POSTGRES_PORT = "5432"
```
* Edit PLC settings in .cargo/config.toml
```
# For example:
# IP adress PLC TRIM5
IP_TRIM5 = "ip_adress:port"

# generator_work modbus adress
TRANSMITTED_WORK = "6"

# connection modbus adress
CONNECTION = "19"
```
## Setting up the environment
* Install [SMLogix](https://segnetics.com/ru/smlogix)
* Upload into PLC Pixel file "ats control.psl"
* Upload into PLC Trim5 file "winter garden.psl"
* Install [Orange Data Mining](https://orangedatamining.com/download/#linux)
* [Сonfigure](https://orangedatamining.com/widget-catalog/data/sqltable/) data reading from SQL database
* Create an account on [ClickSend](https://www.clicksend.com/) and top up your balance
* Create Telegram-bot
* Run PLC Pixel
* Run PLC Trim5
## Run app locally
* Run ATS Monitoring
```
$ cd ats-monitoring && cargo run --release
```
* For run ATS Monitoring with env_logger
```
# For example:
$ RUST_LOG=debug cargo run --release
```
* For write log's to file
```
$ RUST_LOG=debug cargo run --release > log.txt
```
## Run app in docker locally
* Run app, postgresql, in the background
```
$ docker compose up -d
```
* Tear it all down
```
$ docker compose down
```
* Tear it all down with removing volumes
```
$ docker compose down --volumes
```
## Use
#### Orange Data Mining
Сreate and save the report in the form you need in Orange Data Mining
#### SMS
Check your phone for SMS messages from ClickSend
#### Telegram-bot monitoring
To track the instantaneous values of the variables of the automatic reserve input control system, enter the command (or select an item in the menu).
```
/ats
```
To track the instantaneous values of the variables of the automatic winter garden management system, enter the command (or select an item in the menu)
```
/wintergarden
```
#### Telegram bot sends messages in the following cases:
* Errors when working with the database
* Modbus tcp operation errors
* Other critical errors of the application
#### Telegram bot and sms gateway sends messages in the following cases:
* Disconnecting the power supply from the mains
```
# For example:
disconnecting power from the mains,
successful start of the generator
```
or
```
# For example:
disconnecting power from the mains,
the generator startup failed
```
* Restoration of power supply from the power grid
```
# For example:
the power supply from the power grid has been restored,
the generator is working fine
```
or
```
# For example:
the power supply has not been restored,
the generator is faulty
```
* Failure of the generator in the transmission mode of power supply from the city power grid
```
# For example:
Alarm! The generator is faulty! Urgently perform service work!
```
* Restoring the operation of the generator in the mode of electricity transmission from the city power grid
```
the efficiency of the generator in the mode 
of transmission of electricity from the power grid has been restored
```