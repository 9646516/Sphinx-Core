use crate::JudgeReply;
use async_trait::async_trait;

pub struct UpdateRealTimeInfoResult {
    pub a : i32,
    pub b : i64,
}

#[async_trait]
pub trait MainServerClient {
    async fn update_real_time_info(&mut self, reply: &JudgeReply<'_>) -> UpdateRealTimeInfoResult;
}

