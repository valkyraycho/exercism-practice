#!/usr/bin/env bash
result=""
num=$1

if ((num % 3 == 0)); then
    result="${result}Pling"
fi
if ((num % 5 == 0)); then
    result="${result}Plang"
fi
if ((num % 7 == 0)); then
    result="${result}Plong"
fi

if [[ -z "$result" ]]; then
    echo "$num"
else
    echo "$result"
fi
