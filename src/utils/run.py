import discord
from dotenv import load_dotenv

import os
from discord.ext import commands

import asyncio

from src.utils.classes import Shared
from src.utils.consts import VERSION



def run():
  '''Create the client object and init a bunch of stuff'''
  global shared_variables

  # Create the bot object and load .env
  intents = discord.Intents.all() # Enable intents on https://discord.com/developers/applications
  bot = commands.Bot(command_prefix='q!', case_insensitive=True, intents=intents) # Create bot object (required)
  bot_version = f'{VERSION.major}.{VERSION.minor}.{VERSION.lesser}-{VERSION.mode}'

  # .env
  dotenv_path = os.path.join(os.getcwd(), '.env')
  load_dotenv(dotenv_path)
  
  # globals
  shared_variables = Shared()



  @bot.event
  async def on_ready():
    '''on_ready()'''

    # Print bot infos
    print(f'''
Logged with username {bot.user.name}
Logged with userID {bot.user.id}
Discord module version is {discord.__version__}

Servers connected to:
{'\n'.join([ '- ' + guild.name for guild in bot.guilds])}

Cogs loading...''')

    # Change status of the bot
    
    await bot.change_presence(activity=discord.Game(name=bot_version))



  async def load_modules(bot):
    await bot.load_extension('src.commands.ping') # Ping command
    await bot.load_extension('src.events.on_member_join') # Ping command

    await bot.load_extension('src.utils.sync') # Sync all commands to guilds


  async def start_bot(bot):
    await bot.start(os.environ.get('DISCORD_BOT_TOKEN'))


  asyncio.run(load_modules(bot)) # Load commands / events / tasks, then sync
  asyncio.run(start_bot(bot)) # Start the bot

