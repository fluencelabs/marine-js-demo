#! /bin/bash

npx aqua run --input ./demo_calculation.aqua --func "run_on_js_peer(\"$1\", \"$2\")" --addr krasnodar-4
