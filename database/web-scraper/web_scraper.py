import requests
from bs4 import BeautifulSoup
from pathlib import Path
from dataclasses import dataclass, field
from typing import List
import hashlib
import json

'''
Check if a cached version of the website exists.
'''
def check_if_page_exists(path: str) -> bool:
  return Path(path).exists()

'''
Get the contents of a file.
'''
def get_file_contents(path: str) -> str:
  with open(path, "r") as file:
    return file.read()

'''
Caches the contents of a page to a file.
'''
def cache_page_contents(path: str, url: str) -> str:
  with open(path, "w") as file:
    page = requests.get(url)
    file.write(page.text)
    return page.text

'''
Dataclass to represent a chord progression.
'''
@dataclass(unsafe_hash=True)
class ChordProgression:
  genre: str = field(hash=True)
  artist: str = field(hash=True)
  song_name: str = field(hash=True)
  num_chords: int = field(hash=True)
  chord_roman_numerals: str = field(hash=True)
  link: str = field(hash=True)

'''
Scrape the page.
'''
def scrape_page(page: str) -> List[ChordProgression]:
  soup = BeautifulSoup(page, "html.parser")
  results = soup.find('table', class_="List").find_all('tr')

  chord_progressions = []

  for tr in results[1:]:
    chord_progression = None
    row = [td.text for td in tr.find_all('td')[:-1]]
    link = tr['onclick'].split("=")[1].strip()[1:-1]
    link = f"https://internetchorddatabase.com{link}"
    m = hashlib.sha256()
    m.update(link.encode('utf-8'))
    print(m.hexdigest())
    chord_roman_numerals = scrape_roman_numerals(
      cache_page_contents(f'./cache/{m.hexdigest()}.html', link)
      if not check_if_page_exists(f'./cache/{m.hexdigest()}.html') else
      get_file_contents(f'./cache/{m.hexdigest()}.html')
    )

    chord_progression = ChordProgression(
      genre=row[0],
      artist=row[1],
      song_name=row[2],
      num_chords=int(row[3]),
      chord_roman_numerals=chord_roman_numerals,
      link=link
    )
    chord_progressions.append(chord_progression)

  return chord_progressions

'''
Scraper for the roman numerals.
'''
def scrape_roman_numerals(page: str) -> str:
  soup = BeautifulSoup(page, "html.parser")
  text = soup.find('div', class_="PageContent").text
  text = text[text.find("Roman Numerals:"):text.find("Instrument")].strip()
  if text == '':
    return None
  text = text.split(":")[1].strip()
  return text

if __name__ == "__main__":
  if check_if_page_exists('./chord_progressions.json'):
    print("Already scraped the website.")
    exit()
  # Check if a chached version of the website exists.
  URL = "https://internetchorddatabase.com/Songs/Default.aspx"
  file_exists = check_if_page_exists("./cache/website.html")
  if file_exists:
    page = get_file_contents("./cache/website.html")
  else:
    page = cache_page_contents("./cache/website.html", URL)
  
  # Get the page contents.
  chord_progressions = scrape_page(page)
  print(chord_progressions)

  # Save the chord progressions to a json file.
  with open('progressions.json', 'w') as file:
    json.dump(chord_progressions, file, default=lambda o: o.__dict__)
