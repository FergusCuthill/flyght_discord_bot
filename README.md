# Flyght Discord Bot

Bot to manage Flyght discord and related automations



# Connecting to discord
To run this bot, a valid Discord Token is needed. To get started log in to the [Discord developer portal](https://discord.com/developers/applications).

1. Click the New Application button, name your application and click Create.
2. Navigate to the Bot tab in the lefthand menu, and add a new bot.
3. On the bot page click the Reset Token button to reveal your token. Put this token in your `Secrets.toml`. Note that other filenames can be used. The following has to be added to the run/deploy command in that case: --secrets <secrets_file> (replace <secrets_file> with the file name). Make sure the file is covered by the gitignore. 
4. Scroll down in the discord developer portal and add relevant permissions.

To add the bot to a server we need to create an invite link.

1. On your bot's application page, open the OAuth2 page via the lefthand panel.
2. Go to the URL Generator via the lefthand panel, and select the `bot` scope as well as the `Send Messages` permission in the Bot Permissions section.
3. Copy the URL, open it in your browser and select a Discord server you wish to invite the bot to.

For more information please refer to the [Discord docs](https://discord.com/developers/docs/getting-started) as well as the [Serenity repo](https://github.com/serenity-rs/serenity) for more examples.
