cargo build --profile wasm-release --target wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./webapp/ --target web --no-typescript target/wasm32-unknown-unknown/wasm-release/yogamat.wasm && \
cd webapp && \
wasm-opt -Oz -o yogamat_bg.wasm yogamat_bg.wasm && \
echo "ADD to  yogamat.js init function"
echo "async function init(input) {"
echo "    document.addEventListener("contextmenu", function (e){"
echo "        e.preventDefault();"
echo "    }, false);"
