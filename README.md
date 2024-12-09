## Build
Create a python virtual env and install packages
```sh
# Create a virtual env
python -m venv _venv

# Install packages
_venv/bin/python -m pip install -r requirements.txt
```

Then you will need a `.env` file that contains the following :
```sh
DISCORD_BOT_TOKEN='your_personnal_testing_bot_token'
```

Afterward you can run the bot locally on your server with 
```sh
.venv/bin/python main.py
```


## Todo
- Rework


## Bot

### Commands

```
/ping

/raidmode [on/off]

```

### Events

```
On new member -> give member role if not raid mode
```
