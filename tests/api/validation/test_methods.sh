#!/bin/sh

curl -X "GET" \
  https://0.0.0.0:3000/chord-progression?id=1 \
  --insecure | jq .

curl \
  -X "POST" \
  --header 'Content-Type: application/json' \
  --data @example_progression.json \
  https://0.0.0.0:3000/chord-progression \
  --insecure 