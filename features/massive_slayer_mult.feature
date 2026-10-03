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
    
    Scenario: Next turn order can be interrupted by clicking the use spartan
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for Noa is clicked
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        Then Noa's next turn number is 1
          And KingOfMongeese's next turn number is 2
          And Larry's next turn number is 3
        
        When the spartan button for Larry is clicked

        When the next turn button is clicked
        Then Larry's current turn number is 1
          And Noa's current turn number is 2
          And KingOfMongeese's current turn number is 3
    
    Scenario: The last player to use a spartan order is first
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for Noa is clicked
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        Then Noa's next turn number is 1
          And KingOfMongeese's next turn number is 2
          And Larry's next turn number is 3
        
        When the spartan button for Larry is clicked        
        When the spartan button for KingOfMongeese is clicked

        When the next turn button is clicked
        Then KingOfMongeese's current turn number is 1
          And Larry's current turn number is 2
          And Noa's current turn number is 3

    Scenario: The last player to use a spartan order is first, when a player uses a spartan order twice
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for Noa is clicked
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        Then Noa's next turn number is 1
          And KingOfMongeese's next turn number is 2
          And Larry's next turn number is 3
        
        When the spartan button for Larry is clicked
        When the spartan button for KingOfMongeese is clicked
        When the spartan button for Larry is clicked

        When the next turn button is clicked
        Then Larry's current turn number is 1
          And KingOfMongeese's current turn number is 2
          And Noa's current turn number is 3
    
        Scenario: A middle player uses a spartan
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked

        When the spartan button for Larry is clicked

        When the next turn button is clicked
        Then Larry's current turn number is 1
          And KingOfMongeese's current turn number is 2
          And Noa's current turn number is 3

    Scenario: The last player uses a spartan
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked

        When the spartan button for Noa is clicked

        When the next turn button is clicked
        Then Noa's current turn number is 1
          And KingOfMongeese's current turn number is 2
          And Larry's current turn number is 3

    Scenario: The first player uses a spartan after the middle player
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked

        When the spartan button for Larry is clicked
        When the spartan button for KingOfMongeese is clicked

        When the next turn button is clicked
        Then KingOfMongeese's current turn number is 1
          And Larry's current turn number is 2
          And Noa's current turn number is 3

    Scenario: Two middle players use a spartan with the last press first
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        Given the app contains a player called Ned
        Given the app contains a player called Alice
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked
        When the pass button for Ned is clicked
        When the pass button for Alice is clicked

        When the spartan button for Noa is clicked
        When the spartan button for Ned is clicked

        When the next turn button is clicked
        Then Ned's current turn number is 1
          And Noa's current turn number is 2
          And KingOfMongeese's current turn number is 3
          And Larry's current turn number is 4
          And Alice's current turn number is 5

    Scenario: A middle player uses a spartan twice with more than six players
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        Given the app contains a player called Ned
        Given the app contains a player called Alice
        Given the app contains a player called Bob
        Given the app contains a player called Charlie
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked
        When the pass button for Ned is clicked
        When the pass button for Alice is clicked
        When the pass button for Bob is clicked
        When the pass button for Charlie is clicked

        When the spartan button for Ned is clicked
        When the spartan button for Ned is clicked

        When the next turn button is clicked
        Then Ned's current turn number is 1
          And KingOfMongeese's current turn number is 2
          And Larry's current turn number is 3
          And Noa's current turn number is 4
          And Alice's current turn number is 5
          And Bob's current turn number is 6
          And Charlie's current turn number is 7

    Scenario: The last player uses a spartan with more than six players
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        Given the app contains a player called Ned
        Given the app contains a player called Alice
        Given the app contains a player called Bob
        Given the app contains a player called Charlie
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked
        When the pass button for Ned is clicked
        When the pass button for Alice is clicked
        When the pass button for Bob is clicked
        When the pass button for Charlie is clicked

        When the spartan button for Charlie is clicked

        When the next turn button is clicked
        Then Charlie's current turn number is 1
          And KingOfMongeese's current turn number is 2
          And Larry's current turn number is 3
          And Noa's current turn number is 4
          And Ned's current turn number is 5
          And Alice's current turn number is 6
          And Bob's current turn number is 7

    Scenario: Multiple players use a spartan with more than six players
        Given the app contains a player called KingOfMongeese
        Given the app contains a player called Larry
        Given the app contains a player called Noa
        Given the app contains a player called Ned
        Given the app contains a player called Alice
        Given the app contains a player called Bob
        Given the app contains a player called Charlie
        When the pass button for KingOfMongeese is clicked
        When the pass button for Larry is clicked
        When the pass button for Noa is clicked
        When the pass button for Ned is clicked
        When the pass button for Alice is clicked
        When the pass button for Bob is clicked
        When the pass button for Charlie is clicked

        When the spartan button for Larry is clicked
        When the spartan button for Bob is clicked
        When the spartan button for Alice is clicked

        When the next turn button is clicked
        Then Alice's current turn number is 1
          And Bob's current turn number is 2
          And Larry's current turn number is 3
          And KingOfMongeese's current turn number is 4
          And Noa's current turn number is 5
          And Ned's current turn number is 6
          And Charlie's current turn number is 7
