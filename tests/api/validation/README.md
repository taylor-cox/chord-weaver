# Validation Tests
Simple sanity checks that APIs are working properly, and they are efficient.

## How To Run
Contains Linux bash shell scripts. [GNU Parallel](https://www.gnu.org/software/parallel/) must be installed.
Ensure they have the correct permissions:
```
$ chmod +x ./load_testing.sh
$ chmod +x ./test_methods.sh 
```
Run them:
```
$ ./load_testing.sh
$ ./test_methods.sh
```