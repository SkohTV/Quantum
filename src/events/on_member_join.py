from discord import Member
from discord.ext import commands
from src.utils.classes import Custom_Event
from src.utils.consts import IDS




class MyEvent(Custom_Event):
  name = 'on_member_join'


  @commands.Cog.listener(
    name=name
  )
  async def fn(self, member: Member):
    await member.add_roles(IDS.ROLES.membre)



async def setup(bot):
  '''Cog setup'''
  await bot.add_cog(MyEvent(bot), guilds=[IDS.GUILDS.guild_main])
