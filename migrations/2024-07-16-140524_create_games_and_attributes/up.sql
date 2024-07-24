CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title VARCHAR NOT NULL,
    note VARCHAR
);

CREATE TABLE attributes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT,
    parent_id INTEGER REFERENCES attributes(id) ON DELETE CASCADE
);

CREATE TABLE game_attributes (
    game_id INTEGER REFERENCES games(id) ON DELETE CASCADE,
    attribute_id INTEGER REFERENCES attributes(id) ON DELETE CASCADE,
    PRIMARY KEY (game_id, attribute_id)
);