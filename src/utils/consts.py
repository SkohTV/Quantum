import discord


class VERSION:
  major: int = 3
  minor: int = 0
  lesser: int = 1
  mode: str = 'dev'


class IDS:

  class GUILDS:
    guild_main = discord.Object(id=373056630004383744)


  class TEXT_CHANNELS:
    quantum_log = discord.Object(id=1046187380761174078)

    rules = discord.Object(id=422489846754443266)
    schedule = discord.Object(id=1094817197647478897)
    videos = discord.Object(id=420970924678840321)
    annonces = discord.Object(id=393840817124540447)
    welcome = discord.Object(id=1079430152691400724)

    discussions = discord.Object(id=919292528510001213)
    discussions_vip = discord.Object(id=919292838146093097)
    clips = discord.Object(id=1087880621696766112)
    commands_bot = discord.Object(id=975450909729505390)
    memes = discord.Object(id=1041170328572928100)

    coding_general = discord.Object(id=1125163001138397225)
    coding_help = discord.Object(id=1125163330546454680)
    coding_commands = discord.Object(id=1125163069631381645)

    cluster_general = discord.Object(id=1096583451588690115)
    cluster_ark = discord.Object(id=1096583621172797570)
    cluster_minecraft = discord.Object(id=1096583647626276904)

    contact_staff = discord.Object(id=975452567771414620)
    staff_private = discord.Object(id=1125234506954194975)


  class VOICE_CHANNELS:
    vocal_1 = discord.Object(id=977693061192753152)
    vocal_2 = discord.Object(id=977712600756404295)

    cluster_ark = discord.Object(id=1196919933192388638)

    waiting_move = discord.Object(id=1125234506954194975)
    private_admin = discord.Object(id=978051082355896330)


  class FORUM_CHANNELS:
    coding_ressources = discord.Object(id=1125163138891919551)
    coding_challenges = discord.Object(id=1172642300237717578 )

    cluster_annonces = discord.Object(id=1096583322949390477)
    cluster_infos = discord.Object(id=1197259594720878692)

  class ROLES:
    membre = discord.Object(id=919287553495019530)

