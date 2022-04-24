#! /bin/bash

peer_id="12D3KooWHLxVhUQyAuZe6AHMB29P7wkvTNMn7eDMcsqimJYLKREf"

npx aqua run --input ./demo_calculation.aqua --func "run_on_rust_node(\"$peer_id\", \"$1\")" --addr krasnodar-4
