pub trait ConvenienceWrap {
    fn also<F: FnMut(&Self) -> ()>(self, mut callback: F) -> Self
    where
        Self: Sized,
    {
        callback(&self);
        self
    }

    fn apply<F: FnMut(&mut Self) -> ()>(mut self, mut callback: F) -> Self
    where
        Self: Sized,
    {
        callback(&mut self);
        self
    }

    fn map<F: FnMut(Self) -> R, R>(self, mut transform: F) -> R
    where
        Self: Sized,
    {
        transform(self)
    }
}

