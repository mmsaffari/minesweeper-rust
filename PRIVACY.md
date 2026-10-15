# Privacy policy

*Classic Mine Sweeper (minesweeper-rust). Last updated: October 15, 2026.*

The game does not collect, store or share any personal information. There are no accounts,
no ads, no analytics and no tracking.

## What the game saves on your PC

- **Best times** (`best_times.json`): your fastest win for each difficulty.
- **Settings** (`settings.json`): whether the update check is on (GitHub version only).

Both files stay on your PC, in the game's data folder. They are never sent anywhere. In the
Microsoft Store version, Windows keeps them in the app's own storage and deletes them when you
uninstall the game.

## Network connections

- **Microsoft Store version:** the game makes no network connections at all. The Store
  itself installs and updates it.
- **GitHub version** (the `.zip` from GitHub Releases): once at startup, the game asks
  GitHub's public API (`api.github.com`) for the latest release number, to tell you when a
  newer version is out. The request contains nothing about you. Like any web request, it
  reaches GitHub with your IP address and the game's name and version; GitHub handles that
  under the [GitHub Privacy Statement](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement).
  You can turn this off with **Help → Check for updates**. If a newer version is found,
  **Help → Get new version...** opens its page in your web browser, only when you choose it.

## Clipboard

**Game → Copy board** (or `Ctrl+C`) puts the board as text on your clipboard, only when you ask
for it. The game never reads the clipboard.

## Changes and contact

If this policy changes, the new version will be published at this address with a new date.
Questions: open an issue at <https://github.com/mmsaffari/minesweeper-rust/issues>.
