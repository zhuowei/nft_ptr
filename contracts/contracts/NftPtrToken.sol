// https://soliditydeveloper.com/erc-721
// https://github.com/OpenZeppelin/openzeppelin-contracts
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract NftPtrToken is ERC721 {
    constructor() ERC721("MyCollectible", "MCO") {
    }
}