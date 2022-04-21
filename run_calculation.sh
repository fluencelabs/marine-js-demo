#! /bin/bash

echo $1
echo $2

echo "demoCalculation($1, $2)"

npx aqua run --input ./demo_calculation.aqua --func "demoCalculation(\"$1\", \"$2\")" --addr /dns4/stage.fluence.dev/tcp/19001/wss/p2p/12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv 
