-- DROP TABLE IF EXISTS Items;
-- DROP TABLE IF EXISTS Places;

CREATE TYPE placeTypeSelector AS ENUM ('OUT', 'IN');

CREATE TABLE Places (
    placeId UUID PRIMARY KEY NOT NULL,
    placeName VARCHAR(30) NOT NULL,
    placeType placeTypeSelector NOT NULL
);

CREATE TABLE Items (
    itemId UUID PRIMARY KEY NOT NULL,
    placeId UUID REFERENCES Places(placeId) NOT NULL,
    nbOfItems VARCHAR(30) NOT NULL, 
    itemName VARCHAR(30) NOT NULL
);

INSERT INTO Places (placeId, placeName, placeType) 
VALUES (gen_random_uuid(), 'Inventaire', 'IN');

INSERT INTO Items (itemId, placeId, nbOfItems, itemName) 
VALUES (gen_random_uuid(), (SELECT placeId FROM Places WHERE placeName = 'Inventaire'), '3', 'chaise');

INSERT INTO Items (itemId, placeId, nbOfItems, itemName) 
VALUES (gen_random_uuid(), (SELECT placeId FROM Places WHERE placeName = 'Inventaire'), '2', 'hamac');

INSERT INTO Items (itemId, placeId, nbOfItems, itemName) 
VALUES (gen_random_uuid(), (SELECT placeId FROM Places WHERE placeName = 'Inventaire'), '1', 'ballon');

SELECT * FROM Places;
SELECT * FROM Items;