rm .\node_modules\snake-wasm\ -r;
wasm-pack.exe build;
npm install;
npm run start;
