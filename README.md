# Ats-Monitoring
An application for sending notifications via the SMS gateway API and recording events in the PostgreSQL DBMS about the status of the ATS (city power grid, generator, UPS)
## Diagram Ats-Monitoring
[Diagram Ats-Monitoring](https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=diagram%20ats-monitoring.drawio.html#R7VrbcuI4EP0aHpnyBTvwGCDJzhbZYZbszLOwhdFGlryyDCRfvy1bxhjbhBASyAxVFKAjWWq1uk%2BfKLTsQbi6Eyia33Mf05Zl%2BKuWPWxZ1lXXgncFPGnA6mRAIIifQWYBTMgz1qCh0YT4OC4NlJxTSaIy6HHGsCdLGBKCL8vDZpyWV41QgCvAxEO0iv4kvpxr1DSMouMPTIK5Xrrr6I4p8h4DwROm12Oc4awnRPk0emg8Rz5fbkD2TcseCM5l9i1cDTBVXs09lj1329C7NllgJvd5ICZJEDwPb5%2FnV9N%2B2BfB4Puf7V42ywLRRLtCGyufct9gH1ylm1zIOQ84Q%2FSmQPvp%2FrFaxoBWMWbEeQSgCeC%2FWMonfe4okRyguQyp7p1xJnWn6UK7ujW925gnwtN22T3TMXC3052ZTrdjeO08cJAIsNyxZx2aal8bk2un3WEeYimeYIDAFEmyKIcI0pEWrMcVLocv2uv1J9Bkccn7LgXb%2Bz5ZlE7B%2FS9RcZJ6qh2nrrqGAeCkVdGpzKMkYKUnPPAhFpuD3EB%2F0nzGQ5bKZvnKokQ97pNYCjJNJOFMNfGCwDnpJcAv2SrllQFOt5mjW3G3wEISSNFrvSWpgqmfb1Bvy%2B7HEfIICx7SUOsWwAjPVBBYBfK3zl8zxeYoUgt5yVSFcKyjz0h9L4Bj1EbsIQScSuMsQrVtnXLwlqN%2FneWq4fGQePo7RVNM%2B2u6GHDKwfxhShjwmBT8cc09eU7copBQRak%2FsPARQ1upYjXmifIdXu2M7rw3p6ecuXVzWdCgbWtsvsGAa1p7S0LUpqfVmBRRbZyGkO%2BEZUHqqhg1jOL9TakxbcqVvH7lD5rNc0DssX3Sy3Tr0uv6YbKRLdlclSRqyK2K9YBF29hc1HqGquRp3FNGTq99KqHbCCWHklG9t8Z8qY7TiJMogrR50UPV9Y9t0R1mWCDJxTkY8894cg5mjEfAC8aYrJSEfL05AFYj6YVSkodqGqCgUIAeZzRl6RlR5LRJ577g0UMuIYxdYuQVJNtzSiRr2lWWNbs1LLseeHSWNe3Pq%2Fx2qboX1Z9pnkr%2B7TK7UQJ%2BtKD7SbLZjQAJH7OLhPt8Es52T67hnDdouEy9ZZotw9tp3BT6LusD78i2jiXVd7CaO6yMDWCQ4MoTHpoShuUBimwq9tVon0SApaX9QZDQOQel8eMvZU6MxQKLnS7%2FrcRHxzq9%2BDAq%2FBByf5rEKnoGY5jz67jiR5XvddVjk9eVy7aLTkh8PxUr5apiV6uKvWbxLWoXXCKJpuslIg5VMnWK04cX7GZgfHFaDpg5gLZZtOGlhgsJdAFTI5IeI0axXOI4D%2BS9BND%2BB271tuqBWXPgtfXAfa%2FzvvrFxKa5712j6ZyV2jSbbxzfJhsnmmQNwXnYyKwXeXgG8tB1Ti4P8319Dn34Ybd9Iw7JsJ9ieaerwKpgPHQv76XpRpC4qU74NlaxNxzeHFniFciYxzIQePJ9tHPmXVNcy7h9zxmRXAA9vXKaLbF5brLSvfpSFpZXNTrDsp0PFZbV65SLsHw3YekaJxeWzRcNb7wG4%2BIxhrNJ%2F515UTPnq2Z6ndOrGfeiZuouyCjBTO6seb%2BtisluphYEL48vW74JxOCPTcsYQmmBj3vCfn3tYRo1teijxUe3wgMXzXFszdHtVc%2FZPY7kgGbxa7y0b%2BPHjvbN%2Fw%3D%3D)
## Setup for Windows
* Install VS code
* Install Rust
* Clone the repository, edit the http get requests, edit network addresses and
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
* Open configuration file "modbus map for ats-monitoring" in Lectus Modbus OPC/DDE server and edit network addresses
* Install [SMLogix](https://segnetics.com/ru/smlogix)
* Upload into PLC Pixel file "pixel.psl" and edit network addresses
* Upload into PLC Trim5 file "trim5.psl" and edit network addresses
* Run PLC Pixel
* Run PLC Trim5
* Run Lectus Modbus OPC/DDE server
* Run PostgreSQL
* Run ats-monitoring.exe
