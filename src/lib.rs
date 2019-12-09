mod intcode_vm;
pub use intcode_vm::{IntcodeVM, VMState};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
