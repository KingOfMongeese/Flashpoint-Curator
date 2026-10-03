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
    
    Scenario: A next turn order is set by players passing