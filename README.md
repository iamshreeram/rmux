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
