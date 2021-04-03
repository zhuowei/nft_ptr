// SPDX-License-Identifier: UNLICENSED
// https://soliditydeveloper.com/erc-721
// https://github.com/OpenZeppelin/openzeppelin-contracts
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

// ERC721 token with each token representing a memory address managed under an nft_ptr.
// One instance of this contract is created per program launch.
// Based on ERC721PresetMinterPauserAutoId, but simplified so only the owner can mint and pause.
// The entire permissions system of ERC721 is disabled here:
// The account that created the token contract always gains full control over all tokens for simplicity.
// The proper way to do this would be to send transfers out of the owner contract instead,
// for a proof of concept, this is fine.
contract NftPtrToken is ERC721Enumerable, ERC721Burnable, ERC721Pausable, ERC721URIStorage {
    address private _owner;
    string private _baseTokenURI;
    constructor(string memory name_, string memory symbol_, string memory baseTokenURI_) ERC721(name_, symbol_) {
        _owner = _msgSender();
        _baseTokenURI = baseTokenURI_;
    }

    function _baseURI() internal view virtual override returns (string memory) {
        return _baseTokenURI;
    }

    function tokenURI(uint256 tokenId) public view virtual override (ERC721, ERC721URIStorage) returns (string memory) {
        return super.tokenURI(tokenId);
    }

    function mintOrMove(address owner, address previousOwner, uint256 tokenId, string memory tokenURIStorage, string memory /*callerPC*/) public {
        require(_owner == msg.sender, "NftPtrToken: must be the owner to mintOrMove");
        if (!_exists(tokenId)) {
            require(previousOwner == msg.sender, "NftPtrToken: tried to move a non-minted token");
            _mint(owner, tokenId);
        } else {
            // TODO(zhuowei): we don't really need to safe transfer if we know the contract's fine...
            safeTransferFrom(previousOwner, owner, tokenId, "");
        }
        // grant ourself approval over the token.
        _approve(_owner, tokenId);
        _setTokenURI(tokenId, tokenURIStorage);
    }

    function pause() public virtual {
        require(_owner == msg.sender, "NftPtrToken: must be the owner to pause");
        _pause();
    }

    function unpause() public virtual {
        require(_owner == msg.sender, "NftPtrToken: must be the owner to unpause");
        _unpause();
    }

    function _burn(uint256 tokenId) internal virtual override (ERC721, ERC721URIStorage) {
        super._burn(tokenId);
    }

    function _beforeTokenTransfer(address from, address to, uint256 tokenId) internal virtual override(ERC721, ERC721Enumerable, ERC721Pausable) {
        super._beforeTokenTransfer(from, to, tokenId);
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC721, ERC721Enumerable) returns (bool) {
        return super.supportsInterface(interfaceId);
    }
}