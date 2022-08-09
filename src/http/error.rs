
// pub enum ParseError {
//     InvalidRequest,
//     InvalidEncoding,
//     InvalidProtocol,
//     InvalidMethod,
// }


// impl Error for ParseError {}

// impl From<Utf8Error> for ParseError {
//     fn from(_: Utf8Error) -> Self {
//         Self::InvalidEncoding
//     }
// }

// impl ParseError {
//     fn message(&self) -> &str {
//         match self {
//             ParseError::InvalidRequest => "Invalid Request",
//             ParseError::InvalidEncoding => "Invalid Encoding",
//             ParseError::InvalidProtocol => "Invalid Protocol",
//             ParseError::InvalidMethod => "Invalid Method",
//         }
//     }
// }

// impl Display for ParseError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "{}", self.message())
//     }
// }

// impl Debug for ParseError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "{}", self.message())
//     }
// }
