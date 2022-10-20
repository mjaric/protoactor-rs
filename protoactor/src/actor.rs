/// Trait that marks struct as an actor
pub trait Actor
where
    Self: Send + Unpin + 'static,
{
    type Context;
}
