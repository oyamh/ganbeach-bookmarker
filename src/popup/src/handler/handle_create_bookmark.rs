use super::handle_load_access_token;
use create_bookmark_context::CreateBookmarkContext;
use domain::{
    CreateBookmarkCommand, DomainError, FolderTitle, History, List, Metadata, PageUrl, Scene,
};
use futures::future::join_all;

// #[cfg(not(test))]
// use request_create_bookmark_context::RequestCreateBookmarkContext;
// #[cfg(test)]
// use request_create_bookmark_context_mock::RequestCreateBookmarkContext;

pub(crate) async fn handle_send_creating_bookmark(
    command: CreateBookmarkCommand,
) -> Result<Scene, DomainError> {
    let access_token = match handle_load_access_token().await {
        Ok(access_token) => access_token,
        Err(error) => {
            // handle_background_error(error, sender_info).await;
            return Err(error);
        }
    };

    let ctx = CreateBookmarkContext::new();

    let metadata = Metadata::new().with_access_token(access_token);
    let created_result = command.clone().request(&ctx, metadata).await?;

    match command.folder_title {
        FolderTitle::New(ref folder_title) => {
            usecase::create_browser_bookmark(&ctx, folder_title.clone(), PageUrl::default(), None)
                .await?;
        }
        FolderTitle::Old(_) => {}
    }

    let futures = command.new_bookmarks.iter().map(|req| {
        let req = req.to_owned();
        usecase::create_browser_bookmark(
            &ctx,
            req.name,
            req.url,
            Some(command.folder_title.to_owned().into()),
        )
    });
    let errors = join_all(futures)
        .await
        .into_iter()
        .filter(|result| result.is_err())
        .map(|err| {
            log::error!("browser bookmark error: {:#?}", err);
            err.err()
        })
        .flatten()
        .collect::<Vec<DomainError>>();
    if errors.len() > 0 {
        log::debug!("errors: {errors:#?}");
    }

    let created_lists = created_result
        .results
        .iter()
        .map(Into::<List>::into)
        .collect::<Vec<List>>();
    let (folder_id, folder_title) = if command.folder_id.is_some() {
        (
            command.folder_id.ok_or(DomainError::NotFoundParentId)?,
            command.folder_title.into(),
        )
    } else {
        let folders = created_lists
            .iter()
            .filter(|list| list.is_folder())
            .collect::<Vec<&List>>();
        // folders.last().unwrap().id().to_owned()
        let parent = folders.last().ok_or(DomainError::NotFoundNewFolder)?;
        (*parent.id(), parent.title().to_owned())
    };

    if let Err(error) = History::RecentFolderId(folder_id).save(&ctx).await {
        log::debug!("folder_id: {folder_id:?}");
        log::error!("store folder error: {error:?}");
    }
    if let Err(error) = History::RecentTagIds(command.tag_ids.clone())
        .save(&ctx)
        .await
    {
        log::error!("store tag error: {error:?}");
        return Err(error);
    }
    Ok(Scene::Created {
        folder_id,
        folder_title,
        tag_ids: command.tag_ids,
    })
}
