#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct EntityInfo<M: ManagedTypeApi> {
    pub accepted_token: TokenIdentifier<M>,
}

#[multiversx_sc::contract]
pub trait StrategyContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(register)]
    fn register_endpoint(&self, accepted_token: TokenIdentifier) {
        let caller = self.blockchain().get_caller();
        require!(self.blockchain().is_smart_contract(&caller), "not a contract");
        require!(self.entity_infos(&caller).is_empty(), "already registered");

        self.entity_infos(&caller).set(EntityInfo { accepted_token });
    }

    #[payable("*")]
    #[endpoint(participate)]
    fn participate_endpoint(&self, entity: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        let entity_info = self.get_entity_info_or_fail(&entity);
        require!(payment.amount > 0, "must be more than 0");
        require!(payment.token_identifier == entity_info.accepted_token, "invalid token");

        let existing_weight = self.members(&entity).get(&caller).unwrap_or_default();
        let new_weight = &existing_weight + &payment.amount;

        self.members(&entity).insert(caller.clone(), new_weight);

        self.tx().to(&entity).esdt(payment).transfer();
    }

    #[view(getDaoVoteWeight)]
    fn get_dao_vote_weight_view(&self, address: ManagedAddress, token: OptionalValue<TokenIdentifier>) -> BigUint {
        let entity = self.blockchain().get_caller();

        self.members(&entity).get(&address).unwrap_or_default()
    }

    #[view(getDaoMembers)]
    fn get_dao_members_view(&self, token: OptionalValue<TokenIdentifier>) -> MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>> {
        let entity = self.blockchain().get_caller();
        let mut members_multi = MultiValueEncoded::new();

        for (address, weight) in self.members(&entity).iter() {
            members_multi.push((address, weight).into());
        }

        members_multi.into()
    }

    fn get_entity_info_or_fail(&self, address: &ManagedAddress) -> EntityInfo<Self::Api> {
        require!(!self.entity_infos(address).is_empty(), "entity not found");

        self.entity_infos(address).get()
    }

    #[storage_mapper("members")]
    fn members(&self, address: &ManagedAddress) -> MapMapper<ManagedAddress, BigUint>;

    #[storage_mapper("entity_infos")]
    fn entity_infos(&self, address: &ManagedAddress) -> SingleValueMapper<EntityInfo<Self::Api>>;
}
