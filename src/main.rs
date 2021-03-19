use rand;
// use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use std::str;
use rand_chacha;
use std::str::FromStr;
use rand::prelude::*;
use rand::prng::chacha::ChaChaRng;


use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
extern crate pem;
use std::io::{self, Write, BufRead};
use pem::parse;
use rand::{thread_rng, Rng};
// use rand::prng;
static GROUP_SIZE: u64 = 65536;
static GROUP_SIZE_SIGNED: i64 = 65536;

static SEC_PARAM: i64 = 8;
static SEC_PARAM_SIZE: usize = 8;

fn main() {
    println!("CASE ALPHA = X");

    let result = gen(SEC_PARAM_SIZE, 57, 1);
    println!("KEY 0 {:?}", result[0]);
    println!("KEY 1 {:?}", result[1]);
    let k_0 = &result[0];
    let k_1 = &result[1];
    println!("EVAL");

    let check_0 = eval(0, k_0.to_string(), 57, SEC_PARAM_SIZE);
    let check_1 = eval(1, k_1.to_string(), 57, SEC_PARAM_SIZE);
    println!("EXPECTED: s||1; ACTUAL: {:?}",  format!("{:b}", check_0 ^ check_1));


    let result = gen(SEC_PARAM_SIZE, 57, 1);
    println!("KEY 0 {:?}", result[0]);
    println!("KEY 1 {:?}", result[1]);
    let k_0 = &result[0];
    let k_1 = &result[1];
    println!("EVAL");

    println!("CASE ALPHA =/= X");
    let check_0 = eval(0, k_0.to_string(), 47, SEC_PARAM_SIZE);
    let check_1 = eval(1, k_1.to_string(), 47, SEC_PARAM_SIZE);
    println!("EXPECTED: 0; ACTUAL: {:?}",  format!("{:b}", check_0 ^ check_1));

}


fn eval(b: u32, k_b: String, x:i64, sec_param:usize)  -> i64{
    let t_0 = b;
    let mut t_prev = t_0;
    let x_vec: Vec<char> = format!("{:b}", x).chars().collect();
    let n = x_vec.len();
    // println!("EVAL N {:?}", n);
    
    println!("X VEC{:?}", format!("{:b}", x));

    //parse k_b
    let mut cw_vec: Vec<String> = Vec::new();
    let s_0 =  u32::from_str_radix(&k_b[0..sec_param], 2).unwrap();
    let mut end = sec_param;
    // println!("K B {:?}", k_b);
  
    for i in 0..n{
        let cw_i = &k_b[end..end + sec_param + 2];
        println!("CW {:?} ITER {:?}", cw_i, i);

        end = end + sec_param + 2;
        cw_vec.push(cw_i.to_string());
    }
    let cw_end =  i64::from_str_radix(&k_b[end.. k_b.len()], 2).unwrap();
    let mut s_prev = s_0;
    for i in 0..n{
        //parse correction word i
        let s_cw = &cw_vec[i][0.. sec_param];
        let t_l_cw = &cw_vec[i][sec_param.. sec_param + 1];
        let t_r_cw = &cw_vec[i][sec_param + 1.. sec_param + 2];
        let mut ra = ChaChaRng::new_unseeded();

        ra.set_stream(s_prev.into());
        let gen = ra.next_u32();
        let parse_concat = u32::from_str_radix(&format!("{}{}{}{}", s_cw, t_l_cw, s_cw, t_r_cw), 2).unwrap();
        let tau_i = gen ^ (t_prev * parse_concat);
        println!("RUN {:?}", i);

        //parse tau
        let tau_parse_str = format!("{:b}", tau_i);
        println!("TAU {:?}", tau_parse_str);

        let s_l = u32::from_str_radix(&tau_parse_str[0..sec_param], 2).unwrap();
        let t_l = u32::from_str_radix(&tau_parse_str[sec_param..sec_param+1], 2).unwrap();
        let s_r =  u32::from_str_radix(&tau_parse_str[sec_param+1..2*sec_param + 1], 2).unwrap();
        let t_r = u32::from_str_radix(&tau_parse_str[2*sec_param+1..2*sec_param+2], 2).unwrap();//yes
        println!("EVAL S0L {:?}, T0L {:?}, S0R {:?}, T0R {:?}", format!("{:b}",s_l), format!("{:b}",t_l), format!("{:b}",s_r), format!("{:b}",t_r));
        println!("EVAL S1L {:?}, T1L {:?}, S1R {:?}, T1R {:?}", format!("{:b}",s_l), format!("{:b}",t_l), format!("{:b}",s_r), format!("{:b}",t_r));

        if x_vec[i] == '0'{
            s_prev = s_l;
            t_prev = t_l;

        }else{
            s_prev = s_r;
            t_prev = t_r; 
        }
        println!("S PREV {:?}", format!("{:b}", s_prev));
        println!("T PREV {:?}", t_prev);

        println!("___________________________");

    }
    let convert_ = convert(s_prev.into());
    let t:i64 =  t_prev.into();
    let mut return_ = i64::pow(-1, b) * (convert_ + (t * cw_end)) % GROUP_SIZE_SIGNED;
    if return_ < 0{
        return_ += GROUP_SIZE_SIGNED;
    }
    return return_;
}


fn convert(s: u64) -> i64{
    let mut prng = ChaChaRng::new_unseeded();
    prng.set_stream(s);
    let gen_s:u64 = prng.next_u32().into();
    let return_;
    let result = power_of_two(GROUP_SIZE);
 
    if result != -1{
        let m = result;
        let mut bitmask:u64 = 1;
        //create bitmask for truncating s
        for _ in 1..m{
            bitmask = (bitmask << 1) | 1;
        }
        if m <= SEC_PARAM{   
            return_ = (bitmask & s) % GROUP_SIZE;
        }else{
            return_ = (bitmask & gen_s) % GROUP_SIZE;
        }
    }else{
        return_ = gen_s % GROUP_SIZE;
    }
    let signed = return_ as i64;
    return signed;
}


fn power_of_two(i: u64) -> i64{
    let mut check = i;
    let mut end = false;
    let mut m = 0;
    while !end{
        if check == 0{
            m = -1;
            end = true;
        }else if check == 1{
            end = true;
        }else{
            if check & 1 == 1{
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

fn pad_bits(s:String, len:usize) -> String{
    let padded;
    if  s.len() < len{
        let extension = String::from_utf8(vec![b'0'; len - s.len()]).unwrap();
        padded = format!("{}{}", extension, s);
    }else{
        padded = format!("{}", s);
    }
    return padded;
}


fn gen(sec_param: usize , alpha:i64, beta:i64) -> Vec<String>{

    //bit decomposition of alpha
    let alpha_vec: Vec<char> = format!("{:b}", alpha).chars().collect();
    let n = alpha_vec.len();
    println!("N LENGTH {:?}", n);
    println!("ALPHA VEC {:?}", alpha_vec);

    //randomly sample s_0 and s_1
    let mut rng = rand::thread_rng();
    let mut s_0:u32 = 0;
    let mut s_1:u32 = 0;
    for _ in 0..sec_param{
        s_0 = (s_0 << 1)  | rng.gen_range(0,2);
        s_1 = (s_1 << 1)  | rng.gen_range(0,2);

    }
    let mut s0_prev = s_0;
    let mut s1_prev = s_1;

    //begin building k
    let mut k_0 = pad_bits(format!("{:b}", s_0), sec_param);
    let mut k_1 = pad_bits(format!("{:b}", s_1), sec_param);//correct length
    println!("K_0 init {:?}", k_0);
    println!("K_1 init {:?}", k_1);


    //initialize t-bits
    let mut t0_prev:u32 = 0;
    let mut t1_prev:u32 = 1;
    println!("___________________________");

    for i in 0..n{

        //generate based on previous s0
        let mut prng_0 = ChaChaRng::new_unseeded();
        prng_0.set_stream(s0_prev.into());
        let gen_s0 = prng_0.next_u32();
        println!("RUN {:?}", i);
        println!("S0 PREV {:?}", format!("{:b}",s0_prev));
        println!("S1 PREV {:?}", format!("{:b}",s1_prev));
        println!("T0 PREV {:?}", format!("{:b}", t0_prev));
        println!("T1 PREV {:?}", format!("{:b}", t1_prev));

        println!("GEN S_0 {:?}", format!("{:b}",gen_s0));
     
        //parse prng output
        let gen_s0_bits = format!("{:b}", gen_s0);
        let s0_l = u32::from_str_radix(&gen_s0_bits[0..sec_param], 2).unwrap();
        let t0_l = u32::from_str_radix(&gen_s0_bits[sec_param..sec_param+1], 2).unwrap();
        let s0_r =  u32::from_str_radix(&gen_s0_bits[sec_param+1..2*sec_param + 1], 2).unwrap();
        let t0_r = u32::from_str_radix(&gen_s0_bits[2*sec_param+1..2*sec_param+2], 2).unwrap();//yes
        println!("GEN S0L {:?}, T0L {:?}, S0R {:?}, T0R {:?}", format!("{:b}",s0_l), format!("{:b}",t0_l), format!("{:b}",s0_r), format!("{:b}",t0_r));


        //default case alpha_i = 0
        let mut alpha_i = 0;
        let mut s0_keep = s0_l; 
        let mut s0_lose = s0_r; 
        let mut t0_keep = t0_l; 
        let mut t0_lose = t0_r;
        let mut keep = 'l';
        //case alpha_i = 1
        if alpha_vec[i] == '1'{
            s0_keep = s0_r;
            t0_keep = t0_r;
            s0_lose = s0_l;
            t0_lose = t0_l;
            alpha_i = 1;
            keep = 'r';
        }
        println!("KEEP {:?} S0 KEEP {:?}, T0 KEEP {:?}, S0 LOSE {:?}, T0 LOSE {:?}", keep, format!("{:b}", s0_keep), format!("{:b}",t0_keep), format!("{:b}",s0_lose), format!("{:b}",t0_lose));

        //generate based on previous s1
        let mut prng_1 = ChaChaRng::new_unseeded();
        prng_1.set_stream(s1_prev.into());
        let mut gen_s1 = prng_1.next_u32();
        println!("GEN S_1 {:?}", format!("{:b}",gen_s1));

        //parse prng output
        let gen_s1_bits = format!("{:b}", gen_s1);
        let s1_l = u32::from_str_radix(&gen_s1_bits[0..sec_param], 2).unwrap();
        let t1_l = u32::from_str_radix(&gen_s1_bits[sec_param..sec_param+1], 2).unwrap();
        let s1_r =  u32::from_str_radix(&gen_s1_bits[sec_param+1..2*sec_param + 1], 2).unwrap();
        let t1_r = u32::from_str_radix(&gen_s1_bits[2*sec_param+1..2*sec_param+2], 2).unwrap();//yes
        println!("GEN S1L {:?}, T1L {:?}, S1R {:?}, T1R {:?}", format!("{:b}",s1_l), format!("{:b}",t1_l), format!("{:b}",s1_r), format!("{:b}",t1_r));

        //default case alpha_i = 0
        let mut s1_keep = s1_l; 
        let mut s1_lose = s1_r; 
        let mut t1_keep = t1_l; 
        let mut t1_lose = t1_r;
        //case alpha_i = 1
        if alpha_vec[i] == '1'{
            s1_keep = s1_r;
            t1_keep = t1_r;
            s1_lose = s1_l;
            t1_lose = t1_l;
        }
        println!("S1 KEEP {:?}, T1 KEEP {:?}, S1 LOSE {:?}, T1 LOSE {:?}", format!("{:b}", s1_keep), format!("{:b}",t1_keep), format!("{:b}",s1_lose), format!("{:b}",t1_lose));

        let s_cw = s0_lose ^ s1_lose;
        println!("S CW {:?}", format!("{:b}",s_cw));


        let t_cw_l = t0_l ^ t1_l ^ alpha_i ^ 1;
        let t_cw_r = t0_r ^ t1_r ^ alpha_i;
        println!("T CW L{:?}", format!("{:b}",t_cw_l));
        println!("T CW R{:?}", format!("{:b}",t_cw_r));

        let cw_i = format!("{}{}{}", pad_bits(format!("{:b}", s_cw), sec_param), format!("{:b}", t_cw_l), format!("{:b}", t_cw_r));
         println!("CW N {:?} ITER {:?}", cw_i, i);
         println!("S KEEP 0 {:?}", format!("{:b}",s0_keep));
         println!("S KEEP 1 {:?}", format!("{:b}",s1_keep));
 
        let s0_i = s0_keep ^ (t0_prev * s_cw);
        let s1_i = s1_keep ^ (t1_prev * s_cw);
        println!("S 0i {:?}", format!("{:b}",s0_i));
        println!("S 1i {:?}", format!("{:b}",s1_i));

        let t0_i;
        let t1_i;

        if keep == 'l'{
            t0_i = t0_keep ^ (t0_prev * t_cw_l);
            t1_i = t1_keep ^ (t1_prev * t_cw_l);

        }else{
            t0_i = t0_keep ^ (t0_prev * t_cw_r);
            t1_i = t1_keep ^ (t1_prev * t_cw_r);
        }
        println!("T 0i {:?}", format!("{:b}",t0_i));
        println!("T 1i {:?}", format!("{:b}",t1_i));

        t0_prev = t0_i;
        t1_prev = t1_i;

        s0_prev = s0_i;
        s1_prev = s1_i;

        //build up key from correction words
        k_0 = format!("{}{}", k_0, cw_i);
        k_1 = format!("{}{}", k_1, cw_i);
        println!("___________________________");
    }

    //create and append last correction word
    let convert_0 = convert(s0_prev.into());
    let convert_1 = convert(s1_prev.into());

    let mut cw_end = i64::pow(-1, t1_prev) * (beta - convert_0 + convert_1) % GROUP_SIZE_SIGNED; //issue here
    if cw_end < 0{
        cw_end += GROUP_SIZE_SIGNED;
    }
    k_0 = format!("{}{:b}", k_0, cw_end);
    k_1 = format!("{}{:b}", k_1, cw_end);

    return vec![k_0, k_1];
}