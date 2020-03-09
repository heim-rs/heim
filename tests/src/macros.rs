#[macro_export]
macro_rules! assert_delta_le {
    ($left:expr, $right:expr, $relative:expr,) => {
        $crate::assert_delta_le!($left, $right);
    };
    ($left:expr, $right:expr, $relative:expr) => {
        let delta = if $left > $right {
            $left.saturating_sub($right)
        } else {
            $right.saturating_sub($left)
        };

        claim::assert_ge!(delta, $relative);
    };
}
