fn calculate_fuel (input: u32) -> Option<u32>{
    let mut _i = input;
     if _i.checked_rem(3) == None { 
        return None
    }
    _i = _i/3;
    if _i.checked_sub(2) == None {
        return None
    }
     _i = _i-2;
    return Some(_i);
}

fn calculate_total_fuel(mass: u32) -> u32 {
    let mut total_mass: u32 = 0;
    let mut _mass: u32 = mass;

    while calculate_fuel(_mass) != None {
        _mass = calculate_fuel(_mass).unwrap();
        total_mass += _mass;
    }
    return total_mass;
}

pub fn rocket_science() -> u32{
    let mut sum: u32 = 0;
    let _array = [
        139936, 96114, 73984, 101283, 71668, 119408, 54029, 134890, 83831, 147043, 70059, 136810,
        124397, 123543, 107793, 117051, 111300, 93214, 91691, 106815, 61783, 138531, 69277, 75307,
        125517, 72622, 117648, 71161, 147510, 133560, 147273, 101023, 100171, 108241, 128962,
        85755, 50371, 141491, 96585, 103280, 122493, 126025, 124114, 123153, 94956, 86491, 54630,
        112399, 121515, 58560, 80211, 84893, 103375, 65563, 64408, 131671, 90149, 131040, 138115,
        99987, 51281, 57641, 130018, 141946, 111726, 99761, 54792, 75213, 71352, 59004, 136500,
        148962, 144283, 114983, 97115, 87136, 137860, 146991, 67090, 51705, 99242, 109796, 147943,
        83255, 57070, 55343, 67854, 101564, 74996, 74542, 57494, 90227, 69965, 103978, 142175,
        116700, 70493, 62383, 100870, 110806,
    ];
    for x in _array.iter(){
        let val = *x as u32;
        sum += calculate_total_fuel(val);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
fn it_should_calculate_correctly() {
    assert_eq!(calculate_fuel(12),Some(2));
    assert_eq!(calculate_fuel(14),Some(2));
    assert_eq!(calculate_fuel(1969),Some(654));
    assert_eq!(calculate_fuel(100756),Some(33583));
}
#[test]
fn it_should_calculate_fuel_correctly() {
    assert_eq!(calculate_total_fuel(14),2);
    assert_eq!(calculate_total_fuel(1969),966);
    assert_eq!(calculate_total_fuel(100756),50346);
}
}