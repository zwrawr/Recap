extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;

fn main() {
    primes();
}

fn primes() {

    use num_bigint::BigUint;
    use num_traits::cast::FromPrimitive;
    use std::ops::Add;
    use std::ops::Div;
    use std::cmp::Ordering;
    use num_integer::Integer;

    let mut primes:Vec<BigUint> = Vec::new();

    primes.push(BigUint::from_u64(2).unwrap());
    println!("{} is a prime",2);

    primes.push(BigUint::from_u64(3).unwrap());
    println!("{} is a prime",3);

    primes.push(BigUint::from_u64(5).unwrap());
    println!("{} is a prime",5);


    let mut i:BigUint = BigUint::from_u64(5).unwrap();

    loop{

        let mut j:BigUint = BigUint::from_u64(3).unwrap();
        while j.cmp(&i.clone().div(BigUint::from_u64(2).unwrap())) == Ordering::Less {
            if i.clone().mod_floor(&j).eq(&BigUint::from_u64(0).unwrap()) {
                break;
            }
            j = j.add(BigUint::from_u64(1).unwrap());
        }

        if j == i.clone().div(BigUint::from_u64(2).unwrap()) {
            primes.push(i.clone());
            println!("{} is a prime",i.clone());
        }

        i = i.add(BigUint::from_u64(2).unwrap());
    }
}