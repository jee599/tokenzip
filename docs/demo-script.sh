#!/bin/bash
# ContextZip Demo Recording Script
# Run: asciinema rec docs/demo.cast -c "bash docs/demo-script.sh"

export CONTEXTZIP_QUIET=1
export PATH="$HOME/.local/bin:$PATH"

clear
echo ""
echo "  ⚡ ContextZip Demo"
echo "  ─────────────────────"
echo ""
sleep 2

# Scene 1: Version
echo "  $ contextzip --version"
sleep 0.5
contextzip --version
sleep 1.5
echo ""

# Scene 2: Node.js error compression
echo "  $ contextzip err node broken-app.js"
sleep 0.5
cat << 'EOF' | contextzip err cat
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
sleep 2
echo ""

# Scene 3: git status compression
echo "  $ contextzip git status"
sleep 0.5
contextzip git status
sleep 2
echo ""

# Scene 4: gain dashboard
echo "  $ contextzip gain --by-feature"
sleep 0.5
contextzip gain --by-feature 2>&1 | head -20
sleep 3
echo ""

echo "  ⚡ Install: npx contextzip"
echo "  📦 GitHub: github.com/jee599/contextzip"
echo ""
sleep 3
