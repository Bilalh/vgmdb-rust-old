
extern crate hyper;


use std::io::Read;

use self::hyper::Client;
use self::hyper::Url;
use self::hyper::header::Connection;
use self::hyper::header::ConnectionOption;

pub fn bar() {

    let url_str = "http://vgmdb.info/album";
    let mut url = Url::parse(url_str).unwrap();

    let url_params = Some ( [ ("format","json")  ]  );

     // match url_params {
     //    Some(params) => {
     //        url.set_query_from_pairs(params.to_vec().into_iter());
     //    },
     //    None => ()
    // };

    let mut client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]))
        // let 'er go!
        .send().unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
