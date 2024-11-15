A (dot)file manager (intended for use in rebos)

Intended File-structure:
```
.config/rebos
├── files
    ├── ...
│   └── common
├── gen.toml
├── imports
│   └── ...
├── machines
│   └── ...
├── manager_order.toml
└── managers
    ├── ...
    └── files.toml
```

As you sometimes want files to differ on different machines, the 'files' directory is split in multiple sub-directories.

Intended usage: `{some sub-directory of files}/{the file path you want to symlink}` (For example: `common/etc/pacman.conf`)

Because most of the time symlinks are against the common directory, you can just omit the "common". (For example: `/etc/pacman.conf`) (The default sub-dir can be configured with `--default-sub-dir`)

An example managers/files.toml is included in this repo.
