use domain::DomainError;

pub struct ErrorTitleMatcher {}

impl ErrorTitleMatcher {
    pub(crate) fn get_title(error: &DomainError) -> &'static str {
        use DomainError::*;
        match error {
            LoginUrlInstaedOfToken(_) => "Access Token Error",
            GetAccessToken(_) => "Please Login",
            ParseUrl(_) => "Bookmark Error",
            ParseAuthProvider(_) => "Bookmark Error",
            ParseBookmarkId(_) => "Bookmark Error",
            ParseJsValue(_) => "Bookmark Error",
            InvalidNotificationId(_) => "Invalid Parameter",
            CreateBookmarks(_) => "Bookmark Error",
            Server(_) => "Server Error",
            Database(_) => "Browser API Error",
            WebInterface(_) => "Browser API Error",
            Message(_) => "Message Error",
            NotFoundParentId => "Unexpected Error D1",
            NotFoundNewFolder => "Unexpected Error D2",
            DomException(_) => "Dom Exception",
            UnexpectedJsType(_, _) => "Unexpected Js Type Error",
            ConvertJsValue(_) => "Js Convert Error",
            EmptyValue(_) => "Empty Value",
            EmptyJsValue(_) => "Empty JS Value",
            JsValue(_) => "JS Error",
        }
    }
}
