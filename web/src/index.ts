import { Fluence, loadWasmFromServer } from '@fluencelabs/fluence';
import { krasnodar } from '@fluencelabs/fluence-network-environment';

const relay = krasnodar[3];

const wasmPath = 'calc_service.wasm';

async function runServer() {
    await Fluence.start({
        connectTo: relay,
    });

    const service = await loadWasmFromServer(wasmPath);
    await Fluence.registerMarineService(service, 'calc');

    const peerIdMsg = `Peer ID: ${Fluence.getStatus().peerId}`;
    const relayIdMsg = `Relay ID: ${Fluence.getStatus().relayPeerId}`;

    console.log(peerIdMsg);
    console.log(relayIdMsg);

    document.getElementById('peerId').textContent = peerIdMsg;
    document.getElementById('relayId').textContent = relayIdMsg;
}

runServer();
