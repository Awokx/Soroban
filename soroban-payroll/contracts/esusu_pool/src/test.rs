#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn creates_group_and_allows_members_to_join() {
    let env = Env::default();
    let contract_id = env.register(EsusuPool, ());
    let client = EsusuPoolClient::new(&env, &contract_id);

    let organizer = Address::generate(&env);
    let member_one = Address::generate(&env);
    let member_two = Address::generate(&env);

    let group_id = client.create_group(
        &organizer,
        &String::from_str(&env, "Weekly Savings"),
        &100,
        &7,
        &2,
    );

    assert_eq!(group_id, 1);

    client.join_group(&group_id, &member_one);
    client.join_group(&group_id, &member_two);

    let members = client.list_members(&group_id);
    assert_eq!(members.len(), 2);
}

#[test]
fn records_contributions_and_rotates_payouts() {
    let env = Env::default();
    let contract_id = env.register(EsusuPool, ());
    let client = EsusuPoolClient::new(&env, &contract_id);

    let organizer = Address::generate(&env);
    let member_one = Address::generate(&env);
    let member_two = Address::generate(&env);

    let group_id = client.create_group(
        &organizer,
        &String::from_str(&env, "Weekly Savings"),
        &100,
        &7,
        &2,
    );

    client.join_group(&group_id, &member_one);
    client.join_group(&group_id, &member_two);

    client.contribute(&group_id, &member_one, &100);
    client.contribute(&group_id, &member_two, &100);

    let payout = client.rotate_payout(&group_id, &organizer);
    assert_eq!(payout, member_one);
}

#[test]
#[should_panic(expected = "missed contribution")]
fn rejects_payout_when_current_member_has_not_paid() {
    let env = Env::default();
    let contract_id = env.register(EsusuPool, ());
    let client = EsusuPoolClient::new(&env, &contract_id);

    let organizer = Address::generate(&env);
    let member_one = Address::generate(&env);
    let member_two = Address::generate(&env);

    let group_id = client.create_group(
        &organizer,
        &String::from_str(&env, "Weekly Savings"),
        &100,
        &7,
        &2,
    );

    client.join_group(&group_id, &member_one);
    client.join_group(&group_id, &member_two);

    client.contribute(&group_id, &member_one, &100);
    client.rotate_payout(&group_id, &organizer);

    client.rotate_payout(&group_id, &organizer);
}
