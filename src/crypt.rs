use rand::Rng;

/// generate secure password

const UPPERCASE_PROB : f32 = 0.4;

pub fn generate_secure_passwd(len: usize) -> String {
    let mut rng = rand::thread_rng();  
    let chars: Vec<char> = ('!'..='`').collect();
    let mut password = String::default();

    while password.len() < len {
        let rand = rng.gen_range(0..=chars.len());
        let prob = rng.gen_range(0..=1) as f32;
        let mut ch = chars[rand];

        if prob < UPPERCASE_PROB {
            let tmp: Vec<char> = ch.to_uppercase().collect();
            ch = tmp[0];
        }

        password.push(ch);
    }

    password
}
