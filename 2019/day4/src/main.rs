fn main() {
    let m:i32 = 356261;
    let n:i32 = 846303;
    let mut counter = 0;
    let mut checks = 0;
    let mut i = m;
    loop {
        checks += 1;
        if i > n {
            break;
        }

        let s = i.to_string();
        let mut previous_digit = s.chars().next().unwrap();

        // Need to count the digits for each password for part 2
        let mut digit_count = Vec::new();
        for _x in 0..10 {
            digit_count.push(0);
        }
        digit_count[previous_digit as usize - 48] += 1;

        let mut dup_criteria = false;

        for (x, current_digit) in s.chars().enumerate() {
            if x == 0 {continue;}
            digit_count[current_digit as usize - 48] += 1;
            if previous_digit as i8 > current_digit as i8 {
                // Replace current digit and all subsequent digits with previous digit
                let new_digit = previous_digit as i8;
                let mut bytes = s.clone().into_bytes();
                for y in x..6{
                    bytes[y] = new_digit as u8;
                }
                let new_s:String;
                unsafe { new_s = String::from_utf8_unchecked(bytes) }
                i = new_s.parse::<i32>().unwrap() - 1;
                // Fail dupe check (criteria 3) on purpose because criteria 4 has failed
                dup_criteria = false;
                break;
            }
            else if previous_digit == current_digit {
                dup_criteria = true;
            }
            previous_digit = current_digit;
        }
        if dup_criteria {
            let mut have_double:bool = false;
            for x in digit_count{
                if x == 2 {
                    have_double = true;
                }
            }
            if have_double {
                counter += 1;
                println!("{}",i);
            }
        }
        i+=1;
    }
    println!("---------------------------");
    println!("Total possible combinations: {}", counter);
    println!("Total checks: {}", checks);
}
