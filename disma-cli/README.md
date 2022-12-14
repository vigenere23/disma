# Disma - Command line tool

[![disma-cli](https://img.shields.io/crates/v/disma-cli)](https://crates.io/crates/disma-cli)

- [☑️ Prerequisites](#️-prerequisites)
- [🚀 Commands](#-commands)
- [⚙️ Guild configuration file](#️-guild-configuration-file)

## ☑️ Prerequisites

### 1. Install the CLI

#### Method 1 : with `cargo`

```shell
cargo install disma-cli
```

To validate installation, run :

```shell
disma
```

#### Method 2 : standalone

Not yet available.

### 2. Create and get a Discord bot token

If you don't have created a bot yet, here's how to do it :

1. Go to <https://discord.com/developers/applications>, login and create an application.
2. Go to the created application, click on `Bot` on the sidebar and create a bot.
3. Go to the created Bot page and copy the token (might need to create it first).
   - :warning: **Don't forget to save the token** (in the environment variable `DISCORD_BOT_TOKEN` for Disma).

> P.S.: The same bot can be used for all of your Discord servers :wink:

### 3. Add a bot to a Discord server (guild)

If your bot hasn't been already added to the server that you want to manage with Disma, here's how do add it :

1. On your bot's application page, go to `OAuth2` on the sidebar
2. In the `General` section, add a placeholder Redirect URL (can be `http://localhost`) and Save
3. In the `URL Generator` section, select the `identify` scope. A new section for the redirect URL will appear, make sur to select one.
4. Then also select the `bot` scope. A new pannel with Permissions will appear. Disma **will need** the `Administrator` permission, since the `Manage Roles` one is not enough. See the note below \*.
   - If you only want to test the bot for saving configs, you can leave all permissions unselected.
5. Navigate to the generated URL at the bottom of the page. This will bring you to an auth page, asking you to choose which server to add your bot to.
6. When confirming, a blank page will open. Just close it and your bot should have been added to your server!

> \* In Discord, a role with the Manage Roles permission also needs "higher" permissions that the role it is managing. To ensure that Disma can always manage every role, you will need to make it an Administrator.

> P.S.: You will need to redo those steps for every server

> P.P.S.: You can change the bot's permissions directly in the Server Settings without redoing all those steps :wink:

To validate the bot's read access, run :

```shell
disma list
```


## 🚀 Commands

:warning: All commands needs the evironment variable `DISCORD_BOT_TOKEN` to be set.

### 🢒 `list`

List bot's accessible servers. If you don't see access to your server, make sure to [add your bot to it](#add-a-bot-to-a-discord-server-guild).

### 🢒 `compile`

Compile a template config to a full config file. Only the handlebars format is supported for now. Will compile to the original format (JSON or YAML).

**Arguments**

- `--template, -t <TEMPLATE_FILE>` : File to use as a template (contains handlebars tokens).
- `--vars, -v <VARS_FILE>` : File containing variables that populates the template. can be either YAML or JSON.
- `--output, -o <OUTPUT_FILE>` : Compiled config output file.
- `--force, -f` : Bypass the user confirmation step.

### 🢒 `save`

Save a server (guild) configuration.

**Arguments**:

- `--guild, -g <GUILD_ID>` : Id of the guild to save. To find your guild id, use [`list`](#list).
- `--output, -o <OUTPUT_FILE>` : Output file path. Both `.json` and `.yaml`/`.yml` files are supported.
- `--force, -f` : Bypass the user confirmation step.

### 🢒 `apply`

Apply changes to a server based on a configuration file.

**Arguments**

- `--guild, -g <GUILD_ID>` : Id of the guild to save. To find your guild id, use [`list`](#list).
- `--input, -i <INPUT_FILE>` : Configuration file to use. Both `.json` and `.yaml`/`.yml` files are supported. Make sure to correctly follow the [configuration file schemas](#server-configuration-file).
- `--force, -f` : Bypass the user confirmation step.


## ⚙️ Guild configuration file

The configuration file can be either a JSON file (`.json`) or a YAML file (`.yaml` or `.yml`). YAML file can include anchors and merges. It is used to describe the wanted state or a Discord server (guild).

Some examples can be found [here](./docs/examples).


### 🢒 `roles`

**Fields**

- `items`: List of roles.
- `extra_items`: Extra items config.

#### `roles.items[*]`

**Fields**

- `name` (`string`) : Name of the role.
  - ⚠️ Every role needs to have a **unique *name***.
- `permissions` (`string[]`) : List of permissions by name. You can read more about Discord's permissions on the [Discord Developer Portal](https://discord.com/developers/docs/topics/permissions).
- `show_in_sidebar` (`bool`) : Show connection status of members with this role in the Members sidebar. The members will be categorized by role.
- `is_mentionable` (`bool`) : Allow everyone to mention this role with `@` (ex: `@team-01`).
- `color` (optional `string`) : Color of the role, in hexadecimal format (without the `#`).

**Important notes**

- You cannot directly rename a role. The role will be **deleted** and recreated under a different name.
  - To rename a role, please rename it in the Discord interface first, then in the config.
- Every members associated to a role that's been deleted will **lose that role**.

#### `roles.extra_items`

**Fields**

- `strategy` (`string`): Strategy for handling extra roles. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.


### 🢒 `categories`

**Fields**

- `items`: List of categories.
- `extra_items`: Extra items config.

#### `categories.items[*]`

**Fields**

- `name` (`string`) : Name of the category.
  - ⚠️ Every category needs to have a **unique *name***.
- `permissions_overwrites` (`PermissionsOverwrite[]`) : List of permissions overwrites.
- `extra_channels`:
  - `strategy` (`string`): Strategy for handling extra channels under this category. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.

#### `categories.extra_items`

**Fields**

- `strategy` (`string`): Strategy for handling extra categories. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.


### 🢒 `channels`

**Fields**

- `items`: List of channels.
- `extra_items`: Extra items config.

#### `channels.items[*]`

**Fields**

- `name` (`string`) : Name of the channel.
- `type` (optional `string`) : Type of channel. Currently supported are `TEXT` and `VOICE`. Default: `TEXT`.
- `category` (optional `string`) : Name of the channel's parent category.
  - ⚠️ Every channel needs to have a **unique combination of *name*, *category* and *type***.
- `topic` (optional `string`) : Topic of the channel.
- `permissions_overwrites` (`PermissionsOverwrite[]`) : List of permissions overwrites.

**Important notes**

- You cannot directly rename a channel. The channel will be **deleted** and recreated under a different name.
  - To rename a channel, please rename it in the Discord interface first, then in the config.
- Deleted channels will **lose all their messages**.
- You currently cannot allow channels that are not listed in the config. This should be soon permitted, at least for channels associated to categories.

#### `channels.extra_items`

**Fields**

- `strategy` (`string`): Strategy for handling extra channels. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.


### Types

#### 🢒 `PermissionsOverwrite`

Overwrites of permissions to apply to a specific context only. You can read more on the [Discord Developer Portal](https://discord.com/developers/docs/topics/permissions#permission-overwrites).

**Fields**

- `role` (`string`) : Name of the role to apply overwrites to.
- `allow` (`Permission[]`) : Specifically allowed permissions overwrites for the role.
- `deny` (`Permission[]`) : Specifically denied permissions overwrites for the role.

#### 🢒 `Permission`

Uppercase `string` that represents Discord permissions. You can read more on the [Discord Developer Portal](https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags).
