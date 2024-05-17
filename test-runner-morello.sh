#!/bin/bash
# set -e
NAME=`basename $1`
scp "$1" "cheri:/tmp/$NAME"
ssh cheri "/tmp/$NAME"
