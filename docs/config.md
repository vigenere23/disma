# ⚙️ Guild configuration

The configuration file can be either a JSON file (`.json`) or a YAML file (`.yaml` or `.yml`). YAML file can include anchors and merges. It is used to describe the wanted state or a Discord server (guild). A [JSON Schema](https://github.com/vigenere23/disma/blob/master/schema.json) is also available.

**Table of content**

- [`roles`](#roles)
  - [`roles.items[*]`](#rolesitems)
  - [`roles.extra_items`](#rolesextra_items)
- [`categories`](#categories)
  - [`categories.items[*]`](#categoriesitems)
  - [`categories.extra_items`](#categoriesextra_items)
- [`channels`](#channels)
  - [`channels.items[*]`](#channelsitems)
  - [`channels.items[*].permissions_overwrites`](#channelsitemspermissions_overwrites)
  - [`channels.extra_items`](#channelsextra_items)
- [Types](#types)
  - [`PermissionsOverwrite`](#permissionsoverwrite)
  - [`Permission`](#permission)

## `roles`

**Fields**

- `items`: List of roles.
- `extra_items`: Extra items params.

### `roles.items[*]`

**Fields**

- `name` (`string`) : Name of the role.
  - ⚠️ Every role needs to have a **unique _name_**.
- `permissions` (`Permission[]`) : List of [permissions](#permission).
- `show_in_sidebar` (`bool`) : Show connection status of members with this role in the Members sidebar. The members will be categorized by role.
- `is_mentionable` (`bool`) : Allow everyone to mention this role with `@` (ex: `@team-01`).
- `color` (optional `string`) : Color of the role, in hexadecimal format (without the `#`).

**Important notes**

- You cannot directly rename a role. The role will be **deleted** and recreated under a different name.
  - To rename a role, please rename it in the Discord interface first, then in the config.
- Every members associated to a role that's been deleted will **lose that role**.

### `roles.extra_items`

**Fields**

- `strategy` (`string`): Strategy for handling extra roles. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.

## `categories`

**Fields**

- `items`: List of categories.
- `extra_items`: Extra items params.

### `categories.items[*]`

**Fields**

- `name` (`string`) : Name of the category.
  - ⚠️ Every category needs to have a **unique _name_**.
- `permissions_overwrites` (`PermissionsOverwrite[]`) : List of [permissions overwrites](#permissionsoverwrite).
- `extra_channels`:
  - `strategy` (`string`): Strategy for handling extra channels under this category. Options: `REMOVE`, `KEEP`, `SYNC_PERMISSIONS`. Default: `REMOVE`. `SYNC_PERMISSIONS` updates permissions to match categorie's.

### `categories.extra_items`

**Fields**

- `strategy` (`string`): Strategy for handling extra categories. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.

## `channels`

**Fields**

- `items`: List of channels.
- `extra_items`: Extra items params.

### `channels.items[*]`

**Fields**

- `name` (`string`) : Name of the channel.
- `type` (optional `string`) : Type of channel. Currently supported are `TEXT` and `VOICE`. Default: `TEXT`.
- `category` (optional `string`) : Name of the channel's parent category.
  - ⚠️ Every channel needs to have a **unique combination of _name_, _category_ and _type_**.
- `topic` (optional `string`) : Topic of the channel.
- `permissions_overwrites`: Params for setting the permissions overwrites.

**Important notes**

- You cannot directly rename a channel. The channel will be **deleted** and recreated under a different name.
  - To rename a channel, please rename it in the Discord interface first, then in the config.
- Deleted channels will **lose all their messages**.
- You currently cannot allow channels that are not listed in the config. This should be soon permitted, at least for channels associated to categories.

### `channels.items[*].permissions_overwrites`

**Fields**

- `strategy`: Strategy for setting the permissions overwrites.

#### `MANUAL` (default)

Manualy define the list of permissions overwrites.

**Fields**

- `items` (`PermissionsOverwrite[]`): List of [permissions overwrites](#permissionsoverwrite).
  - default: `[]`

#### `FROM_CATEGORY`

Use permissions overwrites from category.

**Important notes**

- This option will **panic** if the channel is not associated to a category.

### `channels.extra_items`

**Fields**

- `strategy` (`string`): Strategy for handling extra channels. Options: `REMOVE`, `KEEP`. Default: `REMOVE`.

## Types

### `PermissionsOverwrite`

Overwrites of permissions to apply to a specific context only. You can read more on the [Discord Developer Portal](https://discord.com/developers/docs/topics/permissions#permission-overwrites).

**Fields**

- `role` (`string`) : Name of the role to apply overwrites to.
- `allow` (`Permission[]`) : Specifically allowed these [permissions](#permission). for the role.
- `deny` (`Permission[]`) : Specifically denied these [permissions](#permission). for the role.

### `Permission`

Uppercase `string` that represents Discord permissions. You can read more on the [Discord Developer Portal](https://discord.com/developers/docs/topics/permissions#permissions-bitwise-permission-flags).
