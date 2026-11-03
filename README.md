# rmux

rmux = remote tmux

Lists the tmux sessions on a remote host as a numbered menu and attaches to the one you pick.
It uses the system `ssh`: `tmux list-sessions` runs remotely, then `ssh -t` runs `tmux attach`.

## Build

```sh
cargo build --release
```

## Usage

```sh
rmux <host> [user]
```

`host` may be a hostname, an IP, `user@host`, or an `~/.ssh/config` alias.
If `user` is given the target is `user@host`; otherwise ssh picks the user.

## Environment

`RMUX_TMUX`: path to `tmux` on the remote (default `tmux`). Set it when tmux
is not on the PATH of non-interactive ssh, e.g. `RMUX_TMUX=/usr/local/bin/tmux`. You need local `ssh` and key or agent access.

## Install / Releases

Pre-built binaries are attached to each GitHub Release (tags `vX.Y.Z`). Download the archive for your platform:

- Linux x86_64: `rmux-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
- Linux aarch64: `rmux-vX.Y.Z-aarch64-unknown-linux-gnu.tar.gz`
- macOS Apple silicon: `rmux-vX.Y.Z-aarch64-apple-darwin.tar.gz`
- macOS Intel: `rmux-vX.Y.Z-x86_64-apple-darwin.tar.gz`
- Windows x86_64: `rmux-vX.Y.Z-x86_64-pc-windows-msvc.zip`

Verify the download with its `.sha256` file from the same release (download both into one folder):

```sh
shasum -a 256 -c rmux-vX.Y.Z-<target>.tar.gz.sha256
```

`SHA256SUMS` lists all five archives, so it only verifies cleanly if you downloaded every archive.

The binaries are not signed. On macOS, if Gatekeeper blocks `./rmux`, remove the quarantine flag:

```sh
xattr -d com.apple.quarantine ./rmux
```

On Windows, SmartScreen may show a warning. Choose "More info" and then "Run anyway" if you trust the download.

### How to release

1. Bump `version` in `Cargo.toml`, then commit.
2. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`. The tag must match the Cargo.toml version.
3. The `release` workflow builds all five platforms and publishes the release.

To test the build without publishing, open the Actions tab and use **Run workflow**. This is a build-only dry run.
