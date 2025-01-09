from src.discord import DiscordBot
from discord import c



def main() -> None:
  '''Entrypoint'''
  discord_bot = DiscordBot()
  discord_bot.run()


if __name__ == '__main__':
  main()

