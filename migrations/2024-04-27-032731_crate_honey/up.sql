-- Your SQL goes here
--  before executing migration on 1st time, you need execute following command on sqlite server for activating foreign key
--  sqlite>   PRAGMA foreign_keys=true;
--  sqlite3 syntax
--  beekeepers main
CREATE TABLE beekeepers (
    id integer PRIMARY KEY,
    beekeeper_name VARCHAR NOT NULL
);
-- honey main
CREATE TABLE honeies (
    id SERIAL PRIMARY KEY,
    honey_name VARCHAR NOT NULL,
    beekeeper_id integer,
    FOREIGN KEY (beekeeper_id) REFERENCES beekeepers(id)
);