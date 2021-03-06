extern crate iron;
extern crate router;
extern crate urlencoded;

#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use std::io::Write;
use std::str::FromStr;
use router::Router;
use urlencoded::UrlEncodedBody;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19)
               , 3 * 11);
}

fn main() {
    // let mut numbers = Vec::new();

    // for arg in std::env::args().skip(1) {
    //     numbers.push(u64::from_str(&arg)
    //                  .expect("error parsing argument"));
    // }

    // if numbers.len() == 0 {
    //     writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
    //     std::process::exit(1);
    // }

    // let mut d = numbers[0];
    // for m in &numbers[1..] {
    //     d = gcd(d, *m)
    // }

    // println("The greatest common divisor of {:?} is {}", numbers, d);
    let mut router = Router::new();

    router.get("/", get_from, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://127.0.0.1:8000...");

    Iron::new(get_from).http("127.0.0.1:8000").unwrap();
}

fn post_gcd(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match _request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' paramater\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' parameter ot a number: {:?}\n", unparsed)
                );
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d)
    );
    
    Ok(response)
}

fn get_from(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));

    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type="submit">Compute GCD</button>
        </form>
    "#);

    Ok(response)
}
