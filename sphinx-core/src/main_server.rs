use crate::JudgeReply;

pub trait MainServerClient {
    fn update_real_time_info(&mut self, reply: &JudgeReply);
}

