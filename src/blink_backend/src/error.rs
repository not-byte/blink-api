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
            ErrorKind::UserDoesNotExist => Self::new("User does not exist"),
            ErrorKind::ConversationNotFound => Self::new("Conversation not found"),
            ErrorKind::UserNotInConversation => Self::new("User not in conversation"),
            ErrorKind::CantAccess => Self::new("You can't access other conversations"),
            ErrorKind::MessageNotFound => Self::new("Message not found"),
            ErrorKind::CantEdit => Self::new("You can't edit this message"),
            ErrorKind::ConversationAlreadyExists(id) => {
                Self::new(format!("Conversation already exists: {id}").as_str())
            }
            ErrorKind::CantRemoveConversation => Self::new("You can't remove this conversation"),
            ErrorKind::UserAlreadyExists => Self::new("User already exists"),
            ErrorKind::Anonymous => Self::new("User is anonymous"),
        }
    }
}
