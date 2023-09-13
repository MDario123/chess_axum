CREATE TABLE users.basic_info (
  id       bigserial PRIMARY KEY NOT NULL,
  username text      UNIQUE      NOT NULL,
  password text                  NOT NULL
)
