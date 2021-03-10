// this enum contain IpAddress-Classes variants.
//
// Variants:=> ClassA, ClassB, ClassC, ClassD, Un_valid.
//
// type-String
#[derive(PartialEq, Eq, Debug)]
pub enum IpAddressClasses {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    UnValid,
}
//
// function to match input IpAddress to particular class variants
//
// #Arguments
//
// input_ip -> u32,u32,u32,u32 which are taking input of ip
//
// Return
//
// Returning Ip Address according to condition

pub fn match_input_ip(a: u32, b: u32, c: u32, d: u32) -> IpAddressClasses {
    let input_ip = (a, b, c, d);
    // this is taking 4 u32 input and matching them to particular class
    match input_ip {
        //
        // it lies in Class A if lies between 0 to 127.
        //
        (a, _, _, _) if (0..=127).contains(&a) => IpAddressClasses::ClassA(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        )),
        //
        // it lies in Class B if lies between 128 to 190.
        //
        (a, _, _, _) if (128..=191).contains(&a) => IpAddressClasses::ClassB(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        )),
        //
        // it lies in Class C if lies between 192 to 223.
        //
        (a, _, _, _) if (192..=223).contains(&a) => IpAddressClasses::ClassC(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        )),
        //
        // it lies in Class D if lies between 224 to 239.
        //
        (a, _, _, _) if (224..=239).contains(&a) => IpAddressClasses::ClassD(format!(
            "{}.{}.{}.{}",
            input_ip.0, input_ip.1, input_ip.2, input_ip.3
        )),
        //
        // Rest cases are Un-Valid in this match condition.
        //
        _ => IpAddressClasses::UnValid,
    }
}
