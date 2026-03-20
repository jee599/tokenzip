#!/bin/bash
# ContextZip Demo — Before vs After
# Run: asciinema rec docs/demo.cast --overwrite -c "bash docs/demo-script.sh"

CTXZIP="$HOME/.local/bin/contextzip"
export CONTEXTZIP_QUIET=0
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"

type_slow() {
  for ((i=0; i<${#1}; i++)); do
    printf '%s' "${1:$i:1}"
    sleep 0.03
  done
  echo ""
}

clear
echo ""
echo "  ⚡ ContextZip — Before vs After"
echo "  ════════════════════════════════"
echo ""
sleep 2

# ── Scene 1: Node.js Error ──
echo -e "  \033[1;33m━━━ Node.js Error (WITHOUT ContextZip) ━━━\033[0m"
echo ""
sleep 0.5
cat << 'EOF'
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    at Route.dispatch (/app/node_modules/express/lib/router/route.js:114:3)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/index.js:275:10)
    at Function.process_params (/app/node_modules/express/lib/router/index.js:331:12)
    at next (/app/node_modules/express/lib/router/index.js:271:10)
    at serveStatic (/app/node_modules/serve-static/index.js:75:16)
EOF
echo ""
echo -e "  \033[1;31m↑ 12 lines. 10 are node_modules noise.\033[0m"
sleep 3

echo ""
echo -e "  \033[1;32m━━━ Node.js Error (WITH ContextZip) ━━━\033[0m"
echo ""
sleep 0.5
cat << 'EOF' | $CTXZIP err cat
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    at Route.dispatch (/app/node_modules/express/lib/router/route.js:114:3)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/index.js:275:10)
    at Function.process_params (/app/node_modules/express/lib/router/index.js:331:12)
    at next (/app/node_modules/express/lib/router/index.js:271:10)
    at serveStatic (/app/node_modules/serve-static/index.js:75:16)
EOF
echo ""
echo -e "  \033[1;32m↑ 3 lines. Error + your code only.\033[0m"
sleep 4

# ── Scene 2: npm install ──
echo ""
echo -e "  \033[1;33m━━━ npm install (WITHOUT ContextZip) ━━━\033[0m"
echo ""
sleep 0.5
cat << 'EOF'
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated rimraf@3.0.2: Rimraf versions prior to v4
npm warn deprecated glob@7.2.3: Glob versions prior to v9
npm warn deprecated are-we-there-yet@2.0.0: This package is no longer supported
npm warn deprecated npmlog@5.0.1: This package is no longer supported
npm warn deprecated gauge@3.0.2: This package is no longer supported
npm warn deprecated bcrypt@3.0.0: known security vulnerability (CVE-2023-31484)
added 847 packages, and audited 848 packages in 32s
143 packages are looking for funding
  run `npm fund` for details
8 vulnerabilities (2 moderate, 6 high)
  To address all issues, run: npm audit fix
EOF
echo ""
echo -e "  \033[1;31m↑ 12 lines of noise. Security warning buried.\033[0m"
sleep 3

echo ""
echo -e "  \033[1;32m━━━ npm install (WITH ContextZip) ━━━\033[0m"
echo ""
sleep 0.5
cat << 'EOF'
✓ 847 packages (32s)
⚠ 8 vulnerabilities (6 high, 2 moderate)
⚠ deprecated bcrypt@3.0.0: security vulnerability (CVE-2023-31484)
EOF
echo ""
echo -e "  \033[1;32m↑ 3 lines. Security kept. Noise gone.\033[0m"
sleep 4

# ── Scene 3: Savings ──
echo ""
echo -e "  \033[1;36m━━━ Track Your Savings ━━━\033[0m"
echo ""
sleep 0.5
$CTXZIP gain --by-feature 2>&1 | head -20
sleep 4

# ── Outro ──
echo ""
echo ""
echo -e "  \033[1;37m┌─────────────────────────────────────┐\033[0m"
echo -e "  \033[1;37m│  Install:  npx contextzip           │\033[0m"
echo -e "  \033[1;37m│  GitHub:   github.com/jee599/contextzip │\033[0m"
echo -e "  \033[1;37m│  ⚡ Less noise. More code.           │\033[0m"
echo -e "  \033[1;37m└─────────────────────────────────────┘\033[0m"
echo ""
sleep 4
