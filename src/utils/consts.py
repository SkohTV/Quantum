import discord

O = lambda x: discord.Object(id=x)


class VERSION:
  major: int = 3
  minor: int = 0
  lesser: int = 1
  mode: str = 'dev'


class IDS:

  class GUILDS:
    guild_main = O(373056630004383744)


  class TEXT_CHANNELS:
    quantum_log = O(1046187380761174078)

    rules = O(422489846754443266)
    schedule = O(1094817197647478897)
    videos = O(420970924678840321)
    annonces = O(393840817124540447)
    welcome = O(1079430152691400724)

    discussions = O(919292528510001213)
    discussions_vip = O(919292838146093097)
    clips = O(1087880621696766112)
    commands_bot = O(975450909729505390)
    memes = O(1041170328572928100)

    coding_general = O(1125163001138397225)
    coding_help = O(1125163330546454680)
    coding_commands = O(1125163069631381645)

    cluster_general = O(1096583451588690115)
    cluster_ark = O(1096583621172797570)
    cluster_minecraft = O(1096583647626276904)

    contact_staff = O(975452567771414620)
    staff_private = O(1125234506954194975)


  class VOICE_CHANNELS:
    vocal_1 = O(977693061192753152)
    vocal_2 = O(977712600756404295)

    cluster_ark = O(1196919933192388638)

    waiting_move = O(1125234506954194975)
    private_admin = O(978051082355896330)


  class FORUM_CHANNELS:
    coding_ressources = O(1125163138891919551)
    coding_challenges = O(1172642300237717578 )

    cluster_annonces = O(1096583322949390477)
    cluster_infos = O(1197259594720878692)

  class ROLES:
    membre = O(919287553495019530)

