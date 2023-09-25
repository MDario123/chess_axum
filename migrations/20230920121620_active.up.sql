CREATE TABLE games.t_active (
  id bigserial NOT NULL PRIMARY KEY,
  player_w text NOT NULL,
  player_b text NOT NULL,
  start_pos text NOT NULL,
  fen text NOT NULL,
  FOREIGN KEY(player_w) REFERENCES users.basic_info(username),
  FOREIGN KEY(player_b) REFERENCES users.basic_info(username),
  UNIQUE(player_w, player_b)
);

CREATE INDEX ON games.t_active(player_w);
CREATE INDEX ON games.t_active(player_b);

CREATE TABLE games.t_moves (
  id_game bigint NOT NULL REFERENCES games.t_active(id),
  move_num int NOT NULL,
  san varchar(8) NOT NULL,
  previous_fen text NOT NULL,
  UNIQUE(id_game, move_num)
);
