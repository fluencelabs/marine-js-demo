# Marine JS demo

Marine JS allows to run marine wasm services on JS peers.

## Why Marine-JS?

Our goal is to make the Fluence JS peer as capable as the Fluence Rust peer. Previously there was no way to run wasm services on the JS peer. Now this feature is available and it's now easier to treat both types of peers as the same.

If you want to run services both on Rust and JS peers, this feature will surely help you. Marine-JS unlocks the ability to make mind-blowing things with aqua and deploy services dynamically on any type of peers (edited)

## Building

First, build the `calc-service`:

```bash
cd service
./build
```

To build and run nodejs example:

```bash
cd node
npm i
npm start
```

The application will start. Peer Id and Relay Id will be printed to console:

```
application started
peer id is:  12D3KooWH71ZkJatk73EfviugJiQ1hGFWCTFDWdsZVgX4JrYZYGm
relay is:  12D3KooWFEwNWcHqi9rtsmDhsYcDbRUCDXH84RC4FW6UfsFWaoHi
press any key to quit...
```

To build and run web example:

```bash
cd web
npm i
npm start
```

The browser window will open. Peer and and Relay Id will be shown in the page:

![Web page screenshot](screen.png)
