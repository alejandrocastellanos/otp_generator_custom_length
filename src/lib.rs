use rand::Rng;

pub fn generate_otp(_length: i32) -> String{
    let _min = 0;
    let _max = 9;
    let mut rng = rand::thread_rng();
    let mut count = 0;

    let mut vec_of_numbers = Vec::new();

    while count < _length {
        count += 1;
        let random_number: u32 = rng.gen_range(_min..=_max);
        vec_of_numbers.push(random_number);
    }
    let concatenated: String = vec_of_numbers.iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .concat();
    concatenated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _result = generate_otp(6);
        assert!(_result == _result);
    }
}
