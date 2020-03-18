use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};
use cdrs::cluster::session;
use cdrs::load_balancing::SingleNode;
use cdrs::query::*;
use cdrs::query_values;
use cdrs::frame::*;
use cdrs::types::prelude::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs_helpers_derive::*;
// use cdrs::cluster::session::Session;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Album {
    name: String,
    release_date: String,
}

fn main() {
    let node = NodeTcpConfigBuilder::new("0.0.0.0:9042", NoneAuthenticator{}).build();
    let cluster = ClusterTcpConfig(vec![node]);
    let session = session::new(&cluster, SingleNode::new()).unwrap();

    let keyspace = "CREATE KEYSPACE IF NOT EXISTS awesome WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': 1 };";
    let table = "CREATE TABLE IF NOT EXISTS awesome.sauce (name TEXT, release_date TEXT, PRIMARY KEY(name));";
    let data = "INSERT INTO awesome.sauce (name, release_date) VALUES (?, ?);";
    let get_data = "SELECT * FROM awesome.sauce;";

    let return_of = query_values!("The Return of the Aquabats", "07.26.1996");
    let fury = query_values!("The Fury of the Aquabats!", "10.28.1997");
    let floating_eye = query_values!("The Aquabats vs The Floating Eye of Death!", "10.26.1999");
    let myths = query_values!("Myths, Legends and Other Amazing Adventures, Vol. 2", "05.17.2000");
    let charge = query_values!("Charge!!", "06.07.2005");
    let soup = query_values!("Hi-Five Soup!", "01.18.2011");
    let supershow = query_values!("The Aquabats! Super Show! Television Soundtrack : Volume One", "03.19.2019");

    session.query(keyspace).unwrap();
    session.query(table).unwrap();
    session.query_with_values(data, return_of).unwrap();
    session.query_with_values(data, fury).unwrap();
    session.query_with_values(data, floating_eye).unwrap();
    session.query_with_values(data, myths).unwrap();
    session.query_with_values(data, charge).unwrap();
    session.query_with_values(data, soup).unwrap();
    session.query_with_values(data, supershow).unwrap();

    let printer = session.query(get_data).unwrap().get_body().unwrap().into_rows().unwrap();
    for print in printer {
        let row: Album = Album::try_from_row(print).unwrap();
        println!("Album Name: {} | Release Date: {}", row.name, row.release_date);
    }
}
