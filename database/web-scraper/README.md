# Web Scraper
Scrapes the chord progressions from the [internet chord database](https://internetchorddatabase.com/). This data is saved to a JSON file, `chord_progressions.json`, which can then be added to the Postgres database with the `add_scraped_content_to_db.py` script.

## Running the Web Scraper
Python 3.12.1 was used for this implementation, along with pyenv.

To run the webscraper, in a terminal, do:
```
pip install -r requirements.txt
python web_scraper.py
python add_scraped_content_to_db.py
```

## Additional Information
All data comes from the internetchorddatabase.com.