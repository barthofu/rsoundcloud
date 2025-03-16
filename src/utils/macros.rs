#[macro_export]
macro_rules! need_authentication {
    ($self:ident) => {
        if !$self.is_authenticated() {
            return Err(crate::errors::ClientError::ShouldBeAuthenticated());
        }
    }
}
