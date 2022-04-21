import { Fluence, loadWasmFromFileSystem } from '@fluencelabs/fluence';
import { krasnodar } from '@fluencelabs/fluence-network-environment';
import path from 'path';

const relay = krasnodar[0];

const wasmPath = path.join(__dirname, '../../service/artifacts/calc_service.wasm');

async function runServer() {
    await Fluence.start({
        connectTo: relay,
    });

    const service = await loadWasmFromFileSystem(wasmPath);
    await Fluence.registerMarineService(service, 'calc');

    console.log('application started');
    console.log('peer id is: ', Fluence.getStatus().peerId);
    console.log('relay is: ', Fluence.getStatus().relayPeerId);
    console.log('press any key to quit...');
}

async function waitForKeypressAndStop() {
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.on('data', async () => {
        await Fluence.stop();
        process.exit(0);
    });
}

runServer().then(waitForKeypressAndStop);
