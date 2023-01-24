# Setup a Discord bot for Disma

## 1. Create and get a Discord bot token

If you don't have created a bot yet, here's how to do it :

1. Go to <https://discord.com/developers/applications>, login and create an application.
2. Go to the created application, click on `Bot` on the sidebar and create a bot.
3. Go to the created Bot page and copy the token (might need to create it first).
   - :warning: **Don't forget to save the token** (in the environment variable `DISCORD_BOT_TOKEN` for `disma-cli`).

> P.S.: The same bot can be used for all of your Discord servers :wink:

## 2. Add a bot to a Discord server (guild)

If your bot hasn't been already added to the server that you want to manage with Disma, here's how do add it :

1. On your bot's application page, go to `OAuth2` on the sidebar
2. In the `General` section, add a placeholder Redirect URL (can be `http://localhost`) and Save
3. In the `URL Generator` section, select the `identify` scope. A new section for the redirect URL will appear, make sur to select one.
4. Then also select the `bot` scope. A new pannel with Permissions will appear. Disma **will need** the `Administrator` permission, since the `Manage Roles` one is not enough. See the note below \*.
   - If you only want to test the bot for saving configs, you can leave all permissions unselected.
5. Navigate to the generated URL at the bottom of the page. This will bring you to an auth page, asking you to choose which server to add your bot to.
6. When confirming, a blank page will open. Just close it and your bot should have been added to your server!

> \* In Discord, a role with the Manage Roles permission also needs "higher" permissions that the role it is managing. To ensure that Disma can always manage every role, you will need to make it an Administrator.

To validate the bot's read access, run :

```shell
disma list
```

### Additional notes

- You will need to redo those steps for every server
- You can change the bot's permissions directly in the Server Settings without redoing all those steps :wink:
