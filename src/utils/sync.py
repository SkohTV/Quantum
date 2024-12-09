from discord import Object
from discord.ext import commands
from src.utils.consts import IDS



class Sync(commands.Cog):
  '''Cog setup'''
  def __init__(self, bot: commands.Bot):
    self.bot = bot

  @commands.Cog.listener()
  async def on_ready(self):
    '''Send message AND DO ACTION when cog is successfully loaded'''
    # Syncing of commands
    fmt = await self.bot.tree.sync(guild=IDS.GUILDS.guild_main) # Sync all commands to tree

    # Print to console
    print(f"Method loaded -> {type(self).__name__.lower()}")
    print(f"Synced {len(fmt)} commands to the current guild")
    print(f"\n{'='*60}\nMonitoring...\n")



# Smol cog class, as the module is imported as a cog for easier async call
async def setup(bot):
  '''Cog setup'''
  await bot.add_cog(Sync(bot), guilds=[IDS.GUILDS.guild_main])
