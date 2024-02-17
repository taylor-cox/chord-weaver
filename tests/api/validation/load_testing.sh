#!/bin/bash
parallel --jobs 0 curl -X GET 0.0.0.0:3000/chord-progression?id={1} ::: {1..100} :: {1..100}