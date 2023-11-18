#[macro_export]
macro_rules! resolve_constant {
    ($type:path, $index:expr, $constant_pool:expr) => {
        match $constant_pool.get($index) {
            $type(value) => value,
            _ => panic!(
                "ConstantPoolInfoType::{} expected at index {}",
                stringify!($type),
                $index
            ),
        }
    };
}
