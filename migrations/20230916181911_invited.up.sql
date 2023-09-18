CREATE VIEW games.invited AS (
  SELECT 
    bi.username AS inv_player,
    am.id AS game_id
  FROM games.active_matches am
    JOIN users.basic_info bi ON bi.id = am.player2
)
