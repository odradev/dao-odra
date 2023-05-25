# Urgent
- Unifiy U256 and U512 to be Balance in contract and tests.
    - U256 and Balance in NFT contract.
    - Add total_supply to ERC721 base?

# Other
- Unify ContractRefsStorage and ContractRefsWithKycStorage.
- VotingEngine should have KycInfo features.
- finish_voting returns VotingSummary, but when used via delegate as an entrypoint it should not return anything. Allow Odra to supress return value?
- remove refs from voters?
- Allow for owning contracts by deployer.
