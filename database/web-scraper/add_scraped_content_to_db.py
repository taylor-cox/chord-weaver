import psycopg
import json

def get_chord_progressions():
  with open('./progressions.json', 'r') as f:
    progressions = json.load(f)
  return progressions

def add_progressions_to_database(progressions):
  # Get the insert query for the database.
  sql_insert_query = ""
  with open('insert_scraped_content_to_db.sql') as f:
    sql_insert_query = f.read()

  # Add the chord progressions to the database.
  with psycopg.connect("postgresql://postgres:password@localhost:5432/postgres") as conn:
    with conn.cursor() as cur:
      for chord_progression in progressions:
        cur.execute(
          sql_insert_query,
          (
            chord_progression['genre'],
            chord_progression['artist'],
            chord_progression['song_name'],
            chord_progression['num_chords'],
            chord_progression['chord_roman_numerals'],
            chord_progression['link']
          )
        )
    conn.commit()

if __name__ == '__main__':
  progressions = get_chord_progressions()
  add_progressions_to_database(progressions)