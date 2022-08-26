import shutil
import subprocess

print('Build started.')
path = 'pkg/bnanw.js'
path_w = '../bnan/src/assets/pkg/bnanw.js'

cargo = subprocess.run("cargo build --target=wasm32-unknown-unknown --release", shell=True)
if cargo.returncode != 0:
  print("cargo build failed!")
  quit()

bindgen = subprocess.run("wasm-bindgen target/wasm32-unknown-unknown/release/bnanw.wasm --out-dir ./pkg --target web", shell=True)
if bindgen.returncode != 0:
  print("wasm-bindgen failed!")
  quit()

shutil.copyfile('pkg/bnanw_bg.wasm', '../bnan/src/assets/pkg/bnanw_bg.wasm')
shutil.copyfile('pkg/bnanw_bg.wasm.d.ts', '../bnan/src/assets/pkg/bnanw_bg.wasm.d.ts')
shutil.copyfile('pkg/bnanw.d.ts', '../bnan/src/assets/pkg/bnanw.d.ts')

with open(path) as f:
    lines = f.readlines()

step = 0

with open(path_w, mode='w') as f:
  for l in lines:
    if step == 0 and l == 'async function init(input) {\n':
      step = 1
      f.write(l)
      f.write('    /*\n')
    elif step == 1 and l == '    }\n':
      step = 2
      f.write(l)
      f.write('    */\n')
    elif step == 2 and l == "    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {\n":
      step = 3
      f.write('    /*\n')
      f.write(l)
    elif step == 3 and l == '    const { instance, module } = await load(await input, imports);\n':
      step = 4
      f.write(l)
      f.write("    */\n")
      f.write("    const bin = await (await fetch('assets/pkg/bnanw_bg.wasm')).arrayBuffer();\n")
      f.write("    const { instance, module } = await WebAssembly.instantiate(bin, imports);\n")
    else:
      f.write(l)

print('BUild successfully end.')
