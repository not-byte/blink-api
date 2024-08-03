use candid::CandidType;

#[derive(CandidType)]
pub struct Error {
    pub message: String,
}

impl Error {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

pub enum ErrorKind {
    UserDoesNotExist,
    UserAlreadyExists,
    UserNotInConversation,
    CantAccess,
    MessageNotFound,
    CantEdit,
    ConversationAlreadyExists(u64),
    CantRemoveConversation,
    ConversationNotFound,
    Anonymous,
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        match value {
            ErrorKind::UserDoesNotExist => Error::new("User does not exist"),
            ErrorKind::ConversationNotFound => Error::new("Conversation not found"),
            ErrorKind::UserNotInConversation => Error::new("User not in conversation"),
            ErrorKind::CantAccess => Error::new("You can't access other conversations"),
            ErrorKind::MessageNotFound => Error::new("Message not found"),
            ErrorKind::CantEdit => Error::new("You can't edit this message"),
            ErrorKind::ConversationAlreadyExists(id) => {
                Error::new(format!("Conversation already exists: {}", id).as_str())
            }
            ErrorKind::CantRemoveConversation => Error::new("You can't remove this conversation"),
            ErrorKind::UserAlreadyExists => Error::new("User already exists"),
            ErrorKind::Anonymous => Error::new("User is anonymous"),
        }
    }
}
