DROP TABLE IF EXISTS Inventory;
DROP TABLE IF EXISTS Items;
DROP TABLE IF EXISTS Places;

CREATE TABLE IF NOT EXISTS Places (
    place_id SERIAL PRIMARY KEY,
    place_name VARCHAR(30) UNIQUE NOT NULL CHECK (TRIM(place_name) <> ''),
    place_type VARCHAR(3) NOT NULL CHECK (TRIM(place_type) <> '')
);

CREATE TABLE IF NOT EXISTS Items (
    item_id SERIAL PRIMARY KEY,
    item_name VARCHAR(30) UNIQUE NOT NULL CHECK (TRIM(item_name) <> '')
);

CREATE TABLE IF NOT EXISTS Inventory (
    place_id INT NOT NULL,
    item_id INT NOT NULL,
    nb_of_items INT NOT NULL CHECK (nb_of_items > 0),
    PRIMARY KEY (place_id, item_id),
    FOREIGN KEY (place_id) REFERENCES Places(place_id),
    FOREIGN KEY (item_id) REFERENCES Items(item_id)
);

-- INSERT INTO Places (place_name, place_type) VALUES ('Dépôt', 'INV');
-- INSERT INTO Places (place_name, place_type) VALUES ('Vachon', 'IN');
-- INSERT INTO Places (place_name, place_type) VALUES ('Pouliot', 'OUT');

-- INSERT INTO Items (item_name) VALUES ('tapis');
-- INSERT INTO Items (item_name) VALUES ('chaise');
-- INSERT INTO Items (item_name) VALUES ('ballon');

-- INSERT INTO Inventory (place_id, item_id, nb_of_items)
-- SELECT (SELECT place_id FROM Places LIMIT 1), --dépot
--        (SELECT item_id FROM Items LIMIT 1), --tapis
--        2;
-- INSERT INTO Inventory (place_id, item_id, nb_of_items)
-- SELECT (SELECT place_id FROM Places LIMIT 1 OFFSET 2), --Pouliot 
--        (SELECT item_id FROM Items LIMIT 1 OFFSET 1), --chaise
--        2;
-- INSERT INTO Inventory (place_id, item_id, nb_of_items)
-- SELECT (SELECT place_id FROM Places LIMIT 1 OFFSET 2), -- Pouliot
--        (SELECT item_id FROM Items LIMIT 1), --tapis
--        3;

-- -- inventory of places filter by item
-- SELECT Places.place_id, Places.place_name, Places.place_type, Inventory.nb_of_items
-- FROM Inventory
-- JOIN Places ON Inventory.place_id = Places.place_id
-- JOIN Items ON Inventory.item_id = Items.item_id
-- WHERE item_name = 'tapis'
-- ORDER BY Inventory.nb_of_items DESC;

-- SELECT item_id, item_name FROM Items ORDER BY item_name;


-- -- inventory of items filtered by place
-- SELECT Items.item_id, Items.item_name, Inventory.nb_of_items
-- FROM Inventory
-- JOIN Places ON Inventory.place_id = Places.place_id
-- JOIN Items ON Inventory.item_id = Items.item_id
-- WHERE place_name = 'Pouliot'
-- ORDER BY Inventory.nb_of_items DESC;

-- SELECT place_id, place_name, place_type FROM Places ORDER BY place_name;


