Feature: App has a configuring ui at start


    Scenario: Adding a Player
        Given the field to add a player contains KingOfMongeese
        When the add player button is clicked
        Then the app contains a player called KingOfMongeese
          And the player KingOfMongeese has zeroed stats