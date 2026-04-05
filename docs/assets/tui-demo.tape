# VHS tape for recording Ralph Engine TUI demo GIF
# Install: pacman -S vhs
# Run: vhs docs/assets/tui-demo.tape

Output docs/assets/tui-demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set TypingSpeed 60ms
Set Padding 10

# Build first
Hide
Type "cd ~/Projects/ralph-engine && cargo build --release 2>/dev/null"
Enter
Sleep 2s
Show

# Launch TUI
Type "cargo run"
Enter
Sleep 2s

# Show the idle dashboard
Sleep 1.5s

# Type /help to show commands
Type "/help"
Enter
Sleep 2s

# Type /doctor to check health
Type "/doctor"
Enter
Sleep 2s

# Open help modal
Type "?"
Sleep 2s

# Close help modal
Type "q"
Sleep 0.5s

# Try typing a message
Type "hello world"
Enter
Sleep 1.5s

# Type /plugins to see installed plugins
Type "/plugins"
Enter
Sleep 2s

# Scroll up
Up Up Up
Sleep 1s

# Focus blocks with j/k
Type "j"
Sleep 0.5s
Type "j"
Sleep 0.5s
Type "k"
Sleep 0.5s

# Press G to follow
Type "G"
Sleep 1s

# Quit
Type "q"
Sleep 0.5s
Type "y"
Sleep 1s
