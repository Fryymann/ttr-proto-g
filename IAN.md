## 2026-02-21 - 8:00pm
The game should be campaign based and will require some UI on the server side. when a user starts the server they select which campaign to load from a list (prebuilt campaigns). Each campaign is a single-save game where progress can be saved and continued but there are no multiple saves.

Each player has an account on the server from which they can create and maintain their characters. A player makes a character and it is saved to their account. They can then join the active campaign, once a character has joined a campaign it is locked to that campaign.
Players can export copies of characters to a standardized JSON file. Players can join and quit at any time but we will need to devise rules around quiting in combat or accidentally being disconnected.
We retain some of the MUD style open world free play but the only one campaign is loaded per server. And once players enter a scene that has an active combat encounter they are pulled into it. We will need to devise rules around scene sizes or a range for encounter pulls.

I am thinking we should create a GUI to handle multiple connections per client so we can stream the "map" as well as combat feeds and comm channels separately. I want to keep the feel of a terminal game but not be too restricted.


## 2026-02-21 - 8:10pm
in a later version the game system should utilize Ollama and a lightweight ai model for 'offline' npc handling but for now I want to have Codex log into the game as a special 'player' who can handle the ai support of the game. the codex controlled 'DM' player should be able to control any NPC at any time (excluding those in an encounter (combat/social) and out of turn).  A queue system or message bus could help with this. DM player controlled by an AI agent gives a way to handle GM discretionary rulings and tie-breakers.


I feel like the SMAUG codebase would be a good resource but it is a little dated and possibly a lot more than we need for our game: https://github.com/smaugmuds/_smaug_

The game should be built on top of the D&D 5.5e mechanics as well as ruleset. By this i mean the classes, subclasses, species. D&D 5.5e serves as the base framework that can be extended with new classes, species, items, backgrounds, spells, feats, etc. 

Each campaign should act as an open world environment where players can be in different scenes and travel in real time. This skews the concept of time that is sometimes preserved in real table top games but I think it is an easy thing to give up to allow players freedom to explore the world of the campaign. scenes link to other scenes which can be something like "Lower Market Street" which had a door that leads to "The Winking Skeever Tavern". Players can move freely until pulled into an encounter.

### Encounters
Encounters should have 2 types: combat and social. the combat encounters will be easy to adapt to our game system but the social encounters will need to be designed from scratch. 

#### Social Encounters
These should be put off for v2 but I want to plan for them as we can. I want social encounters to incorporate the conversational skills, increase a targets focus on a character so another character could sneak and pick pocket. NPCs would need a cone of sight so we could use that plus their passive perception against a player's stealth and sleight of hand checks. DCs for checks would need to have a formula for calculation.

We should try utilizing the DM Agent to handle these situations and call for skills or ability checks from players based on the player's choices. We try to adapt the basic concepts of the combat system. We add charisma modifiers to initiative rolls and allow each user to make a single action. 

### Client UX/UI
I want to maintain the feel and functionality of a terminal, but I want a HUD for players that shows their view of the map from where they are and their character stats. we should look into Rust ruscii Canvas or something similar to using canvas in the terminal for the client. elastisearch, autocompletion, and other quality of life features to keep the terminal vibe fun and not painful.


### v2 Idea for Player Commands
maybe even later than v2, but I would like to look into the idea of using ai act more as a dm and allow players to describe what they want to do and have the ai dm interpret the description and turn it into narrative actions that then would take place.