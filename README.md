# github-backup

Backup all your GitHub repositories with a single command

## How to create a Github personal access token

1. Go to https://github.com/settings/tokens
2. Click on the "Generate new token" button
3. Click on the "repo - Full control of private repositories" checkbox
4. Click on the "Generate token" button
5. Save your newly generated on a safe place, this will be the last time GitHub shows it to you

## How to use

```
~ github-backup help
GitHub Backup 0.2.0
Adrian Tombu <adrian@otso.fr>
Backup all your GitHub repositories with a single command

USAGE:
    github-backup <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    config    Display the config file contents
    help      Print this message or the help of the given subcommand(s)
    init      Initialize the config file
    run       Run the GitHub backup
```