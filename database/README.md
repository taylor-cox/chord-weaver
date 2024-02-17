# Database
Database containing all the information for chord progressions. Also contains web scraper for scraping chords from the Internet Chord Database.

The Postgres version used is 16.2, with Alpine Linux to keep the image size small.

## Getting Started
To start, run the `run.sh` file located in this folder.

## Details
The `postgres/data` folder contains stored database information.

The `postgres/init.sql` file initializes the database with the `chord_progressions` table.

The `web-scraper` folder contains the information needed to reconstruct the database if necessary.