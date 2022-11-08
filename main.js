import init, { run_app } from './pkg/boids_rs.js';
async function main() {
   await init('/pkg/boids_rs_bg.wasm');
   run_app();
}
main()