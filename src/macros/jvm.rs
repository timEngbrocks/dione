#[macro_export]
macro_rules! jvm {
    () => {
        JVM::it() as &'static mut JVM
    };
}
