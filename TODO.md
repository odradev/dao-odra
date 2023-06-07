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

# Final
- TODOs
- install at given key, decide if upgradable
- slashing
- gas optimization
- livenet installer
- move to make repo
    - enable GA tests,
- docs
- delegate
- stable rust
- supress supressed errors
- Optimize dao_world.


# Slashing
post_job_offer
    - add job_offer_id to active_job_offers_ids
submit_bid
    - add bid to active bids
cancel_job_offer
    - remove job_offer_id from active_job_offers_ids
cancel_bid
    - remove bid from active bids
pick_bid
    - remove job_offer_id from active job offers ids
    - add job_id to active jobs.

BidEngine::slash_voter? Confirm logic.
    