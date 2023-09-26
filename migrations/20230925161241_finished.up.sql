CREATE TABLE games.t_finished (
  id bigint NOT NULL,
  player_w text NOT NULL,
  player_b text NOT NULL,
  start_pos varchar(127) NOT NULL,
  moves text NOT NULL,
  end_date timestamp NOT NULL DEFAULT now()
);
