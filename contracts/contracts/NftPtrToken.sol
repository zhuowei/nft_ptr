// https://soliditydeveloper.com/erc-721
// https://github.com/OpenZeppelin/openzeppelin-contracts
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol";
// import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract NftPtrToken is ERC721PresetMinterPauserAutoId {
    constructor(string memory name_, string memory symbol_, string memory baseTokenURI_) ERC721PresetMinterPauserAutoId(name_, symbol_, baseTokenURI_) {
    }

    function mintId(address to, uint256 tokenId, string memory tokenURI) public {
        require(hasRole(MINTER_ROLE, _msgSender()), "NftPtrToken: must have minter role to mint");
        _mint(to, tokenId);
        // TODO(zhuowei): URI
        // _setTokenURI(tokenId, tokenURI);
    }
}