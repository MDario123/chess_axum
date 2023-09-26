CREATE TABLE users.token (
  expiration timestamp NOT NULL DEFAULT now() + '1 day'::INTERVAL,
  token text NOT NULL,
  user_id bigint NOT NULL 
                  REFERENCES users.basic_info(id) 
                  ON DELETE RESTRICT 
                  ON UPDATE CASCADE
);

CREATE INDEX token_expiration_idx 
  ON users.token(token, expiration)
  INCLUDE (user_id);
