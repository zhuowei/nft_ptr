// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
// TODO(zhuowei): update OpenZeppelin to a release with this header
// import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

// Represents an nft_ptr instance on the Blockchain.
// Can receive NftPtrTokens; itself is a stub ERC20 that issues 0 tokens
// partially so Etherscan would display our name, and partially for future compatibility
// with nft_shared_ptr
contract NftPtrOwner is ERC721Holder, IERC20 {
    string private _name;
    constructor(string memory name_) {
        _name = name_;
    }
    // IERC20: Stub ERC20 since a nft_ptr doesn't perform reference counting.

    function totalSupply() external pure override returns (uint256) {
        return 0;
    }
    function balanceOf(address) external pure override returns (uint256) {
        return 0;
    }
    function transfer(address, uint256) external pure override returns (bool) {
        return false;
    }

    function allowance(address, address) external pure override returns (uint256) {
        return 0;
    }

    function approve(address, uint256) external pure override returns (bool) {
        return false;
    }

    function transferFrom(address, address, uint256) external pure override returns (bool) {
        return false;
    }

    // IERC20Metadata

    function name() public view virtual returns (string memory) {
        return _name;
    }

    function symbol() public view virtual returns (string memory) {
        return "PTR";
    }

    function decimals() public view virtual returns (uint8) {
        return 0;
    }
}