slint::include_modules!();

#[test]
fn test_slint_components_compilation() {
    let window = LtkTest::new().unwrap();
    assert_eq!(window.get_active_test(), 0);

    let ltk_window = LtkWindow::new().unwrap();
    assert_eq!(ltk_window.get_current_tab(), 0);
}
