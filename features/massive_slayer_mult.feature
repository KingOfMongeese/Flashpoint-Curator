Feature: App has a main playing mode for massive slayer multiplayer

    Scenario: A player gains a kill
        Given the app contains a player called KingOfMongeese
        When the add kill button for KingOfMongeese is clicked
        Then KingOfMongeese's kills is 1
    
    Scenario: A Player has a kill removed
        Given the app contains a player called KingOfMongeese
        Given KingOfMongeese's kills is 3
        When the remove kill button for KingOfMongeese is clicked
        Then KingOfMongeese's kills is 2
    
    Scenario: Next turn order is set by players passing by default
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for Noa is clicked
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        Then Noa's next turn number is 1
          And KingOfMongeese's next turn number is 2
          And Larry's next turn number is 3
        
        When the next turn button is clicked
        Then Noa's current turn number is 1
        And KingOfMongeese's current turn number is 2
          And Larry's current turn number is 3

        When the pass button for KingOfMongeese is clicked
        When the pass button for Noa is clicked
        When the pass button for Larry is clicked
        Then Noa's next turn number is 2
          And KingOfMongeese's next turn number is 1
          And Larry's next turn number is 3
        
        When the next turn button is clicked
        Then Noa's current turn number is 2
        And KingOfMongeese's current turn number is 1
          And Larry's current turn number is 3
    
    Scenario: Resesting the game sets the status back to default and clears all players
        When the new game button is clicked
        Then the app is in the Configuring state
          And the app's data is set back to default