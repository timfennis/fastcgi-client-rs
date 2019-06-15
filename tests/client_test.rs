use fastcgi_client::{Address, ClientBuilder, Params};
use std::env::current_dir;
use std::error::Error;
use std::io::{self, Read as _};
use std::sync::Arc;
use std::thread;

#[test]
fn test_client() {
    env_logger::init();

    let mut client = ClientBuilder::new(Address::Tcp("127.0.0.1", 9000)).build().unwrap();

    let script_name = current_dir().unwrap().join("tests").join("php").join("index.php");

    let script_name = script_name.to_str().unwrap();

    let params = Params::with_predefine()
        .set_request_method("GET")
        .set_script_name("/index.php")
        .set_script_filename(script_name)
        .set_request_uri("/index.php")
        .set_document_uri("/index.php")
//        .set_remote_addr("127.0.0.1")
//        .set_remote_port("12345")
//        .set_server_addr("127.0.0.1")
//        .set_server_port("80")
//        .set_server_name("jmjoy-pc")
        .set_content_type("")
        .set_content_length("0");
    let output = client.do_request(&params, &mut io::empty()).unwrap();
    dbg!(&output);
    dbg!(String::from_utf8(output.get_stdout().unwrap_or(Default::default())));
    dbg!(String::from_utf8(output.get_stderr().unwrap_or(Default::default())));

    //    let mut client = Arc::new(client);
    //
    //    let mut childs = vec![];
    //    for _ in 0..3 {
    //        let child = thread::spawn(move || {
    //            let mut buf: &[u8] = &[0; 1024];
    //            let response = client.do_request(&Params::with_predefine(), &mut buf);
    //            dbg!(response);
    //        });
    //        childs.push(child);
    //    }
    //    for child in childs {
    //        child.join().unwrap();
    //    }

    //    let response = ClientBuilder::new(Address::Tcp("127.0.0.1", 9000))
    //        .build()
    //        .unwrap()
    //        .do_request(
    //            Params::with(
    //                "GET",
    //                "/home/jmjoy/workspace/rust/fastcgi-client-rs/tests/php/index.php",
    //                "",
    //                "/index.php",
    //                "/index.php",
    //                "127.0.0.1",
    //                "12345",
    //                "127.0.0.1",
    //                "80",
    //                "jmjoy-PC",
    //                "",
    //                "0",
    //            ),
    //            &mut io::empty(),
    //        );

    //    let _: () = response;
    //    dbg!(&response.map(|buf| String::from_utf8(buf ).unwrap()));

    //    if let Err(ref e) = response {
    //        //        let e: &Fail = e;
    //        dbg!(e.backtrace());
    //
    //        //        if let Some(bt) = e.as_fail().and_then(|cause| cause.backtrace()) {
    //        //            println!("{}", bt)
    //        //        }
    //    }
    //
    //    //    response.map_err(|e| {
    //    ////        dbg!(e.source());
    //    //    }).map(|x| {
    //    //        dbg!(x);
    //    //    });
}
