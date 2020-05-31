extern crate rusty_machine_serde as rm;

#[cfg(feature = "datasets")]
pub mod test {

    use rm::datasets;
    use rm::linalg_serde::BaseMatrix;

    #[test]
    fn test_iris() {
        let dt = datasets::iris::load();
        assert_eq!(dt.data().rows(), 150);
        assert_eq!(dt.data().cols(), 4);

        assert_eq!(dt.target().size(), 150);
    }

    #[test]
    fn test_trees() {
        let dt = datasets::trees::load();
        assert_eq!(dt.data().rows(), 31);
        assert_eq!(dt.data().cols(), 2);

        assert_eq!(dt.target().size(), 31);
    }
}
