fn foo() {
    panic!()
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn test() {
        foo();
    }
}