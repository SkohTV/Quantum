from typing import Callable
from discord import Interaction
from src.utils.consts import IDS







# def is_allowed(name: str, msg: Interaction) -> tuple[bool, str]:
#   match name.lower():
#     case 'ping':
#       if not in_valid_channel([IDS.TEXT_CHANNELS.coding_commands], msg):
#         return (False, 'Cannot post message in this channel')
#       
#
#
#   # Return true if verification is successfull (and empty error message)
#   return (True, '')


def is_bot_channel(self, fn: Callable):
  ...




def in_valid_channel(allowed_ids: list[int], msg: Interaction) -> bool:
  return msg.channel_id in allowed_ids

