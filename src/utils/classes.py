from discord.ext import commands


# In order to create a command, event or task, you inherit from the following classes
# They are small boilerplate snippets that set the `bot` variable and print add a log
# When they are loaded

class Generalized_Cog(commands.Cog):
  '''Core of custom cogs, generalized at most'''
  name = 'unnamed'
  method_type = 'untyped'

  '''Cog setup'''
  def __init__(self, bot: commands.Bot):
    self.bot = bot

  @commands.Cog.listener()
  async def on_ready(self):
    '''Send message when cog is successfully loaded'''
    print(f"{self.method_type} loaded -> {self.name}")



class Custom_Command(Generalized_Cog):
  method_type = 'Command'

class Custom_Event(Generalized_Cog):
  method_type = 'Event'

class Custom_Task(Generalized_Cog):
  method_type = 'Task'


# ----------------------- #

class Shared:
  raid_mode: bool = False
