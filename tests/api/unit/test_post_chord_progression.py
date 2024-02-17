import unittest
import requests
import psycopg2
import os
from dotenv import load_dotenv

'''
Tests all possible POST requests for the API.
'''
class TestPOST(unittest.TestCase):
  @classmethod
  def setUpClass(cls):
    load_dotenv()
    cls._db_connection_string = os.getenv('DATABASE_URL')
    cls._api_connection_string = os.getenv('API_URL')
    cls._cert_loc = os.getenv('API_CERT_LOCATION')
    cls._connection = psycopg2.connect(
      cls._db_connection_string
    )

  @classmethod
  def tearDownClass(cls):
    cls.cursor = cls._connection.cursor()
    cls.cursor.execute('DELETE FROM progressions WHERE id > 401')
    cls._connection.commit()
    cls._connection.close()

  def test_add_progression(self):
    '''
    Tests the POST method for the /chord-progression endpoint
    with a valid body.
    '''
    response = requests.post(
      f'{self._api_connection_string}/chord-progression',
      json={
        'artist': 'Example',
        'chord_roman_numeral': 'Example',
        'genre': '',
        'link': 'http://example.com/',
        'num_chords': 0,
        'song_name': "Example Song"
      },
      verify=False
    )

    self.assertEqual(
      response.status_code,
      201
    )
    self.assertEqual(
      response.content[:31],
      b'Chord progression created, id ='
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