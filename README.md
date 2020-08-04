# About
Status Monitor is a daemon which will listen for check-in messages from other
services and, if some service fails to check in within a specified time period,
notify you via a message from your Telegram bot.

# Configuring the bot
Here is how you create a new Telegram bot, and configure Status Monitor to
interface with it:
1. Open your Telegram client and type `BotFather` into the search bar.
2. Click start, and follow the instructions to create a new bot. You will need
   the bot's username and token in a second.
3. Type your bot's username into the Telegram search bar.
4. Click start, and follow up with another arbitrary message. The point is to
   create a conversation that will later be used to deliver status reports.
5. Open a browser or some RESTful client and type
   ```
   https://api.telegram.org/bot<token>/getUpdates
   ```
6. Take note of your chat id (`/result/0/message/chat/id` in the JSON)
7. Open the status-monitor directory
8. Create a copy of `status-monitor-config.yml.template` called
   `status-monitor-config.yml`
9. Edit `status-monitor-config.yml`, inserting your bot's token and your chat id
