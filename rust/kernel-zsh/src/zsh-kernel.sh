#!/usr/bin/env zsh

print "\U10ACDC" | tee /dev/stderr
while read -r code
do
  print -v unescaped "$code"
  eval "$unescaped"
  print "\U10ABBA" | tee /dev/stderr
done < "${1:-/dev/stdin}"
