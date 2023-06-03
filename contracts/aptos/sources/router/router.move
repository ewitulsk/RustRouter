module aptos_router::router {
    use aptos_framework::timestamp;
    use aptos_framework::account;
    use aptos_framework::coin;

    use std::vector;
    use std::signer;

    use aptos_router::pancakepair;
    

    struct RouterStore has key {
        signer_cap: account::SignerCapability
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

    // fun verify_path_pairs_len(path: &vector<address>, pairs: &vector<u64>){
    //     let path_len = vector::length<address>(path);
    //     let pairs_len = vector::length<u64>(pairs);
    //     assert!(path_len == (pairs_len + 1), 0);
    // }

    fun verify_pairs_len(pairs: &vector<u64>){
        let pairs_len = vector::length<u64>(pairs);
        assert!(pairs_len > 0, 0);
    }

    fun initialize_router(
        owner: &signer
    ){
        pancakepair::initialize_pancake_pair(owner);

        let owner_addr = signer::address_of(owner);
        
        assert_router_store_not_exists(owner_addr);

        let (rec_addr, signer_cap) = account::create_resource_account(owner, x"aaa1110000");

        move_to<RouterStore>(owner, RouterStore{signer_cap: signer_cap});
    }



    //TODO: IMPLEMENT THIS FUNCTION
    //Problem: pair swap takes &signer as param, router doesn't have a signer cap for every pair
    //Solution1: Give router a signer cap for every pair <-- Do this one probably
    //Solution2: Make each pair expect the token to already be on it.

    //Note from isn't always the person calling the function, from should also be the previous pair.
    public fun do_swap<IN, OUT>(
        pair: u64, 
        amount_in: u64,
        min_out: u64,
        from: &signer //from
    ) {
        if(pair == 0){
            pancakepair::swap<IN, OUT>(

            )
        }
    }

    public entry fun swap_exact_input_for_output<A,B,C>(
        // path: vector<address>,
        pair_type: vector<u64>,
        pair_resource_addrs: vector<address>,
        //We don't need an extras array yet, we'll deal with that later if needed.
        input_amount: u64,
        min_output_amount: u64,
        to: address,
        deadline: u64,
        store_address: address,
        from: &signer
    ) acquires RouterStore {
        let cur_time = timestamp::now_seconds();
        assert!(cur_time < deadline, 0);

        assert_router_store_exists(store_address);
        // verify_path_pairs_len(&path, &pairs);
        verify_pairs_len(&pairs);

        let router_store = borrow_global_mut<RouterStore>(store_address);
        let resource_signer = account::create_signer_with_capability(&router_store.signer_cap);
        let resource_account_addr = account::get_signer_capability_address(&router_store.signer_cap);

        

        if(coin::is_coin_initialized<A>()){
            let coins = coin::withdraw<A>(from, input_amount);
            coin::deposit<A>(resource_account_addr, coins);
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
                //Output token is B
            };
        }
        else{
            //Output token is A
            let a_bal = coin::balance<A>(resource_account_addr);
            let coins = coin::withdraw<A>(&resource_signer, a_bal);
            coin::deposit<A>(to, coins);
        };


        

    }

    



}