#! /bin/bash

echo $1
echo $2

echo "demoCalculation($1, $2)"

npx aqua run --input ./demo_calculation.aqua --func "demoCalculation(\"$1\", \"$2\")" --addr krasnodar-2
