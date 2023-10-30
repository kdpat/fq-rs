CREATE TABLE games (
    id INTEGER PRIMARY KEY,
    host_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    player_ids JSON NOT NULL,
    opts JSON NOT NULL,
    rounds JSON NOT NULL,
    FOREIGN KEY(host_id) REFERENCES users(id)
);