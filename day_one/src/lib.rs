pub fn works() -> bool {
    return true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert(works());
    }
}
