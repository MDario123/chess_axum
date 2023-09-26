CREATE TABLE games.tbl_pending_invites (
  inviter text not null,
  invited text not null,
  PRIMARY KEY(inviter, invited),
  created_at timestamp NOT NULL DEFAULT now()
);

CREATE VIEW games.v_pending_invites AS (
  SELECT inviter, invited, created_at
  FROM games.tbl_pending_invites
)
WITH CASCADED CHECK OPTION;
