#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

sk=KlJDPvy7PXGesmQUzsDYqSCKq2AkRagPSOMsTGLd4A0= 

aqua remote deploy_service \
    --addr '/dns4/kras-02.fluence.dev/tcp/19001/wss/p2p/12D3KooWHLxVhUQyAuZe6AHMB29P7wkvTNMn7eDMcsqimJYLKREf'  \
    --data-path configs/deployment_cfg.json \
    --service marine-js-demo \
    --sk $sk
