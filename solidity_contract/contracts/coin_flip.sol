//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Strings.sol";

contract CoinFlip {
    uint256 public totalMatches;
    uint256 public lifetimeValue;
    //Make all public matches accessible based on a match ID
    mapping(uint256 => Match) public matches;

    struct Match {
        address player1;
        address player2;
        uint256 bet1;
        uint256 bet2;
        bool complete;
        uint256 result;
    }

    //Create a new match
    function createMatch() public payable returns (uint256) {
        //Require that the player has sent some ether
        require(msg.value > 0, "You must send some ether to create a match");
        //Set the player1 address to the address of the player who created the match
        matches[totalMatches].player1 = msg.sender;
        //Set the bet1 value to the amount of ether sent
        matches[totalMatches].bet1 = msg.value;
        //Increment the totalMatches counter
        totalMatches++;
        //Increment the lifetimeValue counter
        lifetimeValue += msg.value;
        //Return the match ID
        return totalMatches - 1;
    }

    //Allow a player to join the match based on the ID and flip the coin
    function joinMatch(uint matchId) public payable returns (string memory) {
        //Require that the match ID is valid
        require(matchId < totalMatches, "Invalid match ID");
        //Require that the match is not already complete
        require(
            matches[matchId].complete == false,
            "This match is already complete"
        );
        //Require that the player has sent an amount of ether equal to the bet1 value
        require(
            msg.value == matches[matchId].bet1,
            "You must send the same amount of ether as the bet1 value"
        );
        //Set the player2 address to the address of the player who joined the match
        matches[matchId].player2 = msg.sender;
        //Set the bet2 value to the amount of ether sent
        matches[matchId].bet2 = msg.value;
        //Increment the lifetimeValue counter
        lifetimeValue += msg.value;
        //Flip the coin
        return flipCoin(matchId);
    }

    function flipCoin(uint matchId) private returns (string memory) {
        //Require that the match ID is valid
        require(matchId < totalMatches, "Invalid match ID");
        //Generate a random number between 0 and 1
        uint randomNumber = uint(block.timestamp) % 2;
        //If the number is 0, player1 wins
        if (randomNumber == 0) {
            //Transfer the ether to player1
            payable(matches[matchId].player1).transfer(
                matches[matchId].bet1 + matches[matchId].bet2
            );
            matches[matchId].complete = true;
            matches[matchId].result = 1;
            return "The coin came up heads. player 1 wins";
        }
        //If the number is 1, player2 wins
        else {
            //Transfer the ether to player2
            payable(matches[matchId].player2).transfer(
                matches[matchId].bet1 + matches[matchId].bet2
            );
            matches[matchId].complete = true;
            matches[matchId].result = 2;
            return "The coin came up tails, player 2 wins";
        }
    }
}
