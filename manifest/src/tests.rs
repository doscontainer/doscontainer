#[cfg(test)]
mod tests {
    use crate::layer::Layer;

    #[test]
    fn set_valid_url() {
        let mut layer = Layer::default();
        assert!(layer.set_url("https://www.testurl.com").is_ok());
    }

    #[test]
    fn set_invalid_url() {
        let mut layer = Layer::default();
        assert!(layer.set_url("f24fasdf2rq3grfasdf").is_err());
    }

    #[test]
    fn set_category_floppy() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_category("floppy").is_ok());
    }

    #[test]
    fn set_category_hdd() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_category("hdd").is_ok());
    }

    #[test]
    fn set_category_invalid() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_category("INVALID").is_err());
    }

    #[test]
    fn set_disktype_f525_160() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f525_160").is_ok());
    }

    #[test]
    fn set_disktype_f525_180() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f525_180").is_ok());
    }

    #[test]
    fn set_disktype_f525_320() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f525_320").is_ok());
    }

    #[test]
    fn set_disktype_f525_360() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f525_360").is_ok());
    }

    #[test]
    fn set_disktype_f525_12m() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f525_12m").is_ok());
    }

    #[test]
    fn set_disktype_f35_720() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f35_720").is_ok());
    }

    #[test]
    fn set_disktype_f35_144() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f35_144").is_ok());
    }

    #[test]
    fn set_disktype_f35_288() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("f35_288").is_ok());
    }

    #[test]
    fn set_disktype_invalid() {
        let mut layer = Layer::default();
        assert!(layer.set_disk_type("INVALID").is_err());
    }

    #[test]
    fn unsupported_url_schema() {
        let mut layer = Layer::default();
        layer.set_url("file://command.com").unwrap();
        assert!(layer.download().is_err());
    }

    #[test]
    fn download_http() {
        let mut layer = Layer::default();
        let result = layer
            .set_url("https://dosk8s-dist.area536.com/alleycat.zip")
            .unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn download_ftp() {
        let mut layer = Layer::default();
        let result = layer
            .set_url("ftp://ftp.area536.com/doscontainer/distfiles/apicd214.zip")
            .unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn validate_zip_integrity() {
        let mut layer = Layer::default();
        layer
            .set_url("https://dosk8s-dist.area536.com/alleycat.zip")
            .expect("Boom!");
        layer.download().expect("failed to download Alleycat.zip");
        let _ = layer.validate_zip_file();
    }
}
