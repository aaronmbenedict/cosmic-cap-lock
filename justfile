name    := 'cosmic-applet-capslock'
appid   := 'com.system76.CosmicAppletCapslock'
prefix  := '/usr/local'

# Default
default: build-release

build:
    cargo build

build-release:
    cargo build --release

# Install binary + .desktop
install: build-release
    sudo install -Dm0755 target/release/{{name}} {{prefix}}/bin/{{name}}
    install -Dm0644 data/{{appid}}.desktop \
        ~/.local/share/applications/{{appid}}.desktop
    @echo ""
    @echo "Done. Add via: Cosmic Settings → Panel → Add Applet"

uninstall:
    sudo rm -f {{prefix}}/bin/{{name}}
    rm -f ~/.local/share/applications/{{appid}}.desktop

# Run standalone (smoke test outside the panel)
run: build-release
    RUST_LOG=debug ./target/release/{{name}}

# Verify Caps Lock sysfs LED on this machine
check-sysfs:
    #!/usr/bin/env bash
    echo "Scanning /sys/class/leds/ ..."
    found=0
    for dir in /sys/class/leds/*/; do
        name=$(basename "$dir")
        lower="${name,,}"
        if [[ "$lower" == *capslock* || "$lower" == *caps_lock* ]]; then
            val=$(cat "$dir/brightness" 2>/dev/null || echo "?")
            echo "  FOUND: $dir  brightness=$val"
            found=1
        fi
    done
    [[ $found -eq 0 ]] && echo "  None found. xset fallback will be used on X11."
    exit 0
