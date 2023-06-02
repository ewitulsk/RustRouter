module aptos_router::pancakepair {
    use std::signer;

    use aptos_std::math64::pow;

    use aptos_framework::account;
    use aptos_framework::coin;
    use aptos_framework::resource_account;
    use aptos_framework::genesis;

    use pancake::math;
    use pancake::router;
    use pancake::swap_utils;
    use pancake::swap;

    #[test_only]
    use pancake::swap::{initialize};
    #[test_only]
    use pancake::swap_test::{setup_test_with_genesis, setup_test};
    #[test_only]
    use test_coin::test_coins::{Self, TestCAKE, TestBUSD, TestUSDC, TestBNB, TestAPT};
    
    struct PairStore has key {
        signer_cap: account::SignerCapability
    }

    fun assert_pair_store_exists(
        store_holder_addr: address
    ){
        assert!(exists<PairStore>(store_holder_addr), 0);
    }

    fun assert_pair_store_not_exists(
        store_holder_addr: address
    ){
        assert!(!exists<PairStore>(store_holder_addr), 0);
    }

    fun token_order_correct<X, Y>(): bool{
        swap_utils::sort_token_type<X, Y>()
    }

    fun initialize_pair(
        owner: &signer
    ){
        let owner_addr = signer::address_of(owner);
        
        assert_pair_store_not_exists(owner_addr);

        let (rec_addr, signer_cap) = account::create_resource_account(owner, x"aaa1110000");

        move_to<PairStore>(owner, PairStore{signer_cap: signer_cap});
    }

    //Swap pulls the required token from "from" and expects the next pair to do the same.
    fun swap<I, O>(
        amount_in: u64,
        min_out: u64,
        from: &signer,
        store_holder_addr: address
    ): u64 acquires PairStore {
        assert_pair_store_exists(store_holder_addr);
        let pair_store = borrow_global_mut<PairStore>(store_holder_addr);
        let resource_signer = account::create_signer_with_capability(&pair_store.signer_cap);
        let resource_account_addr = account::get_signer_capability_address(&pair_store.signer_cap);

        if(!coin::is_account_registered<I>(resource_account_addr)){
            coin::register<I>(&resource_signer);
        };

        let coins = coin::withdraw<I>(from, amount_in);
        coin::deposit<I>(resource_account_addr, coins);

        let reserve_x: u64;
        let reserve_y: u64;
        let output_amount: u64;
        if(token_order_correct<I, O>()){
            (reserve_x, reserve_y, _) =  swap::token_reserves<I, O>();
            output_amount = swap_utils::get_amount_out(amount_in, reserve_x, reserve_y);
        }
        else {
            (reserve_x, reserve_y, _) =  swap::token_reserves<O, I>();
            output_amount = swap_utils::get_amount_out(amount_in, reserve_y, reserve_x);
        };

        router::swap_exact_input<I, O>(&resource_signer, amount_in, min_out);

        return output_amount
    }

    #[test(owner=@0x1234)]
    fun test_initialize_pair(
        owner: &signer
    ){
        let owner_addr = signer::address_of(owner);
        initialize_pair(owner);
        assert_pair_store_exists(owner_addr);
    }

    #[test_only]
    fun transfer_output<O>(store_holder_addr: address, to: address) acquires PairStore{
        assert_pair_store_exists(store_holder_addr);
        let pair_store = borrow_global_mut<PairStore>(store_holder_addr);
        let resource_signer = account::create_signer_with_capability(&pair_store.signer_cap);
        let resource_account_addr = account::get_signer_capability_address(&pair_store.signer_cap);

        let bal = coin::balance<O>(resource_account_addr);
        coin::transfer<O>(&resource_signer, to, bal);
    }

    const MAX_U64: u64 = 18446744073709551615;
    const MINIMUM_LIQUIDITY: u128 = 1000;

    //copied from pancake tests (https://github.com/pancakeswap/pancake-contracts-move/blob/main/pancake-swap/sources/test/swap_test.move)
    #[test(dev = @dev, admin = @default_admin, resource_account = @pancake, treasury = @0x23456, bob = @0x12345, alice = @0x12346)]
    fun test_pancake_swap(
        dev: &signer,
        admin: &signer,
        resource_account: &signer,
        treasury: &signer,
        bob: &signer,
        alice: &signer
    ) {
        account::create_account_for_test(signer::address_of(bob));
        account::create_account_for_test(signer::address_of(alice));

        setup_test_with_genesis(dev, admin, treasury, resource_account);

        let coin_owner = test_coins::init_coins();

        test_coins::register_and_mint<TestCAKE>(&coin_owner, bob, 100 * pow(10, 8));
        test_coins::register_and_mint<TestBUSD>(&coin_owner, bob, 100 * pow(10, 8));
        test_coins::register_and_mint<TestCAKE>(&coin_owner, alice, 100 * pow(10, 8));

        let initial_reserve_x = 5 * pow(10, 8);
        let initial_reserve_y = 10 * pow(10, 8);
        let input_x = 2 * pow(10, 8);

        //bob provides 5:10 CAKE-BUSD liq
        router::add_liquidity<TestCAKE, TestBUSD>(bob, initial_reserve_x, initial_reserve_y, 0, 0);
        let bob_suppose_lp_balance = math::sqrt(((initial_reserve_x as u128) * (initial_reserve_y as u128))) - MINIMUM_LIQUIDITY;
        let suppose_total_supply = bob_suppose_lp_balance + MINIMUM_LIQUIDITY;

        let alice_token_x_before_balance = coin::balance<TestCAKE>(signer::address_of(alice));

        router::swap_exact_input<TestCAKE, TestBUSD>(alice, input_x, 0);

        let alice_token_x_after_balance = coin::balance<TestCAKE>(signer::address_of(alice));
        let alice_token_y_after_balance = coin::balance<TestBUSD>(signer::address_of(alice));

        let output_y = swap_utils::get_amount_out(input_x, initial_reserve_x, initial_reserve_y);
        let new_reserve_x = initial_reserve_x + input_x;
        let new_reserve_y = initial_reserve_y - (output_y as u64);

        let (reserve_y, reserve_x, _) = swap::token_reserves<TestBUSD, TestCAKE>();
        assert!((alice_token_x_before_balance - alice_token_x_after_balance) == input_x, 99);
        assert!(alice_token_y_after_balance == (output_y as u64), 98);
        assert!(reserve_x == new_reserve_x, 97);
        assert!(reserve_y == new_reserve_y, 96);
        
        assert!(token_order_correct<TestBUSD, TestCAKE>() == true, 95);
    }

     #[test(pair_signer = @111111, dev = @dev, admin = @default_admin, resource_account = @pancake, treasury = @0x23456, bob = @0x12345, alice = @0x12346)]
    fun test_pair_swap(
        pair_signer: &signer,
        dev: &signer,
        admin: &signer,
        resource_account: &signer,
        treasury: &signer,
        bob: &signer,
        alice: &signer
    ) acquires PairStore {
        account::create_account_for_test(signer::address_of(bob));
        account::create_account_for_test(signer::address_of(alice));
        account::create_account_for_test(signer::address_of(pair_signer));

        let pair_addr = signer::address_of(pair_signer);
        let alice_addr = signer::address_of(alice);

        initialize_pair(pair_signer);

        setup_test_with_genesis(dev, admin, treasury, resource_account);

        let coin_owner = test_coins::init_coins();

        test_coins::register_and_mint<TestCAKE>(&coin_owner, bob, 100 * pow(10, 8));
        test_coins::register_and_mint<TestBUSD>(&coin_owner, bob, 100 * pow(10, 8));
        test_coins::register_and_mint<TestCAKE>(&coin_owner, alice, 100 * pow(10, 8));

        let initial_reserve_x = 5 * pow(10, 8);
        let initial_reserve_y = 10 * pow(10, 8);
        let input_x = 2 * pow(10, 8);

        //bob provides 5:10 CAKE-BUSD liq
        router::add_liquidity<TestCAKE, TestBUSD>(bob, initial_reserve_x, initial_reserve_y, 0, 0);
        let bob_suppose_lp_balance = math::sqrt(((initial_reserve_x as u128) * (initial_reserve_y as u128))) - MINIMUM_LIQUIDITY;
        let suppose_total_supply = bob_suppose_lp_balance + MINIMUM_LIQUIDITY;

        let alice_token_x_before_balance = coin::balance<TestCAKE>(signer::address_of(alice));

        coin::register<TestBUSD>(alice);
        let output_amount = swap<TestCAKE, TestBUSD>(input_x, 0, alice, pair_addr);
        transfer_output<TestBUSD>(pair_addr, alice_addr);

        let alice_token_x_after_balance = coin::balance<TestCAKE>(signer::address_of(alice));
        let alice_token_y_after_balance = coin::balance<TestBUSD>(signer::address_of(alice));

        let output_y = swap_utils::get_amount_out(input_x, initial_reserve_x, initial_reserve_y);

        assert!(output_y == output_amount, 94);

        let new_reserve_x = initial_reserve_x + input_x;
        let new_reserve_y = initial_reserve_y - (output_y as u64);

        let (reserve_y, reserve_x, _) = swap::token_reserves<TestBUSD, TestCAKE>();
        assert!((alice_token_x_before_balance - alice_token_x_after_balance) == input_x, 93);
        assert!(alice_token_y_after_balance == (output_y as u64), 92);
        assert!(reserve_x == new_reserve_x, 91);
        assert!(reserve_y == new_reserve_y, 90);
        
    }
}