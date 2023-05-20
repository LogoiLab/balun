CREATE TABLE IF NOT EXISTS disappearing_channels (
  channel_id INTEGER PRIMARY KEY,
  guild_id INTEGER NOT NULL,
  timecount INTEGER NOT NULL,
  timescale TEXT NOT NULL
);
