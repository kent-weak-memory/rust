#!/bin/sh

mkdir -p /tmp/build || exit 1
gcc -Wall -Wextra -O2 main.c -o /tmp/build/main || exit 1
