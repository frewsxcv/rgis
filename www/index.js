import * as wasm from "../rgis/pkg/";
import proj4 from "proj4";
import proj4_defs from "proj4js-definitions";

window.proj4 = proj4;
window.proj4.defs(proj4_defs);

wasm.run();
