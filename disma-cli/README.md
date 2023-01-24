<div align="center">

# Disma - CLI

[![disma-cli](https://img.shields.io/crates/v/disma-cli)](https://crates.io/crates/disma-cli)

**Command line interface for [disma](https://github.com/vigenere23/disma).**

</div>

## ⬇️ Installation

You can install the `disma` binary multiple ways.

### With `cargo`

```shell
cargo install disma-cli
```

### With `curl`

**UNIX (Linux, MacOS)**

```bash
curl -sL "https://github.com/vigenere23/disma/releases/download/disma-cli%2Fv<DISMA_CLI_VERSION>/disma-<ARCH>.tar.gz" | tar -xz
```

where `<ARCH>` is one of:

- `aarch64-apple-darwin`
- `x86_64-apple-darwin`
- `x86_64-unknown-linux-gnu`

## 🚀 Commands

⚠️ All commands needs the evironment variable `DISCORD_BOT_TOKEN` to be set.

### `list`

List bot's accessible servers. If you don't see access to your server, make sure to [add your bot to it](https://github.com/vigenere23/disma/blob/master/docs/bot.md).

### `compile`

Compile a template config to a full config file. Only the handlebars format is supported for now. Will compile to the original format (JSON or YAML).

**Arguments**

- `--template, -t <TEMPLATE_FILE>` : File to use as a template (contains handlebars tokens).
- `--vars, -v <VARS_FILE>` : File containing variables that populates the template. can be either YAML or JSON.
- `--output, -o <OUTPUT_FILE>` : Compiled config output file.
- `--force, -f` : Bypass the user confirmation step.

### `save`

Save a server (guild) configuration.

**Arguments**:

- `--guild, -g <GUILD_ID>` : Id of the guild to save. To find your guild id, use [`list`](#list).
- `--output, -o <OUTPUT_FILE>` : Output file path. Both `.json` and `.yaml`/`.yml` files are supported.
- `--force, -f` : Bypass the user confirmation step.

### `apply`

Apply changes to a server based on a configuration file.

**Arguments**

- `--guild, -g <GUILD_ID>` : Id of the guild to save. To find your guild id, use [`list`](#list).
- `--input, -i <INPUT_FILE>` : Configuration file to use. Both `.json` and `.yaml`/`.yml` files are supported. Make sure to follow the [configuration file format](https://github.com/vigenere23/disma/blob/master/docs/config.md).
- `--force, -f` : Bypass the user confirmation step.
