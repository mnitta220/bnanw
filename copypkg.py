import shutil

print('Hello, world!')
print(1 + 2)
path = 'pkg/bnanw.js'
path_w = '../bnan/src/assets/pkg/bnanw.js'
shutil.copyfile('pkg/bnanw_bg.wasm', '../bnan/src/assets/pkg/bnanw_bg.wasm')
shutil.copyfile('pkg/bnanw_bg.wasm.d.ts', '../bnan/src/assets/pkg/bnanw_bg.wasm.d.ts')
shutil.copyfile('pkg/bnanw.d.ts', '../bnan/src/assets/pkg/bnanw.d.ts')

'''
with open(path) as f:
    s = f.read()
    print(type(s))
    print(s)
with open(path) as f:
    l = f.readlines()
    print(type(l))
    print(l)

with open(path) as f:
    l_strip = [s.strip() for s in f.readlines()]
    print(l_strip)

with open(path) as f:
    l = f.readlines()
    print(l[1])

with open(path) as f:
    for s_line in f:
        print(s_line.strip())
with open(path) as f:
    s_line = f.readline()
    print(s_line)
    s_line = f.readline()
    print(s_line)

with open(path) as f:
    while True:
        s_line = f.readline()
        print(s_line)
        if not s_line:
            break
with open(path) as f:
    for s_line in f:
        print(s_line)
'''

with open(path) as f:
    lines = f.readlines()

# s = 'New file'
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

#with open(path_w) as f:
#    print(f.read())
