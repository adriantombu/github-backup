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
GitHub Backup 0.1.0
Adrian Tombu <adrian@otso.fr>
Backup all your GitHub repositories with a single command

USAGE:
    github-backup [OPTIONS] --username <USERNAME> --token <TOKEN>

OPTIONS:
        --archive-format <ARCHIVE_FORMAT>
            [default: zip] [possible values: tar, zip]

        --archive-ref <ARCHIVE_REF>
            HEAD | main | ... [default: HEAD]

        --backup-path <BACKUP_PATH>
            Path to save the files [default: github-export]

        --backup-type <BACKUP_TYPE>
            Choose between a full clone (git) or a simple archive without history (archive)
            [default: git] [possible values: archive, git]

        --exclude-archived
            Exclude archived repositories

        --exclude-private
            Exclude private repositories

    -h, --help
            Print help information

        --token <TOKEN>
            Your Github personal access token

        --username <USERNAME>
            The Github username you used to create your access token (e.g. adriantombu)

    -V, --version
            Print version information
```