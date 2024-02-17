#!/bin/bash
if ![ -d "./venv" ]; then
  echo "Creating virtual environment"
  python3 -m venv venv
fi
. ./venv/bin/activate
pip install -r requirements.txt
python add_scraped_content_to_db.py