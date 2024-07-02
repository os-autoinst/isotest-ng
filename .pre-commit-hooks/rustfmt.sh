#!/bin/sh

# Run rustfmt on all staged files to check for formatting errors.

STAGED=$(git diff --name-only --cached | grep '.*\.rs')
if ! [ "$STAGED" = '' ]; then
  rustfmt --check "$STAGED" || {
    echo -e "\e[31mYour code is not formatted correctly! Please run rustfmt on all staged files before committing!\e[0m" ; exit 1
  }
fi
