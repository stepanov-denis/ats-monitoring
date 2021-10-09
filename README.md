# Ats-Monitoring
An application for sending notifications via the SMS gateway API and recording events in the PostgreSQL DBMS about the status of the ATS (city power grid, generator, UPS)

[Diagram Ats-Monitoring](https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=diagram%20ats-monitoring.drawio.xml#R7Vpdd9o4EP01PKbHH9iBxwBJtj10S5ds%2ByxsYdTIkleWgeTX78iWMcY2IUACaTmHQ9CVLI1GM3cuCi27Hy7vBYpmX7mPacsy%2FGXLHrQs67pjwbsCnjRgtTMgEMTPILMAxuQZa9DQaEJ8HJcGSs6pJFEZ9Dhj2JMlDAnBF%2BVhU07Lq0YowBVg7CFaRX8SX840ahpG0fEXJsFML91xdMcEeY%2BB4AnT6zHOcNYTonwaPTSeIZ8v1iD7tmX3Becy%2BxQu%2B5gqr%2BYey567a%2BhdmSwwk7s8EJMkCJ4Hd8%2Bz60kv7Img%2F%2F3LVTebZY5ool2hjZVPuW%2BwD67STS7kjAecIXpboL10%2F1gtY0CrGDPkPALQBPAXlvJJnztKJAdoJkOqe6ecSd1putCubk3vNuaJ8LRddtd0DNxpd6am02kb3lUeOEgEWG7Zsw5Nta%2B1ybXT7jEPsRRPMEBgiiSZl0ME6UgLVuMKl8MH7fX6E2iyuOR9l4LtPZ%2FMS6fg%2FpeoOEk9dRWnrrqBAeCkZdGpzKMkYKUnPPAhFuuD3ED%2FpfmM%2ByyVzfKZRYl63CexFGSSSMKZauI5gXPSS4BfslXKKwOcbjNHN%2BJujoUkkKI3ektSBVMv36Delt2LI%2BQRFjykodYpgCGeqiCwCuQfnb9mis1QpBbykokK4VhHn5H6XgDHqI3YAwg4lcZZhGrb2uXgLUf%2FKstVw%2BMh8fRniiaY9lZ00eeUg%2FmDlDDgMSn444p78py4QyGhilJ%2FYOEjhjZSxWrME%2BU7vNwa3XlvTk85c%2BvmoqBB29bYbI0BV7R2SELUpqfVmBRRbZyGkO%2BEZUHqqhg1jOL9oNSYNOVKXr%2FyB83mOSD22C7pZbp16XXzMF7LlmyuShI15FbFesCiTWwmaj1DVfI07ikjp9c%2BldBNhJJ9yajeWyO%2BUMdpxEkUQdq86KHq%2Bse26B4zLJDk4hyM%2BXc0PgczRkPgBWNElkpCvt4cAKuR9EIpyUM1DVBQKECPU5qy9JQoclqnc1%2Fw6CGXEMY2MfIKku06JZI17SrLmp0all0NPDrLmvbHVX7bVN2L6s80TyX%2FtpndKAHfW9D9JNnsRoCEj9lFwn08CWe7J9dwzgEaLlNvmWbL8Ks0bgp9l%2FWBd%2BSVjiXVt7ea26%2BM9WGQ4MoTHpoQhuUeimwidtVoH0SApaX9QZDQOQel8eNvZU6MxRyLrS7%2Fo8RH2zq9%2BDAq%2FBByf5LEKnr6I5jz86jiR5XvddVjndeVyzaLTkh8PxUr5apiV6uKvWLxDWoXXCKJJqslIg5VMnWK04MX7KZvfHJaDpjZh7ZZtOGlhgsJdAFTI5IeI0axXOA4D%2BSdBNDuB251N%2BqBWXPgtfXAfavzvv7NxKa5612j6ZyV2jSbbxwPk41jTbKG4DxsZNaLPDwDeeg6J5eH%2Bb4%2Bhj58t9u%2BIYdk2E2xvNFVYFUw7ruXt9J0Q0jcVCd8G6nYGwxujyzxCmTEYxkIPP4%2B3DrztilupDL1K2dEcgEE9cqJNuTmuQlL9%2FpTWVpe1ygNy3beVVpWL1Qu0vLNpKVrnFxaNl81HHgRxsVjDGeT%2FkPzomfOV89026fXM%2B5Fz9RdkVGCmdxa8%2F5YHZPdTc0JXhxfuHwTiMHXTcsYQGlR%2BoOw3197mEZNLXpv8dGp8MBFcxxbc3S61XN2jyM5oFn8Hi%2FtW%2Fu5o337Pw%3D%3D)
## Setup
* Install Rust for Linux or macOS
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
For Windows, visit [this page](https://www.rust-lang.org/tools/install)
* Clone the repository
* Edit the http get requests
```
let resp = reqwest::blocking::get("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=Сбой+питания+от+электросети.+Успешный+старт+генератора.").unwrap();
```
* Edit the connection configuration strings to PostgreSQL
```
let mut client =
    Client::connect("postgresql://postgres:postgres@localhost/postgres", NoTls)?;
```
* Compile local packages and all their dependencies
```
$ cargo build --release
```
## Setting up the environment
* Install PostgreSQL
* Install ODBC driver for PostgreSQL
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
* Install [Lectus Modbus OPC/DDE server](http://www.lectussoft.com/) or another OPC server
* In OPC server settings connect an external PostgreSQL DBMS
* Open configuration file "modbus map for ats-monitoring" in Lectus Modbus OPC/DDE server and edit network addresses. When using another OPC server, configure the modbus variables yourself
* Install [SMLogix](https://segnetics.com/ru/smlogix)
* Upload into PLC Pixel file "pixel.psl" and edit network addresses
* Upload into PLC Trim5 file "trim5.psl" and edit network addresses
* Install [Orange Data Mining](https://orangedatamining.com/download/#linux)
* [Сonfigure](https://orangedatamining.com/widget-catalog/data/sqltable/) data reading from SQL database
* Create an account on [ClickSend](https://www.clicksend.com/) and top up your balance
* Run PLC Pixel
* Run PLC Trim5
* Run OPC server
* Run PostgreSQL
* Run Ats Monitoring
## Use
* Сreate and save the report in the form you need in Orange Data Minig
* Check your phone for SMS messages from ClickSend
