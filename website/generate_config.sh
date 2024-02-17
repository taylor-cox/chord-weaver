#!/bin/bash -eu

# Reset config file
> ./public/config.js

# Find all the MY_APP_ environment variables in the environment
declare -a customVars
for key in $(env | awk -F "=" '{print $1}' | grep ".*MY_APP_.*")
do
  customVars+=($key)
done

# Recreate a new config.js
for key in "${customVars[@]}"
do
  echo "window.$key='${!key}';" >> ./public/config.js
done