use crate::domain::{
    repositories::quest_viewing::QuestViewingRepository,
    value_object::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};
use anyhow::{Ok, Result};
use std::sync::Arc;

pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        let result = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        let quest_model = result.to_model(adventurers_count);

        Ok(quest_model)
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        let quests = self.quest_viewing_repository.board_checking(filter).await?;

        let mut quest_models: Vec<QuestModel> = Vec::new();

        for quest in quests.into_iter() {
            let adventurers_count = self
                .quest_viewing_repository
                .adventurers_counting_by_quest_id(quest.id)
                .await?;

            quest_models.push(quest.to_model(adventurers_count));
        }

        Ok(quest_models)
    }
}
