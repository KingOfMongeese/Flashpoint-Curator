Feature: App has a configuring ui at start

    Scenario: App starts in configuring
        Then the app is in the Configuring state

    Scenario: Start Game does nothing if there are no players
        When the start game button is clicked
        Then the app is in the Configuring state

    Scenario: Adding a Player
        Given the field to add a player contains KingOfMongeese
        When the add player button is clicked
        Then the app contains a player called KingOfMongeese
          And the player KingOfMongeese has zeroed stats
    
    Scenario: Removing a player
        Given the app contains a player called KingOfMongeese
        When the remove player button for KingOfMongeese is clicked
        Then the app does not contain a player called KingOfMongeese
    
    Scenario: Adding more than 1 player
        Given the field to add a player contains KingOfMongeese
        When the add player button is clicked
        Given the field to add a player contains Fred
        When the add player button is clicked
        Given the field to add a player contains George
        When the add player button is clicked
        Then the app contains a player called KingOfMongeese
          And the player KingOfMongeese has zeroed stats
          And the app contains a player called Fred
          And the player Fred has zeroed stats
          And the app contains a player called George
          And the player George has zeroed stats
          And the app does not contain a player called Jeremy

    Scenario: Done Configuring
        Given the field to add a player contains KingOfMongeese
        When the add player button is clicked
        Given the field to add a player contains Fred
        When the add player button is clicked
          And the start game button is clicked
        Then the app is in the Massive-Slayer-Multiplayer state
          And the app contains a player called KingOfMongeese
          And the player KingOfMongeese has a turn order number but no kills
          And the app contains a player called Fred
          And the player Fred has a turn order number but no kills