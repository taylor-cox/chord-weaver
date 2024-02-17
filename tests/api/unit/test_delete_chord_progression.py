import unittest
import requests
import psycopg2
import os
from dotenv import load_dotenv

'''
Tests all possible DELETE requests for the API.
'''
class TestDELETE(unittest.TestCase):
  @classmethod
  def setUpClass(cls) -> None:
    load_dotenv()
    cls._db_connection_string = os.getenv('DATABASE_URL')
    cls._api_connection_string = os.getenv('API_URL')
    cls._cert_loc = os.getenv('API_CERT_LOCATION')

  def setUp(self) -> None:
    self._connection = psycopg2.connect(
      self._db_connection_string
    )
    self.cursor = self._connection.cursor()
    self.cursor.execute(
      "insert into progressions (genre, artist, song_name, num_chords, chord_roman_numeral, link) values ('Example', 'Example', 'Example', 0, 'Example', 'http://example.com/')"
    )
    self._connection.commit()
    
  def tearDown(self):
    self.cursor.execute(
      "delete from progressions where genre = 'Example' and artist = 'Example' and song_name = 'Example' and num_chords = 0 and chord_roman_numeral = 'Example' and link = 'http://example.com/'"
    )
    self._connection.commit()
    self._connection.close()

  def test_delete_progression(self):
    '''
    Tests the DELETE method for the /chord-progression endpoint.
    '''
    response = requests.delete(
      f'{self._api_connection_string}/chord-progression',
      json={
        'artist': 'Example',
        'chord_roman_numeral': 'Example',
        'genre': '',
        'link': 'http://example.com/',
        'num_chords': 0,
        'song_name': "Example"
      },
      verify=False
    )

    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.content[:32],
      b'Chord progression deleted, id = '
    )
  
  def test_add_progression_invalid(self):
    '''
    Tests the POST method for the /chord-progression endpoint
    with an invalid body.
    '''
    response = requests.post(
      f'{self._api_connection_string}/chord-progression',
      json={ },
      verify=False
    )

    self.assertEqual(
      response.status_code,
      422
    )
    
    self.assertEqual(
      response.content[:57],
      b'Failed to deserialize the JSON body into the target type:'
    )