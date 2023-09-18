CREATE TABLE IF NOT EXISTS games.matches
(
    id bigserial NOT NULL,
    player1 bigint NOT NULL,
    player2 bigint NOT NULL,
    pgn text NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (player1)
        REFERENCES users.basic_info (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY (player2)
        REFERENCES users.basic_info (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS games.active_matches
(
    id bigserial NOT NULL,
    player1 bigint NOT NULL,
    player2 bigint NOT NULL,
    game text NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (player1)
        REFERENCES users.basic_info (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY (player2)
        REFERENCES users.basic_info (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    UNIQUE(player1, player2)
);
