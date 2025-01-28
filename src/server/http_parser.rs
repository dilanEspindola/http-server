use std::borrow::Cow;

pub fn parser(request: &Cow<'_, str>) {
    // let split_request: Vec<&str> = request.split(' ').collect();

    println!("{}", request);

    // for request_item in &split_request[1..] {
    //     println!("request item {}", request_item);
    // }

    // for request_item in split_request.iter() {
    //     println!("request item {}", request_item);
    // }
}
