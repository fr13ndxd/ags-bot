
## Running the bot:
1. Create `.env` file with the following:
```
DISCORD_TOKEN="<replace with your own bot token>"
```
2. Run the bot:
`cargo run --release`


## TODO:
- add error handling for "random" crashes in generate_ai_response() function
- send some fail message so that the user will know that it failed to generate response (currently if fails, no response)
- make it faster somehow
