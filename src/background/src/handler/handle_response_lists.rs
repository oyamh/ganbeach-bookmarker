use domain::{DomainError, Lists, MessageToPopup, SenderInfo};
use load_all_lists_context::LoadAllListsContext;

pub(crate) async fn handle_response_lists(
    sender_info: SenderInfo,
    lists: Lists,
) -> Result<(), DomainError> {
    log::debug!("handle_response_lists");
    let ctx = LoadAllListsContext::new();
    log::debug!("sender_info: {sender_info:#?}");
    log::debug!("lists: {:?}", &lists);

    usecase::send_outer_message(&ctx, sender_info, MessageToPopup::Lists(lists)).await?;
    Ok(())
}
