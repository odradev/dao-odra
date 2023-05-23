// use dao::{utils::types::DocumentHash, voting::{types::VotingId, ballot::Choice, voting_engine::voting_state_machine::VotingType as DaoVotingType}};
// use odra::{types::{U512, Bytes, Address}, test_env};

// use crate::{
//     common::{
//         params::{
//             voting::{Ballot, Voting, VotingType},
//             Account,
//             Balance,
//             Contract,
//         },
//         DaoWorld,
//     },
// };

// mod builder;

// #[odra::external_contract]
// trait Voter {
//     fn vote(&mut self, voting_id: VotingId, voting_type: DaoVotingType, choice: Choice, stake: U512);
//     fn finish_voting(&mut self, voting_id: VotingId, voting_type: DaoVotingType);
//     fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
// }

// #[allow(dead_code)]
// impl DaoWorld {
//     pub fn create_voting(&mut self, creator: Account, voting: Voting) {
//         let stake = voting.get_stake();

//         self.set_caller(&creator);

//         match builder::build(self, voting) {
//             builder::VotingSetup::Admin(contract_to_update, action, subject) => self
//                 .admin
//                 .create_voting(contract_to_update, action, subject, *stake),
//             // builder::VotingSetup::Kyc(subject, document_hash) => self
//             //     .kyc_voter
//             //     .create_voting(subject, document_hash, *stake),
//             // builder::VotingSetup::Slasher(address_to_slash, slash_ratio) => self
//             //     .slashing_voter
//             //     .create_voting(address_to_slash, slash_ratio, *stake),
//             // builder::VotingSetup::Repository(
//             //     variable_repository_address,
//             //     key,
//             //     value,
//             //     activation_time,
//             // ) => self.repo_voter.create_voting(
//             //     variable_repository_address,
//             //     key,
//             //     value,
//             //     activation_time,
//             //     *stake,
//             // ),
//             // builder::VotingSetup::Simple(document_hash) => self
//             //     .simple_voter
//             //     .create_voting(document_hash, *stake),
//             // builder::VotingSetup::Reputation(recipient_address, action, amount, document_hash) => {
//             //     self.reputation_voter.create_voting(
//             //         recipient_address,
//             //         action,
//             //         *amount,
//             //         document_hash,
//             //         *stake,
//             //     )
//             // }
//             setup => panic!("{:?} is not supported", setup),
//         }
//     }

//     pub fn create_test_voting(&mut self, contract: Contract, creator: Account, stake: Balance) {
//         let alice = self.get_address(&Account::Alice);
//         let creator = self.get_address(&creator);
//         let document_hash = DocumentHash::from("123");
//         match contract {
//             // Contract::KycVoter => {
//             //     self.kyc_voter.create_voting(alice, document_hash, *stake)
//             // }
//             // Contract::RepoVoter => self.repo_voter.create_voting(
//             //     self.variable_repository.address(),
//             //     String::from("key"),
//             //     Bytes::from(vec![1u8]),
//             //     None,
//             //     *stake,
//             // ),
//             // Contract::ReputationVoter => self.reputation_voter.create_voting(
//             //     alice,
//             //     dao::voting_contracts::reputation_voter::Action::Mint,
//             //     U512::from(10),
//             //     document_hash,
//             //     *stake,
//             // ),
//             Contract::Admin => self.admin.create_voting(
//                 alice,
//                 dao::voting_contracts::admin::Action::AddToWhitelist,
//                 alice,
//                 *stake,
//             ),
//             // Contract::SlashingVoter => self
//             //     .slashing_voter
//             //     .create_voting(alice, 100, *stake),
//             // Contract::SimpleVoter => self
//             //     .simple_voter
//             //     .create_voting(document_hash, *stake),
//             contract => panic!("{:?} is not a voting contract", contract),
//         }
//     }

//     pub fn vote(&mut self, contract: &Account, ballot: &Ballot) {
//         let voter = self.get_address(&ballot.voter);
//         let voting_id = ballot.voting_id;
//         let choice = ballot.choice.clone().into();
//         let stake = ballot.stake.0;
//         let voting_type = ballot.voting_type.into();

//         self.set_caller(ballot.voter)
//         let contract = self.get_address(contract);
//         VoterRef::at(contract).vote(voting_id, voting_type, choice, stake);

//         on_voting_contract!(
//             self,
//             voter,
//             contract,
//             vote(voting_id, voting_type, choice, stake)
//         )
//     }

//     pub fn finish_voting(
//         &mut self,
//         contract: &Contract,
//         voting_id: u32,
//         voting_type: Option<VotingType>,
//     ) {
//         let voting_type = voting_type.map(|vt| vt.into()).unwrap();

//         let result = on_voting_contract!(self, contract, finish_voting(voting_id, voting_type));
//         result.expect(&format!("Couldn't finish {:?} voting", contract));
//     }

//     pub fn voting_exists(
//         &self,
//         contract: &Contract,
//         voting_id: u32,
//         voting_type: VotingType,
//     ) -> bool {
//         let voting_type = voting_type.into();
//         on_voting_contract!(self, contract, voting_exists(voting_id, voting_type))
//     }

//     pub fn checked_slash_voter(&mut self, contract: Contract, voter: Account, voting_id: u32) {
//         let voter = self.get_address(&voter);
//         on_voting_contract!(self, contract, slash_voter(voter, voting_id))
//     }
// }
