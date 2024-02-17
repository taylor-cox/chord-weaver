import unittest
import requests
import psycopg2
import os
from dotenv import load_dotenv

'''
Tests searching for Spotify URIs.
'''
class TestSpotify(unittest.TestCase):
  @classmethod
  def setUpClass(cls) -> None:
    load_dotenv()
    cls._api_connection_string = os.getenv('API_URL')
  
  def test_spotify_search(self):
    response = requests.get(
      f'{self._api_connection_string}/spotify?search=house of the rising sun',
      verify=False
    )
    self.assertEqual(
      response.json(),
      {"uri":"spotify:track:7BY005dacJkbO6EPiOh2wb"}
    )