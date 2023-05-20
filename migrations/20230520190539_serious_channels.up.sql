-- Add up migration script here
CREATE IF NOT EXISTS TABLE serious_channels (
  channel_id PRIMARY KEY,
  guild_id INTEGER NOT NULL
);
