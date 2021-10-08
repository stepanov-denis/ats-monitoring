## Ats-Monitoring
An application for sending notifications via the SMS gateway API and recording events in the PostgreSQL DBMS about the status of the ATS (city power grid, generator, UPS).
## Setup for Windows
* Install Rust
* Install Ats-Monitoring
```
$ cargo install ats-monitoring
```
If you want to use the SMS notification function, clone the repository, edit the http get requests and
compile local packages and all their dependencies
```
$ cargo build --release
```
## Setting up the environment
* Install PostgreSQL
* Install OLEDB for ODBC driver for PostgreSQL
* Create the following tables in PostgreSQL:
```
CREATE TABLE avr_control_insert (
    mains_power_supply int NOT NULL,
    start_generator int NOT NULL,
    generator_faulty int NOT NULL,
    generator_work int NOT NULL,
    connection int NOT NULL,
    mark timestamptz default current_timestamp
);
```
```
CREATE TABLE журнал_работы_приложения (
    событие text NOT NULL,
    время_и_дата timestamp default current_timestamp
);
```
```
CREATE TABLE события_авр (
    событие text NOT NULL,
    время_и_дата timestamp default current_timestamp
);
```
```
CREATE TABLE зимний_сад (
    фитоосвещение_1 int NOT NULL,
    фитоосвещение_2 int NOT NULL,
    фитоосвещение_3 int NOT NULL,
    фитоосвещение_4 int NOT NULL,
    вентилятор int NOT NULL,
    автополив_1 int NOT NULL,
    автополив_2 int NOT NULL,
    автополив_3 int NOT NULL,
    температура int NOT NULL,
    влажность int NOT NULL,
    освещенность_в_помещении int NOT NULL, 
    освещенность_на_улице int NOT NULL,
    дата_и_время timestamp default current_timestamp
);
```
* Install [Lectus Modbus OPC/DDE server](http://www.lectussoft.com/)
* In Lectus Modbus OPC/DDE server settings connect an external PostgreSQL DBMS
* Open configuration file "modbus map for ats-monitoring" in Lectus Modbus OPC/DDE server
* Install [SMLogix](https://segnetics.com/ru/smlogix)
* Upload into plc Segnetics Trim5 file "trim5.psl"
* Run plc Trim5
* Run Lectus Modbus OPC/DDE server
* Run ats-monitoring.exe
