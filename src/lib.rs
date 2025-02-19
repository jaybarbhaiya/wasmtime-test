#[no_mangle]
pub extern "C" fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[no_mangle]
pub extern "C" fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

#[no_mangle]
pub extern "C" fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

#[no_mangle]
pub extern "C" fn divide(left: u64, right: u64) -> u64 {
    left / right
}

#[no_mangle]
pub extern "C" fn remainder(left: u64, right: u64) -> u64 {
    left % right
}

#[no_mangle]
pub extern "C" fn power(base: u64, exponent: u64) -> u64 {
    base.pow(exponent as u32)
}

#[no_mangle]
pub extern "C" fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
