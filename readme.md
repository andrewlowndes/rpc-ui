# RPC UI
An experiment of controlling the frontend from backend by emitting RPC style requests for the UI to operate on. No WebAssembly is used (usually slower for DOM operations).

## Running
Ensure Node and Rust is installed. Install Node dependencies via `npm i`.

Bundle the js library via `npx webpack`. The FE dependencies will be bundled into a file at `public/lib.js` ready for using in the FE app.

Start the Rust service via `cargo run`. View the demo at http://localhost:3000.

## Roadmap
For this to be practical the following is also required:
- Autogenerate the typescipt component type defs to Rust types
- Add a state machine to backend and use a single endpoint to determine which rpc methods to call
