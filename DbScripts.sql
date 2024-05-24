 DROP TABLE IF EXISTS Items;
 DROP TABLE IF EXISTS Places;

CREATE TABLE Places (
    placeId SERIAL PRIMARY KEY,
    placeName VARCHAR(30) NOT NULL,
    placeType VARCHAR(3) NOT NULL
);

CREATE TABLE Items (
    itemId SERIAL PRIMARY KEY,
    placeId INT REFERENCES Places(placeId) NOT NULL,
    nbOfItems INT not NULL, 
    itemName VARCHAR(30) NOT NULL
);

INSERT INTO Places (placeName, placeType) 
VALUES ('Inventaire', 'IN');

INSERT INTO Items (placeId, nbOfItems, itemName) 
VALUES ((SELECT placeId FROM Places WHERE placeName = 'Inventaire'), 3, 'chaise');

INSERT INTO Items (placeId, nbOfItems, itemName) 
VALUES ((SELECT placeId FROM Places WHERE placeName = 'Inventaire'), 2, 'hamac');

INSERT INTO Items (placeId, nbOfItems, itemName) 
VALUES ((SELECT placeId FROM Places WHERE placeName = 'Inventaire'), 1, 'ballon');

SELECT * FROM Places;
SELECT * FROM Items;



