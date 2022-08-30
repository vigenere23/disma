[![CI](https://github.com/vigenere23/dac/actions/workflows/ci.yml/badge.svg)](https://github.com/vigenere23/dac/actions/workflows/ci.yml)

# Discord as Config (dac)

👨🏼‍🔧 Manage Discord server settings with a repeatable, versionable config file.

- [Prerequesites](#prerequesites)
  - [Install the executable](#install-the-executable)
  - [Create and get a Discord bot token](#create-and-get-a-discord-bot-token)
  - [Add a bot to a Discord server (guild)](#add-a-bot-to-a-discord-server-guild)
- [Commands](#commands)
  - [`list`](#list)
  - [`compile`](#compile)
  - [`save`](#save)
  - [`apply`](#apply)
- [Server configuration file](#server-configuration-file)
  - [`roles`](#roles)

## Prerequesites

### Install the executable

#### Method 1 : with `cargo`

```shell
cargo install dac
```

To validate installation, run :

```shell
dac
```

#### Method 2 : standalone

Not yet available.

### Create and get a Discord bot token

If you don't have created a bot yet, here's how to do it :

1. Go to <https://discord.com/developers/applications>, login and create an application.
2. Go to the created application, click on `Bot` on the sidebar and create a bot.
3. Go to the created Bot page and copy the token (might need to create it first).
   - :warning: **Don't forget to save the token** (in the environment variable `DAC_DISCORD_BOT_TOKEN` for dac).

> P.S.: The same bot can be used for all of your Discord servers :wink:

### Add a bot to a Discord server (guild)

If your bot hasn't been already added to the server that you want to manage with dac, here's how do add it :

1. On your bot's application page, go to `OAuth2` on the sidebar
2. In the `General` section, add a placeholder Redirect URL (can be `http://localhost`) and Save
3. In the `URL Generator` section, select the `identify` scope. A new section for the redirect URL will appear, make sur to select one.
4. Then also select the `bot` scope. A new pannel with Permissions will appear. dac only needs the `Manage Roles` and `Manage Channels` permissions, so you can select those.
   - If you only want to test the bot for saving configs, you can leave all permissions unselected.
5. Navigate to the generated URL at the bottom of the page. This will bring you to an auth page, asking you to choose which server to add your bot to.
6. Confirm and your bot should have been added to your server!

> P.S.: You will need to redo those steps for every server

> P.P.S.: You can change the bot's permissions directly in the Server Settings without redoing all those steps :wink:

To validate the bot's read access, run :

```shell
dac list
```

## Commands

:warning: All commands needs the evironment variable `DAC_DISCORD_BOT_TOKEN` to be set.

### `list`

List bot's accessible servers. If you don't see access to your server, make sure to [add your bot to it](#add-a-bot-to-a-discord-server-guild).

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
- `--input, -i <INPUT_FILE>` : Configuration file to use. Both `.json` and `.yaml`/`.yml` files are supported. Make sure to correctly follow the [configuration file schemas](#server-configuration-file).
- `--force, -f` : Bypass the user confirmation step.



## Server configuration file

The configuration file can be either a JSON file (`.json`) or a YAML file (`.yaml` or `.yml`). YAML file can include anchors and merges. It is used to describe the wanted state or a Discord server (guild).

<details>
<summary>Configuration file example (click to reveal)</summary>

```yaml
roles:
  - name: '@everyone'
    permissions:
      - CHANGE_NICKNAME
      - VIEW_CHANNEL
      - CONNECT
      - SPEAK
      - USE_VAD
      - STREAM
      - REQUEST_TO_SPEAK
      - SEND_MESSAGES
      - READ_MESSAGE_HISTORY
      - CREATE_PUBLIC_THREADS
      - SEND_MESSAGES_IN_THREADS
      - ADD_REACTIONS
      - ATTACH_FILES
    show_in_sidebar: true
    is_mentionable: true

  - name: admin
    permissions:
      - ADMINISTRATOR
    color: 36AF6D
    show_in_sidebar: false
    is_mentionable: false

  - name: dac
    permissions:
      - MANAGE_ROLES
    color: 98ADF3
    show_in_sidebar: false
    is_mentionable: false

  - name: team-01
    template: team
  - name: team-02
    template: team
  - name: team-03
    template: team

templates:
  roles:
    - name: team
      permissions:
        - CHANGE_NICKNAME
        - VIEW_CHANNEL
        - CONNECT
        - SPEAK
        - USE_VAD
        - STREAM
        - REQUEST_TO_SPEAK
        - SEND_MESSAGES
        - READ_MESSAGE_HISTORY
        - CREATE_PUBLIC_THREADS
        - SEND_MESSAGES_IN_THREADS
        - ADD_REACTIONS
        - ATTACH_FILES
      show_in_sidebar: true
      is_mentionable: true
```

</details>

### `roles`

**Fields**

- `name` (`string`) : Name of the role. :warning: **Every role needs to have a unique name**.
- `permissions` (`string[]`) : List of permissions by name. You can read more about Discord's permissions on the [Discord Developer Portal](https://discord.com/developers/docs/topics/permissions).
- `show_in_sidebar` (`bool`) : Show connection status of members with this role in the Members sidebar. The members will be categorized by role.
- `is_mentionable` (`bool`) : Allow everyone to mention this role with `@` (ex: `@team-01`).
- `color` (optional `string`) : Color of the role, in hexadecimal format (without the `#`).
