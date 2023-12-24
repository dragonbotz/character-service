-- This script contains all the initialization procedures for the Character 
-- Service's database.
--
-- To appy changes to an existing database, just run this script once again, it 
-- will execute all the ALTER procedures it contains without whipping off the 
-- existing data.
--
-- Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
 
 -- Creates the Character Service's database and its tables:
 -- # Tables:
 -- * character
CREATE DATABASE characterdb;

-- Go to characterdb
\c characterdb

CREATE TABLE IF NOT EXISTS character (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(35),
    image_url TEXT,
	rarity SMALLINT
);
