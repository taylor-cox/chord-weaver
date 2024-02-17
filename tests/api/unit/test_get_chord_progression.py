import unittest
import requests
import os
from dotenv import load_dotenv

'''
Tests all possible GET requests for the API.
'''
class TestGET(unittest.TestCase):
  @classmethod
  def setUpClass(cls) -> None:
    load_dotenv()
    cls._db_connection_string = os.getenv('DATABASE_URL')
    cls._api_connection_string = os.getenv('API_URL')
    cls._cert_loc = os.getenv('API_CERT_LOCATION')
  
  def test_get_id(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with an id query parameter.
    '''
    # Testing getting a known id.
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?id=1',
      verify=False
    )
    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.json(),
      [{
        'artist': 'Animals, The',
        'chord_roman_numeral': 'i, III, IV, V7, VI',
        'genre': '',
        'id': 1,
        'link': 'https://internetchorddatabase.com/Songs/The_Animals_House_of_the_Rising_Sun_by_Animals_The_v1',
        'num_chords': 5,
        'song_name': "The Animals' House of the Rising Sun"
      }]
    )
  
  def test_get_id_invalid(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with an invalid id query parameter.
    '''
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?id="1"',
      verify=False
    )
    self.assertEqual(
      response.status_code,
      400
    )
    self.assertEqual(
      response.content,
      b'Failed to deserialize query string: invalid digit found in string'
    )
  
  def test_get_artist(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with a song query parameter.
    '''
    # Testing getting a known song.
    response = requests.get(
      f"{self._api_connection_string}/chord-progression?artist=Animals",
      verify=False
    )
    
    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.json(),
      [
        {
          "artist":"Animals, The",
          "chord_roman_numeral":"i, III, IV, V7, VI",
          "genre":"",
          "id":1,
          "link":"https://internetchorddatabase.com/Songs/The_Animals_House_of_the_Rising_Sun_by_Animals_The_v1",
          "num_chords":5,
          "song_name":"The Animals\' House of the Rising Sun"
        },
        {
          "artist":"Animals, The",
          "chord_roman_numeral":"I, ii, III7, IV, V, vi",
          "genre":"",
          "id":2,
          "link":"https://internetchorddatabase.com/Songs/Dont_Let_Me_Be_Misunderstood_by_Animals_The_v1",
          "num_chords":6,
          "song_name":"Don\'t Let Me Be Misunderstood"
        }
      ]
    )

  def test_get_artist_invalid(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with an invalid song query parameter.
    '''
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?artist="Animals"',
      verify=False
    )
    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.json(),
      []
    )

  def test_get_song(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with a song query parameter.
    '''
    # Testing getting a known song.
    response = requests.get(
      f"{self._api_connection_string}/chord-progression?song=The%20Animals\'%20House%20of%20the%20Rising%20Sun",
      verify=False
    )
    
    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.json(),
      [
        {
          "artist":"Animals, The",
          "chord_roman_numeral":"i, III, IV, V7, VI",
          "genre":"",
          "id":1,
          "link":"https://internetchorddatabase.com/Songs/The_Animals_House_of_the_Rising_Sun_by_Animals_The_v1",
          "num_chords":5,
          "song_name":"The Animals\' House of the Rising Sun"
        }
      ]
    )

  def test_get_song_invalid(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with an invalid song query parameter.
    '''
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?song="The Animals\' House of the Rising Sun"',
      verify=False
    )
    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.json(),
      []
    )

  def test_get_num_chords(self):
    '''
    Tests the GET method for the /chord-progression endpoint 
    with a num_chords query parameter.
    '''
    # Testing getting a known song.
    response = requests.get(
      f"{self._api_connection_string}/chord-progression?num_chords=22",
      verify=False
    )

    self.assertEqual(
      response.status_code,
      200
    )
    self.assertEqual(
      response.json(),
      [
        {
          'artist': 'Leroy Anderson',
          'chord_roman_numeral': 'I, A1, I6, Imaj7, Imaj9, II, A2, II7, ii7, m3, III, III7, iii7, IV, A4, V, V6, V7, vi, VI7, VII7',
          'genre': 'Christmas',
          'id': 88,
          'link': 'https://internetchorddatabase.com/Songs/Sleigh_Ride_by_Leroy_Anderson',
          'num_chords': 22,
          'song_name': 'Sleigh Ride'
        }
      ]
    )
  
  def test_get_num_chords_invalid(self):
    '''
    Tests the GET method for the /chord-progression endpoint
    with an invalid num_chords query parameter.
    '''
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?num_chords="22"',
      verify=False
    )
    self.assertEqual(
      response.status_code,
      400
    )
    self.assertEqual(
      response.content,
      b'Failed to deserialize query string: invalid digit found in string'
    )

  def test_get_progression(self):
    '''
    Tests the GET method for the /chord-progression endpoint
    with a progression query parameter.
    '''
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?progression=I,%20A1,%20I6,%20Imaj7,%20Imaj9,%20II,%20A2,%20II7,%20ii7,%20m3,%20III,%20III7,%20iii7,%20IV,%20A4,%20V,%20V6,%20V7,%20vi,%20VI7,%20VII7',
      verify=False
    )

    self.assertEqual(
      response.status_code,
      200
    )
    
    self.assertEqual(
      response.json(),
      [
        {
          "artist":"Leroy Anderson",
          "chord_roman_numeral":"I, A1, I6, Imaj7, Imaj9, II, A2, II7, ii7, m3, III, III7, iii7, IV, A4, V, V6, V7, vi, VI7, VII7",
          "genre":"Christmas",
          "id":88,
          "link":"https://internetchorddatabase.com/Songs/Sleigh_Ride_by_Leroy_Anderson",
          "num_chords":22,
          "song_name":"Sleigh Ride"
        }
      ]
    )

  def test_get_progression_invalid(self):
    '''
    Tests the GET method for the /chord-progression endpoint
    with an invalid progression query parameter.
    '''
    response = requests.get(
      f'{self._api_connection_string}/chord-progression?progression=1',
      verify=False
    )
    self.assertEqual(
      response.status_code,
      200
    )