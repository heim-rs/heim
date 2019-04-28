pub trait StreamExt2 {
}

pub trait TryStreamExt2 {
    fn try_then<T, F>(self, f: F)
}
