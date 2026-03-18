#!/bin/sh
# tokenzip installer
# Usage: curl -fsSL https://raw.githubusercontent.com/jee599/tokenzip/main/install.sh | sh
set -e

REPO="jee599/tokenzip"
BINARY_NAME="tokenzip"
INSTALL_DIR="$HOME/.local/bin"
VERSION="${TOKENZIP_VERSION:-latest}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

info()  { printf "${GREEN}✓${NC} %s\n" "$1"; }
warn()  { printf "${YELLOW}⚠${NC} %s\n" "$1"; }
error() { printf "${RED}✗${NC} %s\n" "$1" >&2; exit 1; }

# --- OS / Arch detection ---

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)  PLATFORM="linux" ;;
        Darwin) PLATFORM="macos" ;;
        *)      error "Unsupported OS: $OS" ;;
    esac

    case "$ARCH" in
        x86_64|amd64)  ARCH="x86_64" ;;
        arm64|aarch64) ARCH="aarch64" ;;
        *)             error "Unsupported architecture: $ARCH" ;;
    esac

    TARGET="tokenzip-${PLATFORM}-${ARCH}"
}

# --- Download ---

download_binary() {
    if [ "$VERSION" = "latest" ]; then
        URL="https://github.com/${REPO}/releases/latest/download/${TARGET}"
    else
        URL="https://github.com/${REPO}/releases/download/${VERSION}/${TARGET}"
    fi

    TEMP_DIR=$(mktemp -d)
    TEMP_BIN="${TEMP_DIR}/${BINARY_NAME}"

    printf "Downloading %s ...\n" "$TARGET"
    if ! curl -fsSL "$URL" -o "$TEMP_BIN"; then
        rm -rf "$TEMP_DIR"
        error "Failed to download from $URL"
    fi

    mkdir -p "$INSTALL_DIR"
    mv "$TEMP_BIN" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    rm -rf "$TEMP_DIR"

    info "tokenzip installed to ~/.local/bin/tokenzip"
}

# --- PATH check ---

check_path() {
    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            warn "~/.local/bin is not in your PATH"
            printf "  Add this to your shell profile:\n"
            printf "    export PATH=\"\$HOME/.local/bin:\$PATH\"\n\n"
            ;;
    esac
}

# --- RTK migration ---

detect_rtk() {
    RTK_SETTINGS="$HOME/.claude/settings.json"
    HAS_RTK=false

    if command -v rtk >/dev/null 2>&1; then
        HAS_RTK=true
    fi

    if [ -f "$RTK_SETTINGS" ] && grep -q "rtk-rewrite" "$RTK_SETTINGS" 2>/dev/null; then
        HAS_RTK=true
    fi

    if [ "$HAS_RTK" = false ]; then
        return
    fi

    printf "\n${YELLOW}Existing RTK installation detected.${NC}\n"
    printf "  1) Replace RTK with tokenzip (recommended)\n"
    printf "  2) Keep both (coexist)\n"
    printf "  3) Cancel installation\n"
    printf "Choose [1/2/3]: "

    # Non-interactive fallback: default to replace
    if [ ! -t 0 ]; then
        printf "1 (non-interactive, defaulting to replace)\n"
        CHOICE="1"
    else
        read -r CHOICE
    fi

    case "$CHOICE" in
        1)
            # Replace: swap rtk-rewrite.sh → tokenzip-rewrite.sh in settings
            if [ -f "$RTK_SETTINGS" ]; then
                sed -i.bak 's/rtk-rewrite\.sh/tokenzip-rewrite.sh/g' "$RTK_SETTINGS" 2>/dev/null || \
                sed -i '' 's/rtk-rewrite\.sh/tokenzip-rewrite.sh/g' "$RTK_SETTINGS" 2>/dev/null || true
                rm -f "${RTK_SETTINGS}.bak"
            fi
            # Remove old rtk hook if it exists
            OLD_HOOK="$HOME/.claude/hooks/bash/rtk-rewrite.sh"
            if [ -f "$OLD_HOOK" ]; then
                rm -f "$OLD_HOOK"
                info "Removed old RTK hook"
            fi
            info "Replaced RTK with tokenzip"
            ;;
        2)
            info "Keeping both RTK and tokenzip"
            ;;
        3)
            printf "Installation cancelled.\n"
            exit 0
            ;;
        *)
            error "Invalid choice. Aborting."
            ;;
    esac
}

# --- Hook installation ---

install_hook() {
    "${INSTALL_DIR}/${BINARY_NAME}" init -g --hook-only --auto-patch
    info "Claude Code hook installed"
}

# --- Success message ---

print_success() {
    printf "\n"
    info "tokenzip installed to ~/.local/bin/tokenzip"
    info "Claude Code hook installed"
    info "Ready! Restart Claude Code to activate."
    printf "\n"
    printf "  Quick check:  ${BOLD}tokenzip gain${NC}\n"
    printf "  Full status:  ${BOLD}tokenzip init --show${NC}\n"
    printf "\n"
}

# --- Main ---

main() {
    detect_platform
    download_binary
    check_path
    detect_rtk
    install_hook
    print_success
}

main
