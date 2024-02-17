#!/bin/bash
echo "Running this application requires Spotify Client ID and Secret. Please enter them below:"
read -p "Spotify Client ID: " SPOTIFY_CLIENT_ID
read -p "Spotify Client Secret: " SPOTIFY_CLIENT_SECRET
touch ./api/.spotify-env
printf "SPOTIFY_CLIENT_ID=$SPOTIFY_CLIENT_ID\nSPOTIFY_CLIENT_SECRET=$SPOTIFY_CLIENT_SECRET" > ./api/.spotify-env

echo "Generating SSL certs for API and Website"
cd ./api/ssl-certs
./generate_certs.sh >/dev/null
cd ../../website/ssl-certs
./generate_certs.sh >/dev/null

echo "Adding data to database. This may take a while"
cd ../../database
docker compose up -d
sleep 15
cd ./web-scraper
pwd
./add_scraped_data.sh
cd ..
pwd
docker compose down
docker compose rm -f