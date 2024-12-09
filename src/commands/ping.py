from discord.ext import commands
from discord import app_commands, Interaction
from src.utils.classes import Custom_Command
from src.utils.consts import IDS




class MyCommand(Custom_Command):
  name = 'ping'


  @app_commands.command(
    name=name,
    description='Renvoi le délai de Quantum entre les messages et réponses'
  )
  async def fn(self, interaction: Interaction):

    await interaction.response.send_message(content='⌛ Loading...')
    websockets_latency = self.bot.ws.latency

    await interaction.edit_original_response(content=f"Discord Websocket ⇒ `{websockets_latency:.3f}ms`")




async def setup(bot):
  '''Cog setup'''
  await bot.add_cog(MyCommand(bot), guilds=[IDS.GUILDS.guild_main])
