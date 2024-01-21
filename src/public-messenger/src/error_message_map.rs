use domain::DomainError;

#[derive(Debug, Default)]
pub(crate) struct ErrorMessageMatcher {}

impl ErrorMessageMatcher {
    pub(crate) fn get_message(error: &DomainError, language: &Language) -> &'static str {
        use DomainError::*;
        let message = match error {
            LoginUrlInstaedOfToken(_) => match language {
                Language::English => "Re login",
                Language::Japanese => "再ログインします。",
            },
            GetAccessToken(_) => match language {
                Language::English => "Could not obtain an access token. Please login again.",
                Language::Japanese => {
                    "アクセストークンの取得ができませんでした。もう一度ログインしてください。"
                }
            },
            ParseUrl(_) => match language {
                Language::English => "Failed to parse URL. Please try again.",
                Language::Japanese => "URLの解析に失敗しました。",
            },
            ParseAuthProvider(_) => match language {
                Language::English => "Failed to parse Authentication Info. Please login again.",
                Language::Japanese => "認証情報の解析に失敗しました。",
            },
            ParseBookmarkId(_) => match language {
                Language::English => "Failed to parse Bookmark ID.",
                Language::Japanese => "ブックマークIDのデータ変換に失敗しました。",
            },
            ParseJsValue(_) => match language {
                Language::English => "Javascript data conversion failed.",
                Language::Japanese => "Javascriptのデータ変換に失敗しました。",
            },
            InvalidNotificationId(_) => match language {
                Language::English => "Invalid Notification ID.",
                Language::Japanese => "Notification IDが不正な値です。",
            },
            CreateBookmarks(_) => match language {
                Language::English => "Failed to create bookmarks. Please try again.",
                Language::Japanese => "ブックマークの作成に失敗しました。",
            },
            Server(_) => match language {
                Language::English => "Internal server error.",
                Language::Japanese => "サーバーでエラーが発生しました。",
            },
            Database(_) => match language {
                Language::English => "Error occurred in the browser internal database.",
                Language::Japanese => "ブラウザ内部データベースでエラーが発生しました。",
            },
            WebInterface(_) => match language {
                Language::English => "Browser API execution failed.",
                Language::Japanese => "ブラウザAPIの実行に失敗しました。",
            },
            Message(_) => match language {
                Language::English => "Communication between tab and browser failed.",
                Language::Japanese => "タブとブラウザ間の通信に失敗しました。",
            },
            NotFoundParentId => match language {
                Language::English => "Unexpected Error D1",
                Language::Japanese => "予期せぬエラー D1",
            },
            NotFoundNewFolder => match language {
                Language::English => "Unexpected Error D2",
                Language::Japanese => "予期せぬエラー D2",
            },
            DomException(_) => match language {
                Language::English => "Dom Exception Error occurred.",
                Language::Japanese => "Dom Exceptionエラーが発生しました。",
            },
            UnexpectedJsType(_, _) => match language {
                Language::English => "Unexpected Js Type Error occurred.",
                Language::Japanese => "予期しない型エラーが発生しました。(JS)",
            },
            ConvertJsValue(_) => match language {
                Language::English => "Failed to convert JS value.",
                Language::Japanese => "型変換に失敗しました。(JS)",
            },
            EmptyValue(_) => match language {
                Language::English => "Got empty value.",
                Language::Japanese => "取得した値が空でした。",
            },
            EmptyJsValue(_) => match language {
                Language::English => "Get empty JS value.",
                Language::Japanese => "取得した値が空でした。(JS)",
            },
            JsValue(_) => match language {
                Language::English => "Unexpected Error D3",
                Language::Japanese => "予期せぬエラー D3",
            },
        };
        message
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum Language {
    #[default]
    English,
    #[allow(dead_code)]
    Japanese,
}
