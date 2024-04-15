# dart-sweeper

A discord bot aimed at reducing spam on public servers.

## How it works

The bot will delete messages that contain invite links to other servers. You can use slash commands to configure which roles can or cannot post invites.

## Setup

Add the bot to your server. If you want to use the bot instance that I set up, then ask me for a bot invite link. Otherwise, you can host the bot on your own computer (instructions for that coming soon).

By default, the bot will prevent all users from posting invites, but that can be easily changed using the commands documented below.

There are three slash commands currently implemented:
1. `/whitelist <role>` will allow the specified role to post invites.
2. `/unwhitelist <role>` will remove the specified role from the whitelist and prohibit it from posting invites.
3. `/setmodchannel <channel>` will cause the bot to start logging its actions to the specified channel.

*Important note: ONLY USERS WITH PERMISSION TO DELETE OTHER PEOPLE'S MESSAGES ARE ABLE TO RUN THESE SLASH COMMANDS*

TODO: Add commands to view whitelist and unnasign the mod channel.

## Why did I make this?

I saw the spam on the r/Nerf discord server and wondered if there was a way to stop unverified members from posting invite links. Also, writing discord bots is easy and fun.