# iNotifier
OSX Agent to handle irssi notifications on a remote server

## Installation

### Part 1: Installing inotify-agent on your Mac

1. Go to ~/Library/LaunchAgents
2. Create the file com.ntrippar.inotify.plist
3. Paste the following into the file and fix the path of the inotify-agent binary:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.ntrippar.inotify</string>
  <key>ProgramArguments</key>
  <array>
    <string>/absolute/path/to/inotify_agent</string>
  </array>
  <key>StandardErrorPath</key>
  <string>/dev/null</string>
  <key>StandardOutPath</key>
  <string>/dev/null</string>
  <key>KeepAlive</key>
  <true/>
</dict>
</plist>
```
4. load the agent to the user account:
`launchctl load -F ~/Library/LaunchAgents/com.ntrippar.inotify.plist`


### Part 2: Installing the irssi inotify plugin on your remote server
1. connect to your remote server doing a remote port forwarding of the port 31337
`ssh -R 31337:localhost:31337 username@server`
2. move the file `iNotify.pl` to `~/.irssi/scripts/`
3. load the plugin

