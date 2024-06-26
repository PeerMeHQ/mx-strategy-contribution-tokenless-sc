// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    strategy_tokenless
    (
        init => init
        upgrade => upgrade
        register => register_endpoint
        participate => participate_endpoint
        getDaoVoteWeight => get_dao_vote_weight_view
        getDaoMembers => get_dao_members_view
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
