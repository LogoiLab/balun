CREATE TABLE IF NOT EXITS disappearing_channels (
  channel_id PRIMARY KEY,
  guild_id INTEGER NOT NULL,
  timecount INTEGER NOT NULL,
  timescale TEXT NOT NULL
);
