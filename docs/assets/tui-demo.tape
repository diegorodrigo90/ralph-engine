# VHS tape for recording Ralph Engine TUI demo GIF
# Run: vhs docs/assets/tui-demo.tape

Output docs/assets/tui-demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Catppuccin Mocha"
Set TypingSpeed 50ms
Set Padding 10

# Launch TUI hidden, show after it's up
Hide
Type "cd ~/Projects/cp/correndo-provas && ralph-engine"
Enter
Sleep 2.5s
Show

# Idle dashboard visible
Sleep 2s

# Run a story
Type "/run plan 5.3"
Enter
Sleep 3s

# Help modal
Type "?"
Sleep 2.5s
Type " "

# Quit
Sleep 0.5s
Type "q"
Sleep 0.5s
Type "y"
Sleep 0.5s
