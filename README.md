
## Running the bot:
1. Create `.env` file with the following:
```
DISCORD_TOKEN="<replace with your own bot token>"
```
2. Run the bot:
`cargo run --release`


## TODO:
- [x] add error handling for "random" crashes in generate_ai_response() function
- [ ] send some fail message so that the user will know that it failed to generate response (currently if fails, no response)
- [ ] make it faster somehow
- [ ] make it remember the chat history (use api/chat instead of api/generate to generate responses)
- [ ] make it work if it gets **any** ping or user replies to the bot's message
