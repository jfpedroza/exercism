pub fn raindrops(n: u32) -> String {
    let mut sounded = false;
    let mut result = String::new();

    if n % 3 == 0 {
        sounded = true;
        result += "Pling";
    }

    if n % 5 == 0 {
        sounded = true;
        result += "Plang";
    }

    if n % 7 == 0 {
        sounded = true;
        result += "Plong";
    }

    if !sounded {
        result = n.to_string();
    }

    result
}
