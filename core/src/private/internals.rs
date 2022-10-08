#[repr(transparent)]
pub struct Internal<T>(pub T);

impl<T> From<T> for Internal<T> {
    fn from(this: T) -> Self {
        Self(this)
    }
}

impl<T> Internal<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn map<R>(self, mapper: impl FnOnce(T) -> R) -> Internal<R> {
        mapper(self.into_inner()).into()
    }
}

impl<T> core::ops::Deref for Internal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Internal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Internal<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Internal<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
