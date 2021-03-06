mod drop;

use self::drop::{DetectDrop, Flag};
use eyre::DefaultContext;
use eyre::ErrReport;
use std::marker::Unpin;
use std::mem;

#[test]
fn test_error_size() {
    assert_eq!(mem::size_of::<ErrReport>(), mem::size_of::<usize>());
}

#[test]
fn test_null_pointer_optimization() {
    assert_eq!(
        mem::size_of::<Result<(), ErrReport>>(),
        mem::size_of::<usize>()
    );
}

#[test]
fn test_autotraits() {
    fn assert<E: Unpin + Send + Sync + 'static>() {}
    assert::<ErrReport>();
}

#[test]
fn test_drop() {
    let has_dropped = Flag::new();
    drop(ErrReport::<DefaultContext>::new(DetectDrop::new(
        &has_dropped,
    )));
    assert!(has_dropped.get());
}
