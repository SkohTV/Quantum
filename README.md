# Quantum

---

### Commands

```
/ping ✔️
/embed [channel] [parameters] ✔️
/monitorwebsite [url] ❌

/kick [username] ❌
/ban [username] ❌
/mute [username] [time] ❌
/timeout [username] [time] ❌
/raidmode [on/off] ❌
```

### Events
```
On new member -> give member role ✔️
```

---

### Quantum-db
- Store cluster people data

### Quantum-cluster
Manage a cluster server database, to let people create profiles, add their infos
```
/clusteradd [username] ✔️
/clusterdel [username] ✔️
/clusterlist ✔️
/clusterinfo [username=self] ❌
/clusterupdate ❌
```

Also add people playing into summary message in \#infos ❌
```
On people join server -> add usertag to msg ❌
On people leave server -> remove usertag to msg ❌
On people join server -> add back cluster role if in db ❌
```

### Quantum-ytb
Auto create events on ytb livestream / premiere post ❌

Ytb chat command:
```
.clip [name=unnamed_year-mon-day-hh-mm-ss] [duration=60sec] ❌
```
