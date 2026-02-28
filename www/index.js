import init, { run } from "rgis-pkg/rgis.js";

async function main() {
    // Initialize the wasm module
    await init();
    // Signal that WASM has loaded and initialized
    document.title = "rgis - ready";
    // Call the run function
    run();
}

main().catch(console.error);
