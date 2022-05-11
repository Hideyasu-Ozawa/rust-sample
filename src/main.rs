fn simple_branch(flg: bool) -> String {
    if flg {
        "true branch".to_string()
    } else {
        "false branch".to_string()
    }
}

fn main() {
    println!("Hello, world!");

    let flg = false;

    simple_branch(flg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_branch() {
        assert_eq!("true branch", simple_branch(true));
    }

    #[test]
    fn false_branch() {
        assert_eq!("false branch", simple_branch(false));
    }

    // #[test]
    // fn mian() {
    //     assert_eq!((), main());
    // }
}
