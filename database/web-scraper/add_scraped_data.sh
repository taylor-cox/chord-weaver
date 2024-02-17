#!/bin/bash
if ![ -d "./venv" ]; then
  echo "Creating virtual environment"
  python -m venv venv
fi
source ./venv/bin/activate
pip install -r requirements.txt
python add_scraped_content_to_db.py