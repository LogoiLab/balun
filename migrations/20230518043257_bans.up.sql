CREATE TABLE IF NOT EXISTS bans (
  user_id PRIMARY KEY,
  guild_id INTEGER,
  global bool DEFAULT 0
);
