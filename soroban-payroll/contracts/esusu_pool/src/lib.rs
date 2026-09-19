#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Vec};

#[contract]
pub struct EsusuPool;

#[contracttype]
#[derive(Clone)]
pub struct Group {
    pub organizer: Address,
    pub name: String,
    pub contribution_amount: i128,
    pub cycle_length: u64,
    pub member_limit: u32,
    pub members: Vec<Address>,
    pub total_pot: i128,
    pub current_turn: u32,
    pub paid_this_cycle: Map<Address, bool>,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    NextId,
    Group(u64),
}

fn get_group(env: &Env, group_id: u64) -> Group {
    env.storage()
        .persistent()
        .get(&DataKey::Group(group_id))
        .expect("group not found")
}

fn save_group(env: &Env, group_id: u64, group: &Group) {
    env.storage().persistent().set(&DataKey::Group(group_id), group);
}

#[contractimpl]
impl EsusuPool {
    pub fn create_group(
        env: Env,
        organizer: Address,
        name: String,
        contribution_amount: i128,
        cycle_length: u64,
        member_limit: u32,
    ) -> u64 {
        if contribution_amount <= 0 {
            panic!("contribution amount must be positive");
        }
        if member_limit == 0 {
            panic!("member limit must be greater than zero");
        }

        let group_id = env
            .storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(1u64);

        env.storage().instance().set(
            &DataKey::NextId,
            &(group_id + 1),
        );

        let group = Group {
            organizer,
            name,
            contribution_amount,
            cycle_length,
            member_limit,
            members: Vec::new(&env),
            total_pot: 0,
            current_turn: 0,
            paid_this_cycle: Map::new(&env),
        };

        env.storage().persistent().set(&DataKey::Group(group_id), &group);
        group_id
    }

    pub fn join_group(env: Env, group_id: u64, member: Address) {
        let mut group = get_group(&env, group_id);

        if group.members.len() >= group.member_limit {
            panic!("member limit reached");
        }

        let mut already_member = false;
        for existing in group.members.iter() {
            if existing == member {
                already_member = true;
                break;
            }
        }

        if already_member {
            panic!("member already joined");
        }

        group.members.push_back(member);
        save_group(&env, group_id, &group);
    }

    pub fn contribute(env: Env, group_id: u64, member: Address, amount: i128) {
        let mut group = get_group(&env, group_id);

        if amount != group.contribution_amount {
            panic!("invalid contribution amount");
        }

        let mut is_member = false;
        for existing in group.members.iter() {
            if existing == member {
                is_member = true;
                break;
            }
        }

        if !is_member {
            panic!("member is not part of this group");
        }

        if group.paid_this_cycle.get(member.clone()).unwrap_or(false) {
            panic!("duplicate contribution");
        }

        group.paid_this_cycle.set(member.clone(), true);
        group.total_pot += amount;
        save_group(&env, group_id, &group);
    }

    pub fn rotate_payout(env: Env, group_id: u64, _admin: Address) -> Address {
        let mut group = get_group(&env, group_id);

        if group.members.len() == 0 {
            panic!("group has no members");
        }

        let current_index = group.current_turn as u32;
        let current_member = group
            .members
            .get(current_index)
            .expect("invalid payout turn");

        if !group
            .paid_this_cycle
            .get(current_member.clone())
            .unwrap_or(false)
        {
            panic!("missed contribution");
        }

        group.current_turn = if group.current_turn + 1 >= group.members.len() {
            0
        } else {
            group.current_turn + 1
        };
        group.paid_this_cycle = Map::new(&env);
        save_group(&env, group_id, &group);

        current_member
    }

    pub fn list_members(env: Env, group_id: u64) -> Vec<Address> {
        let group = get_group(&env, group_id);
        group.members
    }
}

mod test;
