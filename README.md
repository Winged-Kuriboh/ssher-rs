# ssher

English | [简体中文](./README_zh.md)

ssher is an easy-to-use command line tool for connecting to remote servers.

## Installation

```bash
cargo install ssher
```

## Usage

1. Select a server and connect

```bash
ssher
ssher -s <server>
```

2. Add a server

```bash
ssher add
```

3. Remove servers

```bash
# remove, rm
ssher rm
ssher rm <server_a> <server_b>
```

4. List servers

```bash
# list, ls
ssher ls
```

5. Rename a server

```bash
ssher rename
ssher rename <server_a>
```

6. Edit a server

```bash
ssher edit
ssher edit <server>
```

7. Check version

```bash
# version, v
ssher v
```

8. Help

```bash
ssher help
```

## Completions

```bash
# bash
source <(ssher completion bash)
source <(COMPLETE=bash ssher)

# zsh
source <(ssher completion zsh)
source <(COMPLETE=bash zsh)

# fish
ssher completion fish | source
source (COMPLETE=fish ssher | psub)

# powershell
ssher completion powershell > ssher.ps1
. .\ssher.ps1
$env:COMPLETE = "powershell"
ssher | Out-String | Invoke-Expression
```

> You can add the command to your shell's profile e.g. `~/.bashrc` or `~/.zshrc` to enable completions for each session.

## Configuration

The configuration file is saved in the `~/.ssher.yaml` file.

## Troubleshooting

1. MacOS default bash version is too low, you may occur the following error when using completions `source <(COMPLETE=bash ssher)`:

```bash
$ source <(COMPLETE=bash ssher)
error: Broken pipe (os error 32)
```

Solutions:

- Use the following command to upgrade bash:

```bash
brew install bash
```

- Generate completions to a file and source it:

```bash
COMPLETE=bash ssher > .ssher_bash_completion.sh
source .ssher_bash_completion.sh
```
