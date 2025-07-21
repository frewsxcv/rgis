import init, { run } from "rgis-pkg/rgis.js";

async function main() {
    // Initialize the wasm module
    await init();
    // Call the run function
    run();
}

main().catch(console.error);
