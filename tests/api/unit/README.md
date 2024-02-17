# Unit Tests
Uses Python 3.12.1 + requests for unit testing the API.

## How to run
Install Python 3.12.1 (I used pyenv), then:
```
You can skip this if you don't want to run in a venv. Instructions for Linux.
$ python -m venv venv
$ . ./venv/bin/activate
```
```
$ pip install -r requirements.txt
$ ./run_tests.sh
```
The `run_tests.sh` file is a very simple bash script for running unit tests in Python. If you are running on another operating system, simply copy-paste from the .sh file into the command line.