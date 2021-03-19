
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use prio::client;
use prio::server;
use prio::encrypt;
// use openssl::rsa::Rsa;
use prio::finite_field::*;
extern crate pem;
use std::io::{self, Write, BufRead};
use pem::parse;
use rand::{thread_rng, Rng};

fn main() {

    let dimension = 100;
    let num_clients = 60;
    let period = 10;
    
    //create private keys to be used by the server
    let priv_key1 = encrypt::PrivateKey::from_base64(
        "BNNOqoU54GPo+1gTPv+hCgA9U2ZCKd76yOMrWa1xTWgeb4LhFLMQIQoRwDVaW64g\
        /WTdcxT4rDULoycUNFB60LER6hPEHg/ObBnRPV1rwS3nj9Bj0tbjVPPyL9p8QW8B+w==", //TODO- generate different private keys
    ).unwrap();

    let priv_key2 = encrypt::PrivateKey::from_base64(
        "BIl6j+J6dYttxALdjISDv6ZI4/VWVEhUzaS05LgrsfswmbLOgNt9HUC2E0w+9Rq\
        Zx3XMkdEHBHfNuCSMpOwofVSq3TfyKwn0NrftKisKKVSaTOt5seJ67P5QL4hxgPWvxw==",
    ).unwrap();

    let mut server1 = server::Server::new(dimension, true, priv_key1);
    let mut server2 = server::Server::new(dimension, false, priv_key2);
    
    for _ in 0..period {
        for _ in 0..num_clients {
            let priv_key1 = encrypt::PrivateKey::from_base64(
                "BNNOqoU54GPo+1gTPv+hCgA9U2ZCKd76yOMrWa1xTWgeb4LhFLMQIQoRwDVaW64g\
                /WTdcxT4rDULoycUNFB60LER6hPEHg/ObBnRPV1rwS3nj9Bj0tbjVPPyL9p8QW8B+w==",
            ).unwrap();
        
            let priv_key2 = encrypt::PrivateKey::from_base64(
                "BIl6j+J6dYttxALdjISDv6ZI4/VWVEhUzaS05LgrsfswmbLOgNt9HUC2E0w+9Rq\
                Zx3XMkdEHBHfNuCSMpOwofVSq3TfyKwn0NrftKisKKVSaTOt5seJ67P5QL4hxgPWvxw==",
            ).unwrap();
        
            let pub_key1 = encrypt::PublicKey::from(&priv_key1);
            let pub_key2 = encrypt::PublicKey::from(&priv_key2);

            let mut client = client::Client::new(dimension, pub_key1, pub_key2).unwrap();


            // random one-hot vector 
            let mut rng = rand::thread_rng();
            let mut data_u32 = vec![0u32; dimension];
            let pos = rng.gen_range(0..dimension);
            data_u32[pos] = 1u32;
            let data = data_u32
                .iter()
                .map(|x| Field::from(*x))
                .collect::<Vec<Field>>();
        
            let (share1, share2) = client.encode_simple(&data).unwrap();
            let eval_at = server1.choose_eval_at();
            let v1 = server1
                .generate_verification_message(eval_at, &share1)
                .unwrap();
            let v2 = server2
                .generate_verification_message(eval_at, &share2)
                .unwrap();
                assert_eq!(server1.aggregate(&share1, &v1, &v2).unwrap(), true);
                assert_eq!(server2.aggregate(&share2, &v1, &v2).unwrap(), true);

        }
    }
    let total1 = server1.total_shares();
    let total2 = server2.total_shares();

    let reconstructed = prio::util::reconstruct_shares(total1, total2).unwrap();
    print!("{:?} reconstruscted", reconstructed);
}