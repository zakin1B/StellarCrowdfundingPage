#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Campaign {
    pub owner: Address,
    pub title: String,
    pub description: String,
    pub goal_amount: i128,
    pub total_raised: i128,
}

#[contracttype]
pub enum DataKey {
    Campaign,
    Donation(Address),
}

#[contract]
pub struct StellarCrowdfunding;

#[contractimpl]
impl StellarCrowdfunding {
    pub fn create_campaign(
        env: Env,
        owner: Address,
        title: String,
        description: String,
        goal_amount: i128,
    ) {
        owner.require_auth();

        let campaign = Campaign {
            owner: owner.clone(),
            title,
            description,
            goal_amount,
            total_raised: 0,
        };

        env.storage().persistent().set(&DataKey::Campaign, &campaign);

        env.events().publish(
            (symbol_short!("CREATE"), owner),
            goal_amount,
        );
    }

    pub fn donate(
        env: Env,
        donor: Address,
        amount: i128,
    ) -> i128 {
        donor.require_auth();

        if amount <= 0 {
            panic!("Donation amount must be greater than 0");
        }

        let mut campaign: Campaign = env
            .storage()
            .persistent()
            .get(&DataKey::Campaign)
            .expect("Campaign not found");

        campaign.total_raised += amount;

        let previous_donation: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Donation(donor.clone()))
            .unwrap_or(0);

        let new_total_donation = previous_donation + amount;

        env.storage()
            .persistent()
            .set(&DataKey::Donation(donor.clone()), &new_total_donation);

        env.storage()
            .persistent()
            .set(&DataKey::Campaign, &campaign);

        env.events().publish(
            (symbol_short!("DONATE"), donor),
            amount,
        );

        campaign.total_raised
    }

    pub fn get_campaign(env: Env) -> Campaign {
        env.storage()
            .persistent()
            .get(&DataKey::Campaign)
            .expect("Campaign not found")
    }

    pub fn get_total_raised(env: Env) -> i128 {
        let campaign: Campaign = env
            .storage()
            .persistent()
            .get(&DataKey::Campaign)
            .expect("Campaign not found");

        campaign.total_raised
    }

    pub fn get_goal_amount(env: Env) -> i128 {
        let campaign: Campaign = env
            .storage()
            .persistent()
            .get(&DataKey::Campaign)
            .expect("Campaign not found");

        campaign.goal_amount
    }

    pub fn get_donation(env: Env, donor: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Donation(donor))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_create_campaign() {
        let env = Env::default();
        let contract_id = env.register(StellarCrowdfunding, ());
        let client = StellarCrowdfundingClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        env.mock_all_auths();

        client.create_campaign(
            &owner,
            &String::from_str(&env, "Stellar Student Fund"),
            &String::from_str(&env, "Support student builders on Stellar"),
            &1000,
        );

        let campaign = client.get_campaign();

        assert_eq!(campaign.owner, owner);
        assert_eq!(campaign.goal_amount, 1000);
        assert_eq!(campaign.total_raised, 0);
    }

    #[test]
    fn test_donate() {
        let env = Env::default();
        let contract_id = env.register(StellarCrowdfunding, ());
        let client = StellarCrowdfundingClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let donor = Address::generate(&env);
        env.mock_all_auths();

        client.create_campaign(
            &owner,
            &String::from_str(&env, "Stellar Student Fund"),
            &String::from_str(&env, "Support student builders on Stellar"),
            &1000,
        );

        let total = client.donate(&donor, &100);

        assert_eq!(total, 100);
    }

    #[test]
    fn test_get_donation() {
        let env = Env::default();
        let contract_id = env.register(StellarCrowdfunding, ());
        let client = StellarCrowdfundingClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let donor = Address::generate(&env);
        env.mock_all_auths();

        client.create_campaign(
            &owner,
            &String::from_str(&env, "Stellar Student Fund"),
            &String::from_str(&env, "Support student builders on Stellar"),
            &1000,
        );

        client.donate(&donor, &250);

        let donation = client.get_donation(&donor);

        assert_eq!(donation, 250);
    }
}