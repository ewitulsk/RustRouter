module aptos_router::router {
    use aptos_framework::timestamp;
    use aptos_framework::account;
    use aptos_framework::coin;

    use aptos_std::math64::pow;

    use std::vector;
    use std::signer;

    use aptos_router::pancakepair::{Self};

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

    struct RouterStore has key {
        router_signer_cap: account::SignerCapability,
        router_resource_account_addr: address,
    }

    struct UninitializedCoin {}

    fun assert_router_store_exists(
        store_holder_addr: address
    ){
        assert!(exists<RouterStore>(store_holder_addr), 0);
    }

    fun assert_router_store_not_exists(
        store_holder_addr: address
    ){
        assert!(!exists<RouterStore>(store_holder_addr), 0);
    }

    fun initialize_router(
        owner: &signer
    ){
        // let pancake_resource_account_addr = pancakepair::initialize_pancake_pair(owner);

        let owner_addr = signer::address_of(owner);
        
        assert_router_store_not_exists(owner_addr);

        let (rec_addr, router_signer_cap) = account::create_resource_account(owner, x"aaa2220000");
        let router_resource_account_addr = account::get_signer_capability_address(&router_signer_cap);

        move_to<RouterStore>(owner, RouterStore{
                router_signer_cap: router_signer_cap, 
                router_resource_account_addr: router_resource_account_addr
            }
        );
    }

    fun check_and_reg_tokens<IN, OUT>(store_address: address) acquires RouterStore {
        let router_store = borrow_global_mut<RouterStore>(store_address);
        let router_resource_signer = account::create_signer_with_capability(&router_store.router_signer_cap);
        let router_resource_account_addr = account::get_signer_capability_address(&router_store.router_signer_cap);

        if(!coin::is_account_registered<IN>(router_resource_account_addr)){
            coin::register<IN>(&router_resource_signer);
        };

        if(!coin::is_account_registered<OUT>(router_resource_account_addr)){
            coin::register<OUT>(&router_resource_signer);
        };
    }

    fun check_and_reg_token<IN>(store_address: address) acquires RouterStore {
        let router_store = borrow_global_mut<RouterStore>(store_address);
        let router_resource_signer = account::create_signer_with_capability(&router_store.router_signer_cap);
        let router_resource_account_addr = account::get_signer_capability_address(&router_store.router_signer_cap);

        if(!coin::is_account_registered<IN>(router_resource_account_addr)){
            coin::register<IN>(&router_resource_signer);
        };
    }


    //Note from isn't always the person calling the function, from should also be the previous pair.
    public fun do_swap<IN, OUT>(
        pair: u64, 
        amount_in: u64,
        store_holder_addr: address
    ): u64 acquires RouterStore { 
        let router_store = borrow_global_mut<RouterStore>(store_holder_addr);
        let router_resource_signer = account::create_signer_with_capability(&router_store.router_signer_cap);
        let router_resource_account_addr = account::get_signer_capability_address(&router_store.router_signer_cap);

        let output_amount: u64 = 0;

        check_and_reg_tokens<IN, OUT>(store_holder_addr);

        if(pair == 0){
            
            output_amount = pancakepair::swap_pancake_pair<IN, OUT>(
                amount_in,
                &router_resource_signer,
                router_resource_account_addr
            )
        };

        return output_amount
    }

    //Every pair expects their resource account to have the required tokens on them.
    public entry fun swap_exact_input_for_output<A,B,C>(
        pair_types: vector<u64>,
        //We don't need an extras array yet, we'll deal with that later if needed.
        input_amount: u64,
        min_output_amount: u64,
        from: &signer,
        to: address,
        deadline: u64,
        store_address: address
    ) acquires RouterStore {
        let cur_time = timestamp::now_seconds();
        assert!(cur_time < deadline, 0);

        assert_router_store_exists(store_address);

        let router_store = borrow_global_mut<RouterStore>(store_address);
        let router_resource_signer = account::create_signer_with_capability(&router_store.router_signer_cap);
        let router_resource_account_addr = account::get_signer_capability_address(&router_store.router_signer_cap);

        let output_amount: u64 = 0;        

        if(coin::is_coin_initialized<A>()){
            check_and_reg_token<A>(store_address);
            let coins = coin::withdraw<A>(from, input_amount);
            coin::deposit<A>(router_resource_account_addr, coins);
        }
        else{
            //You need to have a coin to swap
            abort(0);
        };


        if(coin::is_coin_initialized<B>()){
            
            if(coin::is_coin_initialized<C>()){
                //Check D, Ect
            }
            else{
                //Do Swap A->B
                let pair_type = *vector::borrow<u64>(&pair_types, 0);
                output_amount = do_swap<A, B>(
                    pair_type,
                    input_amount,
                    store_address
                );
                //Output is B
                let coins = coin::withdraw<B>(&router_resource_signer, output_amount);
                coin::deposit<B>(to, coins);
            };
        }
        else{
            //Output token is A
            let coins = coin::withdraw<A>(&router_resource_signer, input_amount);
            coin::deposit<A>(to, coins);
        };
    }

    const MAX_U64: u64 = 18446744073709551615;
    const MINIMUM_LIQUIDITY: u128 = 1000;

    #[test(router_signer = @111111, dev = @dev, admin = @default_admin, resource_account = @pancake, treasury = @0x23456, bob = @0x12345, alice = @0x12346)]
    fun test_router(
        router_signer: &signer,
        dev: &signer,
        admin: &signer,
        resource_account: &signer,
        treasury: &signer,
        bob: &signer,
        alice: &signer
    ) acquires RouterStore {
        account::create_account_for_test(signer::address_of(bob));
        account::create_account_for_test(signer::address_of(alice));
        account::create_account_for_test(signer::address_of(router_signer));

        let router_addr = signer::address_of(router_signer);
        let alice_addr = signer::address_of(alice);

        initialize_router(router_signer);

        setup_test_with_genesis(dev, admin, treasury, resource_account);

        let router_store = borrow_global_mut<RouterStore>(router_addr);
        let router_resource_signer = account::create_signer_with_capability(&router_store.router_signer_cap);
        let router_resource_account_addr = account::get_signer_capability_address(&router_store.router_signer_cap);

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

        check_and_reg_token<TestCAKE>(router_addr);

        let coins = coin::withdraw<TestCAKE>(alice, input_x);
        coin::deposit<TestCAKE>(router_resource_account_addr, coins); //Deposit onto resource account
        
        coin::register<TestBUSD>(alice);

        let output_amount = do_swap<TestCAKE, TestBUSD>(0, input_x, router_addr);
        
        let coins = coin::withdraw<TestBUSD>(&router_resource_signer, output_amount);
        coin::deposit<TestBUSD>(alice_addr, coins);

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

    // public entry fun swap_exact_input_for_output<A,B,C>(
    // pair_types: vector<u64>,
    // input_amount: u64,
    // min_output_amount: u64,
    // from: &signer,
    // to: address,
    // deadline: u64,
    // store_address: address
    // )

    #[test(router_signer = @111111, dev = @dev, admin = @default_admin, resource_account = @pancake, treasury = @0x23456, bob = @0x12345, alice = @0x12346)]
    fun test_router_do_swap_pancake(
        router_signer: &signer,
        dev: &signer,
        admin: &signer,
        resource_account: &signer,
        treasury: &signer,
        bob: &signer,
        alice: &signer
    ) acquires RouterStore {
        account::create_account_for_test(signer::address_of(bob));
        account::create_account_for_test(signer::address_of(alice));
        account::create_account_for_test(signer::address_of(router_signer));

        let router_addr = signer::address_of(router_signer);
        let alice_addr = signer::address_of(alice);

        initialize_router(router_signer);

        setup_test_with_genesis(dev, admin, treasury, resource_account);

        let router_store = borrow_global_mut<RouterStore>(router_addr);
        let router_resource_signer = account::create_signer_with_capability(&router_store.router_signer_cap);
        let router_resource_account_addr = account::get_signer_capability_address(&router_store.router_signer_cap);

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

        //Register output to Alice
        coin::register<TestBUSD>(alice);
        


        let deadline = timestamp::now_seconds() + 10; 
        let pair_types = vector::empty<u64>();
        vector::push_back<u64>(&mut pair_types, 0);
        
         // public entry fun swap_exact_input_for_output<A,B,C>(
        // pair_types: vector<u64>,
        // input_amount: u64,
        // min_output_amount: u64,
        // from: &signer,
        // to: address,
        // deadline: u64,
        // store_address: address
        // )
        swap_exact_input_for_output<TestCAKE, TestBUSD, UninitializedCoin>(
            pair_types,
            input_x,
            0,
            alice,
            alice_addr,
            deadline,
            router_addr
        );

        let alice_token_x_after_balance = coin::balance<TestCAKE>(signer::address_of(alice));
        let alice_token_y_after_balance = coin::balance<TestBUSD>(signer::address_of(alice));

        let output_y = swap_utils::get_amount_out(input_x, initial_reserve_x, initial_reserve_y);

        assert!((alice_token_x_before_balance - alice_token_x_after_balance) == input_x, 93);
        assert!(alice_token_y_after_balance == (output_y as u64), alice_token_y_after_balance);
    }


    

}