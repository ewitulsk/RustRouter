use std::{any::Any};

use super::{Pair, PairMetadata, PairNames, Descriptor};

use serde::{Serialize, Deserialize};
use serde_json::Value;
use ethers_core::types::U256;

#[derive(Serialize, Deserialize)]
pub struct LiquidswapDescriptor {
    pub network: String,
    pub protocol: String,
    pub pair_name: PairNames,
    pub pool_addr: String,
    pub token_arr: Vec<String>,
    pub router_pair_addr: String,
}

impl Descriptor for LiquidswapDescriptor {}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum CurveType {
    Uncorrelated,
    Stable
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LiquidswapMetadata {
    pub reserves: Vec<u64>
}
impl PairMetadata for LiquidswapMetadata {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LiquidswapPair {
    pub network: String,
    pub protocol: String,
    pub pair_name: PairNames,
    pub pair_key: String,
    pub pool_addr: String,
    pub token_arr: Vec<String>,
    pub router_pair_addr: String,
    pub curve_type: CurveType,
    pub x_scale: u64,
    pub y_scale: u64,
    pub fee: u64,
    pub dao_fee: u64,
    pub metadata: LiquidswapMetadata
}

const FEE_SCALE: u64 = 10000;
const ONE_E_8: u64 = 100000000;

fn stable_d(x0_u256: U256, y_u256: U256) -> U256 {
    let three_u256 = U256::from(3);

    // 3 * x0 * (y * y / 1e8) / 1e8
    let xyy3 = three_u256 * x0_u256 * y_u256 * y_u256;
    let xxx = x0_u256 * x0_u256 * x0_u256;

    // x0 * x0 / 1e8 * x0 / 1e8
    xyy3 + xxx
}

fn stable_f(x0_u256: U256, y_u256: U256) -> U256 {
    let a = x0_u256 * y_u256 * y_u256 * y_u256;
    let b = y_u256 * x0_u256 * x0_u256 * x0_u256;

    // a + b
    a + b
}

fn stable_lp_value(x_coin: u128, x_scale: u64, y_coin: u128, y_scale: u64) -> U256 {
    let one_e_8 = U256::from(ONE_E_8);
    let x_u256 = U256::from(x_coin);
    let y_u256 = U256::from(y_coin);

    let x_scale_u256 = U256::from(x_scale);
    let y_scale_u256 = U256::from(y_scale);

    let _x =  (x_u256 * one_e_8) / x_scale_u256;
    let _y = (y_u256 * one_e_8) / y_scale_u256;

    let _a = _x * _y;

    let _b = (_x * _x) + (_y * _y);
    _a * _b
}

fn stable_get_y(x0: U256, xy: U256, mut y: U256) -> U256 {
    let mut i = 0;

    while (i < 255) {
        let k = stable_f(x0, y);

        let mut _dy = U256::from(0);
        if (k < xy) {
            _dy = ((xy - k) / stable_d(x0, y)) + 1;
            y = y + _dy;
        } else {
            _dy = ((k - xy) / stable_d(x0, y));
            y = y - _dy;
        };

        if (_dy <= U256::from(1)) {
            return y
        };

        i = i + 1;
    };

    y
}

fn stable_curve_coin_out(coin_in: u128, scale_in: u64, scale_out: u64, reserve_in: u128, reserve_out: u128) -> u128 {
    let one_e_8 = U256::from(ONE_E_8);
    let xy = stable_lp_value(reserve_in, scale_in, reserve_out, scale_out);

    let scale_in_u256 = U256::from(scale_in);
    let scale_out_u256 = U256::from(scale_out);

    let reserve_in_u256 = (U256::from(reserve_in) * one_e_8) / scale_in_u256;
    let reserve_out_u256 = (U256::from(reserve_out) * one_e_8) / scale_out_u256;

    let amount_in = (U256::from(coin_in) * one_e_8) / scale_in_u256;
    let total_reserve = amount_in + reserve_in_u256;
    let y = reserve_out_u256 - stable_get_y(total_reserve, xy, reserve_out_u256);
    let r = (y * scale_out_u256) / one_e_8;

    r.as_u128()
}

fn mul_div_u128(x: u128, y: u128, z: u128) -> u64 {
    if(z==0) {
        panic!("Divide by zero");
    }
    let r =(x * y) / z;
    (r as u64)
}

impl Pair for LiquidswapPair {
    fn output_amount(&self, input_amount: u64, token_in: &String, token_out: &String) -> u64 {
        let fee_pct = self.fee;
        let fee_multiplier = FEE_SCALE - fee_pct;
        let mut reserve_in_u128;
        let mut reserve_out_u128;
        let mut scale_in;
        let mut scale_out;

        if token_in == &self.token_arr[0] && token_out == &self.token_arr[1] {
            reserve_in_u128 = (self.metadata.reserves[0] as u128);
            reserve_out_u128 = (self.metadata.reserves[1] as u128);
            scale_in = self.x_scale;
            scale_out = self.y_scale;
        } else if token_in == &self.token_arr[1] && token_out == &self.token_arr[0]{
            reserve_in_u128 = (self.metadata.reserves[1] as u128);
            reserve_out_u128 = (self.metadata.reserves[0] as u128);
            scale_in = self.y_scale;
            scale_out = self.x_scale;
        }
        else {
           return 0;
        }

        if (self.curve_type == CurveType::Stable) {
            let coin_in_val_scaled = (input_amount as u128) * (fee_multiplier as u128);
            let coin_in_val_after_fees = if (coin_in_val_scaled % (FEE_SCALE as u128) != 0) {
                (coin_in_val_scaled / (FEE_SCALE as u128)) + 1
            } else {
                coin_in_val_scaled / (FEE_SCALE as u128)
            };
    
            return (stable_curve_coin_out(
                coin_in_val_after_fees,
                scale_in,
                scale_out,
                reserve_in_u128,
                reserve_out_u128
            ) as u64);
        }
        else if (self.curve_type == CurveType::Uncorrelated) {
            let coin_in_val_after_fees = (input_amount as u128) * (fee_multiplier as u128);
            let new_reserve_in = (reserve_in_u128 * (FEE_SCALE as u128)) + coin_in_val_after_fees;
            return mul_div_u128(coin_in_val_after_fees,
                reserve_out_u128,
                new_reserve_in)
        }
        else {
            panic!("Curve type not supported");
        }
    }

    fn get_descriptor(&self) -> Box<dyn Descriptor> {
        return Box::new(
            LiquidswapDescriptor {
                network: self.network.clone(),
                protocol: self.protocol.clone(),
                pair_name: self.pair_name.clone(),
                pool_addr: self.pool_addr.clone(),
                token_arr: self.token_arr.clone(),
                router_pair_addr: self.router_pair_addr.clone()

            }
        )
    }

    fn get_protocol(&self) -> &str {
        return &self.protocol;
    }

    fn get_token_arr(&self) -> &Vec<String> {
        return &self.token_arr;
    }

    fn get_pair_key(&self) -> &str {
        return &self.pair_key;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}