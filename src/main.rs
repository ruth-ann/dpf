use rand;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use std::str;
use rand_chacha;
// use rand_chacha::ChaChaRng;
use std::str::FromStr;
use rand::prelude::*;
// use chacha20::{ChaCha20, Key, Nonce};
// use chacha20::cipher::{NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek}; https://docs.rs/chacha20/0.6.0/chacha20/
// use   rand::CryptoRng;
use rand::prng::chacha::ChaChaRng;
// use rand::prng;
static GROUP_SIZE: u64 = 64;
static SEC_PARAM: i64 = 4;

fn main() {
    let result = gen(3, 65, 65);
    println!("{:?}", result[0]);
    println!("{:?}", result[1]);
    let k = &result[0];
    let check = eval(0, k.to_string(), 4, 3, 3);

}


fn eval(b: u32, k_b: String, x:u32, sec_param:usize, n:usize)  -> i64{
    let t_0 = b;
    let mut t_prev = t_0;
    let mut ra = ChaChaRng::new_unseeded();
    let p = 67;
    let mut cw_vec: Vec<String> = Vec::new();
    let s_0 =  u32::from_str_radix(&k_b[0..sec_param], 2).unwrap();
    let mut end = sec_param;
    for i in 1..n{
        let cw_i = &k_b[end..end + sec_param + 2];
        end = end + sec_param + 2;
        cw_vec.push(cw_i.to_string());
    }
    // let cw_end = 56;
    println!("{:?} end {:?} len {:?}", k_b, end, k_b.len());// TODO - how is this happening

    let cw_end =  i64::from_str_radix(&k_b[end..std::cmp::min(k_b.len(), 63)], 2).unwrap();
    let mut s_prev = s_0;
    for i in 1..n{
        let s_cw = &cw_vec[i - 1][0.. sec_param];
        let t_l_cw = &cw_vec[i - 1][sec_param.. sec_param + 1];
        let t_r_cw = &cw_vec[i - 1][sec_param + 1.. sec_param + 2];
        ra.set_stream(s_prev.into());
        let gen = ra.next_u32();
        let check = format!("{}{}{}{}", s_cw, t_l_cw, s_cw, t_r_cw);
        let parse_concat = u32::from_str_radix(&format!("{}{}{}{}", s_cw, t_l_cw, s_cw, t_r_cw), 2).unwrap();
        let tau_i = gen ^ (t_prev * parse_concat);
        let tau_parse_str = format!("{:b}", tau_i);
        let s_l = u32::from_str_radix(&tau_parse_str[0..sec_param], 2).unwrap();
        let t_l = u32::from_str_radix(&tau_parse_str[sec_param..sec_param+1], 2).unwrap();
        let s_r =  u32::from_str_radix(&tau_parse_str[sec_param+1..2*sec_param + 1], 2).unwrap();
        let t_r = u32::from_str_radix(&tau_parse_str[2*sec_param+1..2*sec_param+2], 2).unwrap();//yes
        if 1 == 1{
            s_prev = s_l;
            t_prev = t_l;

        }else{
            s_prev = s_r;
            t_prev = t_r; 
        }
    }
    let convert_ = convert(s_prev.into());
//format!("{}{}{}{}", s_cw, t_l_cw, s_cw, t_r_cw), 2
//let cw_end = i64::pow(-1, t1_prev) * (b - convert_0 + convert_1) % p; 
//
let return_ = i64::pow(-1, b) * (convert_ + 1 + cw_end) % p;  //to do - fix
println!("{:?}", return_);
return return_;


}
// fn main() {
fn convert(s: u64) -> i64{
    //convert
    let secpar = 3;
    let s:u64 = 238767;
    let mut ra = ChaChaRng::new_unseeded();
    let return_;
    let mut rng = rand::thread_rng();
    let v = rng.gen_range(0,GROUP_SIZE);
    let m = power_of_two(v);
    // println!("{:?}", m);

    // println!("{:?}", v);
    let mut bitmask:u64 = 1;
    for _ in 1..m{
        bitmask = (bitmask << 1) | 1;
    }
    if m != -1{
        if m <= SEC_PARAM{
         
            return_ = (bitmask & s) % GROUP_SIZE;//seens to eork

    // let rng = rand_chacha::ChaCha8Rng::from_seed(seed);
    // let mut prng = rand_chacha::ChaCha8Rng::from_seed(s);

        }else{
            ra.set_stream(s);
            let mut gen_s:u64 = ra.next_u32().into(); //may not be right
            return_ = (bitmask & gen_s) % GROUP_SIZE;
        }
    }else{
        ra.set_stream(s);
        let mut gen_s:u64 = ra.next_u32().into(); //may not be right
        return_ = gen_s % GROUP_SIZE;
    }

    // if (x != 0) && ((x & (x - 1)) == 0);
    let signed = return_ as i64;
    return signed;



}
// pub fn decrypt_share(share: &[u8], key: &PrivateKey) -> Result<Vec<u8>, EncryptError> {


fn power_of_two(i: u64) -> i64{
    let mut check = i;
    let mut end = false;
    let mut m = 0;
    let mut pow_2 = true;
    while !end{
        if check == 0{
            pow_2 = false;
            m = -1;
            end = true;
        }else if check == 1{
            end = true;
        }else{
            if check & 1 == 1{
                pow_2 = false;
                m = -1;

                end = true;
            }else{
                check = check >> 1;
                m += 1;
            }
        }
    } 
    return m;

}
//https://docs.rs/itertools/0.7.8/itertools/structs/struct.Groups.html


fn gen(sec_param: usize , a:u32, b:i64) -> Vec<String>{
    let a = 8;
    let b:i64 = 67;
    let p = 88;

    //bit decomposition of alpha
    let a_vec: Vec<char> = format!("{:b}", a).chars().collect();
    //length
    let n = a_vec.len();

    let mut rng = rand::thread_rng();
    let g = sec_param;

    //randomly sample s_0 and s_1
    let mut s_0:u32 = 0;
    let mut s_1:u32 = 0;
 
    let mut ra = ChaChaRng::new_unseeded();



    for _ in 0..g{
        s_0 = (s_0 << 1)  | rng.gen_range(0,2);
        s_1 = (s_1 << 1)  | rng.gen_range(0,2);

    }
    let cw = "";
    let mut k_0 = format!("{:b}", s_0);
    let mut k_1 = format!("{:b}", s_1);
    let mut t0_prev:u32 = 0;
    let mut t1_prev:u32 = 1;
    let mut s0_prev = s_0;
    let mut s1_prev = s_1;

    for i in 1..n{
        ra.set_stream(s0_prev.into());
        let mut gen_s_0 = ra.next_u32();
        //case alpha_i = 0
        let mut alpha_i = 0;
        let exp_0 = format!("{:b}", gen_s_0);
        let s_l_0 = u32::from_str_radix(&exp_0[0..g], 2).unwrap();
        let t_l_0 = u32::from_str_radix(&exp_0[g..g+1], 2).unwrap();
        let s_r_0 =  u32::from_str_radix(&exp_0[g+1..2*g + 1], 2).unwrap();
        let t_r_0 = u32::from_str_radix(&exp_0[2*g+1..2*g+2], 2).unwrap();//yes

        let mut s_keep_0 = s_l_0; 
        let mut s_lose_0 = s_r_0; 
        let mut t_keep_0 = t_l_0; 
        let mut t_lose_0 = t_r_0;
        let mut keep = 'l';
        let mut lose = 'r';
        if a_vec[i] == '1'{
            s_keep_0 = s_r_0;
            t_keep_0 = t_r_0;
            s_lose_0 = s_l_0;
            t_lose_0 = t_l_0;
            alpha_i = 1;
            keep = 'r';
            lose = 'l';
        }



        ra.set_stream(s1_prev.into());
        let mut gen_s_1 = ra.next_u32();
        let exp_1 = format!("{:b}", gen_s_1);
        let s_l_1 = u32::from_str_radix(&exp_1[0..g], 2).unwrap();
        // println!("{}", intval);
        let t_l_1 = u32::from_str_radix(&exp_1[g..g+1], 2).unwrap();
        let s_r_1 =  u32::from_str_radix(&exp_1[g+1..2*g + 1], 2).unwrap();
        let t_r_1 = u32::from_str_radix(&exp_1[2*g+1..2*g+2], 2).unwrap();//yes

        let mut s_keep_1 = s_l_1; 
        let mut s_lose_1 = s_r_1; 
        let mut t_keep_1 = t_l_1; 
        let mut t_lose_1 = t_r_1;
        if a_vec[i] == '1'{
            s_keep_1 = s_r_1;
            t_keep_1 = t_r_1;
            s_lose_1 = s_l_1;
            t_lose_1 = t_l_1;
        }


        let s_cw = s_lose_0 ^ s_lose_1;
        let t_l_cw = t_l_0 ^ t_l_1 ^ alpha_i ^ 1;
        let t_r_cw = t_r_0 ^ t_r_1 ^ alpha_i;

        let cw_i = format!("{}{}{}", format!("{:b}", s_cw), format!("{:b}", t_l_cw), format!("{:b}", t_r_cw));
        // print!("{:?}", cw_i);

        let s_i_0 = (s_keep_0 ^ t0_prev) * s_cw;
        let s_i_1 = (s_keep_1 ^ t1_prev) * s_cw;

        let t_i_0;
        let t_i_1;

        if keep == 'l'{
            t_i_0 = (t_keep_0 ^ t0_prev) * t_l_cw;
            t_i_1 = (t_keep_1 ^ t1_prev) * t_l_cw;

        }else{
            t_i_0 = (t_keep_0 ^ t0_prev) * t_r_cw;
            t_i_1 = (t_keep_1 ^ t1_prev) * t_r_cw;
        }
       
        t0_prev = t_i_0;
        t1_prev = t_i_1;

        s0_prev = s_i_0;
        s1_prev = s_i_1;
        //line 12

        // let my_int = from_str::<int>(my_str);

        //line 5
        //Generate using s_0 and s_1 as seeds -- how to split
        //let mut prng = rand_chacha::ChaChaRng::from_seed(s);  - first gamma - get results from prng and chop up    

        //operations on group elements
        //group element represented by

        //concatenation?

        k_0 = format!("{}{}", k_0, cw_i);
        k_1 = format!("{}{}", k_1, cw_i);


}

// println!("{:?}",k_0);
// println!("{:?}",k_1);
let convert_0 = convert(s0_prev.into());
let convert_1 = convert(s1_prev.into());

let cw_end = i64::pow(-1, t1_prev) * (b - convert_0 + convert_1) % p; 

k_0 = format!("{}{:b}", k_0, cw_end);
k_1 = format!("{}{:b}", k_1, cw_end);

// println!("{:?}",k_0);
// println!("{:?}",k_1);
// let n:u64 = 2;
// let mut rng = rand::thread_rng();

// let mut ra = ChaChaRng::new_unseeded();
// ra.set_stream(n);
// println!("{:?}", ra.next_u32());
// println!("{:?}", ra.next_u32());
// let mut rng =  ChaChaRng::get_word_pos(1);
// let mut ra = ChaCha20Rng::set_stream(n);

// let key = Key::from_slice(b"an example very very secret key.");
// let nonce = Nonce::from_slice(b"secret nonce");
// let mut cipher = ChaCha20::new(&key, &nonce);
    return vec![k_0, k_1];

}