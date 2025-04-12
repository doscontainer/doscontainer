#[cfg(test)]
mod tests {
    use crate::layer::Layer;

    #[test]
    fn valid_url() {
        let mut layer = Layer::default();
        assert!(layer.set_url("https://www.testurl.com").is_ok());
    }

    #[test]
    fn invalid_url() {
        let mut layer = Layer::default();
        assert!(layer.set_url("f24fasdf2rq3grfasdf").is_err());
    }
}
