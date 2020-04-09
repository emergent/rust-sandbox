use std::fmt;

enum Lang {
    JA,
    EN,
    ZH,
    KO,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Lang::JA => write!(f, "ja"),
            Lang::EN => write!(f, "en"),
            Lang::ZH => write!(f, "zh"),
            Lang::KO => write!(f, "ko"),
        }
    }
}

fn main() {
    println!("{}", Lang::JA);
    println!("{}", Lang::EN);
    println!("{}", Lang::ZH);
    println!("{}", Lang::KO);

    let s = format!("formatted:{}", Lang::JA);
    println!("{}", s);
    let t = Lang::JA.to_string();
    println!("{}", t);
}
