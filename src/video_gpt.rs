use anthropic_ai_sdk::{
    client::AnthropicClient,
    types::message::{
        ContentBlock, CreateMessageParams, Message, MessageClient, MessageError,
        RequiredMessageParams, Role,
    },
};

use crate::supdata::Supdata;

const ANTHROPIC_SONNET_MODEL: &str = "claude-sonnet-4-20250514";
const ANTHROPIC_HAIKU_MODEL: &str = "claude-3-5-haiku-latest";

const MAX_TOKENS: u32 = 64000;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Level {
    High,
    Low,
}

pub struct VGConfig {
    pub supdata_key: String,
    pub anthropic_key: String,
    pub level: Level,
}
pub struct VideoGpt {
    supdata: Supdata,
    anthropic: AnthropicClient,

    system_prompt: String,
    model: String,
}

impl VideoGpt {
    pub fn new(config: VGConfig) -> Self {
        let supdata = Supdata::new(config.supdata_key);
        let anthropic =
            AnthropicClient::new::<MessageError>(config.anthropic_key, "2023-06-01").unwrap();

        let system_prompt = r#"
            You are an expert technical writer tasked with turning a raw tutorial transcript into a clear, complete, and action-oriented guide for others to follow.

            • TRANSCRIPT:   Inlcuded between <transcript> and </transcript> tags.

            === WHAT TO DELIVER ===  
            Return structured **Markdown** document that captures all essential knowledge—no detail left behind—and translates it into concise, executable instructions.

            ## 1 - TL;DR (≤3 sentences)  
            • A lightning-quick overview of what the viewer learns.

            ## 2 - Key Takeaways  
            • Bullet list of the **5-12** most important points.  
            • Focus on business/e-commerce/marketing relevance.  
            • Include short **verbatim quotes** only when they sharpen the point (“quote”).  
            • If a takeaway suggests action, tag the bullet start with **[Actionable]**; otherwise omit the tag.

            ## 3 - Actionable Playbook  
            • Step-by-step instructions someone could follow to implement the actionable takeaways.  
            • Use ordered lists; be concrete (tools, metrics, timelines when mentioned).  
            • If a step depends on assumptions the video doesn't cover, note the assumption.

            ## 4 - Interesting Nuggets  
            • Brief bullets of “good to know” facts, anecdotes, stats, or context that aren't directly actionable.

            ## 5 - Suggested Follow-Up Tasks  
            • Up to five bullets beginning with an **action verb**, phrased like reminders you'd send me.  
            _Examples: “Draft a headline A/B test using the 3-act copy formula.”_


            === STYLE NOTES ===  
            • Write in clear, professional English.  
            • Keep bullets concise; avoid fluff and repetition.  
            • No timestamps are needed.  
            • Use Markdown conventions only (no HTML).  
            • Do not mention these instructions in your output.
        "#.to_string();

        // ## 5 - Glossary  
        // | Term | Plain-English definition (≤15 words) |  
        // | ---- | ------------------------------------ |  
        // | …    | …                                    |  
        // • Include every bit of jargon or niche acronym found in the transcript.  
        // • Omit the section entirely if no jargon appears.

        Self {
            supdata,
            anthropic,
            system_prompt,
            model: match config.level {
                Level::High => ANTHROPIC_SONNET_MODEL.to_string(),
                Level::Low => ANTHROPIC_HAIKU_MODEL.to_string(),
            },
        }
    }

    pub async fn get_transcript(&self, video_url: &str) -> anyhow::Result<String> {
        let transcript = self.supdata.get_transcript(video_url).await?;
        Ok(transcript)
    }

    pub async fn get_instructions(&self, transcript: &str) -> anyhow::Result<String> {
        let body: CreateMessageParams = CreateMessageParams::new(RequiredMessageParams {
            model: self.model.clone(),
            messages: vec![
                Message::new_text(Role::User, self.system_prompt.clone()), // system
                Message::new_text(
                    Role::User,
                    format!("<transcript>{}</transcript>", transcript),
                ),
            ],
            max_tokens: MAX_TOKENS,
        });

        let response = self.anthropic.create_message(Some(&body)).await.unwrap();

        let summary = response.content[0].clone();
        let summary_text = match summary {
            ContentBlock::Text { text } => text,
            _ => panic!("Expected text content"),
        };

        Ok(summary_text)
    }

    pub async fn get_gpt(&self, url: &str) -> anyhow::Result<String> {
        let transcript = self.get_transcript(url).await?;
        let gpt = self.get_instructions(&transcript).await?;

        Ok(gpt)
    }
}
