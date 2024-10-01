fn main() {
    println!("Hello, world! Again!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
