CREATE TABLE IF NOT EXISTS games.matches
(
    id bigserial NOT NULL,
    player1 bigint NOT NULL,
    player2 bigint NOT NULL,
    game jsonb NOT NULL,
    CONSTRAINT "matches_pkey" PRIMARY KEY (id),
    CONSTRAINT "matches_player1_fkey" FOREIGN KEY (player1)
        REFERENCES users.basic_info (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT "matches_player2_fkey" FOREIGN KEY (player2)
        REFERENCES users.basic_info (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
)
