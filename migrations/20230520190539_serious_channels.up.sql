-- Add up migration script here
CREATE TABLE serious_channels (
  channel_id PRIMARY KEY,
  guild_id INTEGER NOT NULL
);
