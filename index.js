import init, { run, get_widget_rect, get_all_widget_rects } from "rgis-pkg/rgis.js";

async function main() {
    // Initialize the wasm module
    await init();
    // Expose widget position functions for Playwright tests
    window.get_widget_rect = get_widget_rect;
    window.get_all_widget_rects = get_all_widget_rects;
    // Signal that WASM has loaded and initialized
    document.title = "rgis - ready";
    // Call the run function
    run();
}

main().catch(console.error);
