pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";

contract NftPtrOwner is ERC721Holder {
    string private _name;
    constructor(string memory name_) {
        _name = name_;
    }
}