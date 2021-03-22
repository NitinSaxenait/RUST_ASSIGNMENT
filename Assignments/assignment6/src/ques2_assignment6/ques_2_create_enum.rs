use log::*;
#[derive(PartialEq, Eq, Debug)]
/// IpAddressClasses an enum having 6 variants of classes of ip address and None.
pub enum IpAddressClasses {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}
/// match_input_ip is matching input ip address to the corresponding class or None if ip is invalid.
///
/// #Arguments
///
///  a,b,c,d all u32 types as ip address which are to be match with correspond class.
///
/// #Return
///
/// Function is returning a IpAddressClasses as particular class in which address exist.
pub fn match_input_ip(a: u32, b: u32, c: u32, d: u32) -> Result<IpAddressClasses, String> {
    let input_ip = (a, b, c, d);

    match input_ip {
        (1..=126, 0..=255, 0..=255, 1..=254) => Ok(IpAddressClasses::ClassA(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        ))),

        (128..=191, 0..=255, 0..=255, 1..=254) => Ok(IpAddressClasses::ClassB(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        ))),

        (192..=223, 0..=255, 1..=254, 1..=254) => Ok(IpAddressClasses::ClassC(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        ))),

        (224..=239, 0..=255, 0..=255, 0..=255) => Ok(IpAddressClasses::ClassD(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        ))),
        (240..=254, 0..=255, 0..=255, 0..=254) => Ok(IpAddressClasses::ClassE(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        ))),

        _ => {
            warn!("You are putting Invalid ip.");
            Err(String::from("Invalid Ip."))
        }
    }
}
