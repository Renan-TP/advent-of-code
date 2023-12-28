
fn parse(input: &str)  -> String{
    todo!()
}

pub fn process(input: &str) -> String {
    todo!();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!("4361", process(input))
    }
}
