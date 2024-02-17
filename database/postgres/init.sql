CREATE TABLE progressions (
  id SERIAL PRIMARY KEY,
  genre VARCHAR(255) NOT NULL,
  artist VARCHAR(255) NOT NULL,
  song_name VARCHAR(255) NOT NULL,
  num_chords INTEGER NOT NULL,
  chord_roman_numeral VARCHAR(255) NOT NULL,
  link VARCHAR(255) NOT NULL
);