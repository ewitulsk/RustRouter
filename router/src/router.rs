use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::pairs::Pair;

pub struct Route {
    pub pairs: Vec<Rc<RefCell<Box<dyn Pair>>>>,
    pub path: Vec<String>,
    pub path_amounts: Vec<u64>
}

pub fn find_best_routes_for_fixed_input_amount(
    pairs_by_token: &HashMap<String, Vec<Rc<RefCell<Box<dyn Pair>>>>>,
    input_token: &String,
    output_token: &String,
    input_amount: u64,
    max_swaps: u8
) -> Vec<Rc<Route>>{

    let mut completed_routes: Vec<Rc<Route>> = Vec::new();

    //Token Address -> Route
    let mut current_routes: HashMap<String, Rc<Route>> = HashMap::new();

    //Token Address -> OutputAmount
    let mut max_output_amounts: HashMap<String, u64> = HashMap::new();  

    current_routes.insert(input_token.to_string(), Rc::new(Route {
         pairs: Vec::new(), 
         path: vec![input_token.to_string()], 
         path_amounts: vec![input_amount] 
        }
    ));

    max_output_amounts.insert(input_token.to_string(), input_amount);

    let mut d=0;
    while d<max_swaps {

        //Token Addr -> Route
        let mut next_routes: HashMap<String, Rc<Route>> = HashMap::new();
        for route in current_routes.values_mut() {
            let route_output_token = &route.path[route.path.len()-1];
            let route_output_amount = route.path_amounts[route.path_amounts.len()-1];
            let matching_pairs = pairs_by_token.get(route_output_token).unwrap();
            for pair_ref in matching_pairs {
                let pair = (**pair_ref).borrow();
                let token_arr = pair.get_token_arr();

                for pair_output_token in token_arr {

                    if pair_output_token == route_output_token {
                        continue;
                    }

                    if !token_arr.contains(&route_output_token) {
                        panic!("route output token not contained in token_arr");
                    }

                    //skip if pair already exists in the route, we have already explored better routes before.
                    let mut pair_key_already_exists = false;
                    for p_ref in &route.pairs {
                        let p = (**p_ref).borrow();
                        if p.get_pair_key() == pair.get_pair_key() {
                            pair_key_already_exists = true;
                        }
                    }
                    if pair.get_pair_key() != String::new() && pair_key_already_exists {
                        continue;
                    }
                    
                    let pair_output_amount = pair.output_amount(route_output_amount, route_output_token, pair_output_token);
                    let cur_max_ouput = max_output_amounts.get(pair_output_token).unwrap_or(&0);

                    //check to see if we have a better route
                    if cur_max_ouput >= &pair_output_amount {
                        continue;
                    }

                    max_output_amounts.insert(pair_output_token.clone(), pair_output_amount);
                    
                    let mut new_pairs: Vec<Rc<RefCell<Box<dyn Pair>>>> = Vec::new();
                    new_pairs.extend(route.pairs.iter().cloned());
                    new_pairs.push(pair_ref.clone());

                    let mut new_path: Vec<String> = Vec::new();
                    new_path.extend(route.path.iter().cloned());
                    new_path.push(pair_output_token.to_string());

                    let mut new_path_amounts: Vec<u64> = Vec::new();
                    new_path_amounts.extend(route.path_amounts.iter().cloned());
                    new_path_amounts.push(pair_output_amount);

                    let pair_route = Route {
                        pairs: new_pairs,
                        path: new_path,
                        path_amounts: new_path_amounts
                    };

                    let pair_route_ref = Rc::new(pair_route);

                    next_routes.insert(pair_output_token.to_string(), pair_route_ref.clone());
                    if pair_output_token == output_token {
                        completed_routes.push(pair_route_ref);
                    }
                }
            }
        }
        current_routes = next_routes;

        if current_routes.len() == 0 {
            break;
        }

        d+=1;
    }

    completed_routes.sort_by(|a, b| a.path_amounts[a.path_amounts.len()-1].cmp(&b.path_amounts[b.path_amounts.len()-1]));

    return completed_routes;
}