use super::Pid;
use crate::diagnostics::DiagnosticsTypeName;
use crate::message::Message;
use std::any::type_name;
use std::collections::HashMap;

pub type MessageHeader = HashMap<String, String>;

/// Adds headers and sender information to a message.
///
/// Message must implement [Message] trait.
pub struct MessageEnvelope<M>
where
    M: Message + Send,
    M::Result: Send,
{
    sender: Option<Pid>,
    message: M,
    header: MessageHeader,
}

impl<M> MessageEnvelope<M>
where
    M: Message + Send + 'static,
    M::Result: Send,
{
    /// Creates a new message envelope.
    ///
    /// # Arguments
    /// * `message` - message to wrap
    /// * `sender` - Sender [Pid]
    /// * `header` - Headers
    ///
    /// # Returns
    pub fn new(message: M, sender: Option<Pid>, header: Option<MessageHeader>) -> Self {
        Self {
            message,
            sender,
            header: Option::unwrap_or_default(header),
        }
    }

    #[inline]
    pub fn wrap(message: M) -> Self {
        message.into()
    }

    #[inline]
    pub fn get_message(&self) -> &M {
        &self.message
    }

    #[inline]
    pub fn get_sender(&self) -> &Option<Pid> {
        &self.sender
    }

    #[inline]
    pub fn get_header(&self) -> &MessageHeader {
        &self.header
    }

    pub fn with_sender(&mut self, sender: Pid) -> &mut Self {
        self.sender = Some(sender);
        self
    }

    pub fn with_header<T>(&mut self, header: T) -> &mut Self
    where
        T: Into<MessageHeader>,
    {
        self.header = header.into();
        self
    }

    /// Extends the message envelope with additional headers.
    ///
    /// Since this feature should be used in middlewares or context decorators, and considering
    /// security implications from remoting, for any header key that exists in envelope it's value
    /// will be overwritten with value from header argument with same key.
    ///
    /// # Arguments:
    /// * `header` - A instance of header elements.
    /// # Examples:
    /// ```
    ///  use protoactor::message::{Message, MessageHeader, MessageEnvelope};
    ///
    ///  struct TestMessage;
    ///
    ///  impl Message for TestMessage {
    ///     type Result = ();
    ///  }
    ///
    ///  let mut envelope = MessageEnvelope::wrap(TestMessage);
    ///  envelope.with_header([
    ///         ("key0".to_string(), "value0".to_string()),
    ///         ("key1".to_string(), "value1".to_string()),
    ///     ]);
    ///  let to_merge: MessageHeader = [
    ///      ("key1".to_string(), "value1".to_string()),
    ///      ("key2".to_string(), "value2".to_string()),
    ///  ].into();
    ///
    ///  envelope.merge_header(to_merge);
    ///
    ///  assert_eq!(3, envelope.get_header().len());
    ///  assert_eq!(
    ///     *envelope.get_header(),
    ///     [
    ///         ("key0".to_string(), "value0".to_string()),
    ///         ("key1".to_string(), "value1".to_string()),
    ///         ("key2".to_string(), "value2".to_string()),
    ///     ].into()
    /// );
    /// ```
    #[inline]
    pub fn merge_header(&mut self, header: MessageHeader) {
        self.header.extend(header);
    }
}

/// Blanked implementation for any message that implements [Message] trait.
impl<M> From<M> for MessageEnvelope<M>
where
    M: Message + Send + 'static,
    M::Result: Send,
{
    /// Wraps message [M] into [MessageEnvelope]. Sender pid is set to [None]
    fn from(msg: M) -> Self {
        Self {
            message: msg,
            sender: None,
            header: Default::default(),
        }
    }
}

impl<M> DiagnosticsTypeName for MessageEnvelope<M>
where
    M: Message + Send + 'static,
    M::Result: Send,
{
    fn get_type_name(&self) -> String {
        // todo: How to silently handle type name of the message?
        format!("MessageEnvelope({})", type_name::<M>())
    }
}

#[cfg(test)]
mod tests {
    use crate::diagnostics::DiagnosticsTypeName;
    use crate::message::{Message, MessageEnvelope, MessageHeader};
    use std::any::{Any, TypeId};

    struct TestMessage;

    impl Message for TestMessage {
        type Result = ();
    }

    #[test]
    fn it_can_wrap_message() {
        let envelope: MessageEnvelope<TestMessage> = TestMessage.into();
        assert_eq!(
            TypeId::of::<TestMessage>(),
            envelope.get_message().type_id()
        );
    }

    #[test]
    fn should_merge_header_by_overwriting_existing() {
        let expected_result = MessageHeader::from([
            ("key0".to_string(), "value0".to_string()),
            ("key1".to_string(), "overwritten".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);
        let to_merge = MessageHeader::from([
            ("key1".to_string(), "overwritten".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]);

        let mut envelope = MessageEnvelope::wrap(TestMessage);
        envelope.with_header(MessageHeader::from([
            ("key0".to_string(), "value0".to_string()),
            ("key1".to_string(), "value1".to_string()),
        ]));
        envelope.merge_header(to_merge);

        assert_eq!(3, envelope.get_header().len());
        assert_eq!(expected_result, *envelope.get_header())
    }

    #[test]
    fn should_return_diagnostics_type_name() {
        let name = MessageEnvelope::wrap(TestMessage).get_type_name();
        assert_eq!(
            "MessageEnvelope(protoactor::message::message_envelope::tests::TestMessage)",
            name
        )
    }
}
