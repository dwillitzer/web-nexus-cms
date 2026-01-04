// AI Integration Module
// Add to contracts/src/

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use garde::Validate;

// ============================================================================
// AI SERVICE PROVIDERS
// ============================================================================

/// AI service provider configuration
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AIProvider {
    /// OpenAI (GPT-4, GPT-3.5)
    OpenAI { api_key: String },

    /// Anthropic (Claude)
    Anthropic { api_key: String },

    /// Google AI (Gemini)
    GoogleAI { api_key: String },

    /// Local LLM (Ollama, etc.)
    Local { endpoint: String },

    /// Custom OpenAI-compatible API
    CustomOpenAI {
        endpoint: String,
        api_key: String,
    },
}

/// AI service configuration
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIServiceConfig {
    /// Unique config ID
    pub id: String,
    /// Site this config belongs to
    pub site_id: String,
    /// Provider type
    pub provider: AIProvider,
    /// Default model to use
    pub default_model: String,
    /// Max tokens per request
    pub max_tokens: Option<u32>,
    /// Temperature (0-2)
    pub temperature: Option<f32>,
    /// Monthly token limit
    pub monthly_limit: Option<u32>,
    /// Current monthly usage
    pub monthly_usage: u32,
    /// Service status
    pub status: AIServiceStatus,
}

/// AI service status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AIServiceStatus {
    /// Service is active
    Active,
    /// Service is disabled
    Disabled,
    /// Service credentials invalid
    Invalid,
    /// Rate limited
    RateLimited,
    /// Over token limit
    OverLimit,
    /// Service error
    Error(String),
}

// ============================================================================
// AI ASSISTANT TASKS
// ============================================================================

/// AI assistant task types
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AITask {
    /// Generate show announcement
    GenerateShowAnnouncement,
    /// Generate social media post
    GenerateSocialPost,
    /// Generate newsletter content
    GenerateNewsletter,
    /// Summarize analytics
    SummarizeAnalytics,
    /// Suggest setlist based on venue
    SuggestSetlist,
    /// Generate SEO description
    GenerateSEODescription,
    /// Reply to contact form
    ReplyToContact,
    /// Generate blog post outline
    GenerateBlogOutline,
    /// Custom prompt
    CustomPrompt { prompt: String },
}

/// AI assistant request
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIRequest {
    /// Unique request ID
    pub id: String,
    /// User making the request
    pub user_id: String,
    /// Site this request is for
    pub site_id: String,
    /// Task type
    pub task: AITask,
    /// Context data (show info, venue, etc.)
    pub context: serde_json::Value,
    /// Additional instructions
    pub instructions: Option<String>,
    /// Requested model (overrides default)
    pub model: Option<String>,
    /// Temperature override
    pub temperature: Option<f32>,
    /// Request status
    pub status: AIRequestStatus,
    /// Created timestamp
    pub created_at: i64,
    /// Completed timestamp
    pub completed_at: Option<i64>,
}

/// AI request status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AIRequestStatus {
    /// Request is queued
    Queued,
    /// Currently processing
    Processing,
    /// Completed successfully
    Completed,
    /// Failed
    Failed { error: String },
    /// Cancelled
    Cancelled,
}

/// AI assistant response
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIResponse {
    /// Request this response is for
    pub request_id: String,
    /// Generated content
    pub content: String,
    /// Tokens used
    pub tokens_used: u32,
    /// Model used
    pub model: String,
    /// Provider used
    pub provider: String,
    /// Response time in milliseconds
    pub response_time_ms: u32,
    /// Quality score (0-1)
    pub quality_score: Option<f32>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

// ============================================================================
// AI TRAINING & FINE-TUNING
// ============================================================================

/// Training dataset for custom models
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrainingDataset {
    /// Unique dataset ID
    pub id: String,
    /// Site this dataset belongs to
    pub site_id: String,
    /// Dataset name
    pub name: String,
    /// Dataset description
    pub description: Option<String>,
    /// Training examples
    pub examples: Vec<TrainingExample>,
    /// Dataset status
    pub status: DatasetStatus,
    /// Created timestamp
    pub created_at: i64,
}

/// Training example (prompt → completion pair)
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TrainingExample {
    /// Example ID
    pub id: String,
    /// Input prompt
    pub prompt: String,
    /// Expected completion
    pub completion: String,
    /// Quality score (0-1)
    pub quality_score: Option<f32>,
}

/// Dataset status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DatasetStatus {
    /// Dataset is being built
    Building,
    /// Ready for training
    Ready,
    /// Training in progress
    Training,
    /// Training completed
    Completed,
    /// Training failed
    Failed { error: String },
}

/// Fine-tuned model
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FineTunedModel {
    /// Unique model ID
    pub id: String,
    /// Site this model belongs to
    pub site_id: String,
    /// Model name
    pub name: String,
    /// Base model used
    pub base_model: String,
    /// Dataset used for training
    pub dataset_id: String,
    /// Model status
    pub status: ModelStatus,
    /// Training progress (0-100)
    pub training_progress: u8,
    /// Model endpoint (when deployed)
    pub endpoint: Option<String>,
    /// Created timestamp
    pub created_at: i64,
}

/// Fine-tuned model status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ModelStatus {
    /// Model is being prepared
    Preparing,
    /// Training in progress
    Training,
    /// Training completed, ready for use
    Ready,
    /// Model is deployed
    Deployed,
    /// Training failed
    Failed { error: String },
}

// ============================================================================
// AI INSIGHTS & ANALYTICS
// ============================================================================

/// AI-generated insight
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIInsight {
    /// Unique insight ID
    pub id: String,
    /// Site this insight is for
    pub site_id: String,
    /// Insight type
    pub insight_type: AIInsightType,
    /// Insight title
    pub title: String,
    /// Insight description
    pub description: String,
    /// Confidence score (0-1)
    pub confidence: f32,
    /// Actionable recommendations
    pub recommendations: Vec<String>,
    /// Related data
    pub data: serde_json::Value,
    /// Generated timestamp
    pub generated_at: i64,
}

/// Types of AI insights
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AIInsightType {
    /// Best performing content
    TopContent,
    /// Audience engagement patterns
    EngagementPattern,
    /// Optimal posting times
    OptimalTiming,
    /// Content suggestions
    ContentSuggestions,
    /// Trending topics
    TrendingTopics,
    /// Audience demographics
    Demographics,
    /// Sentiment analysis
    SentimentAnalysis,
}

// ============================================================================
// AI PROMPT TEMPLATES
// ============================================================================

/// Reusable AI prompt template
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIPromptTemplate {
    /// Unique template ID
    pub id: String,
    /// Site this template belongs to
    pub site_id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: Option<String>,
    /// System prompt
    pub system_prompt: String,
    /// User prompt template (with {{variables}})
    pub user_prompt: String,
    /// Available variables
    pub variables: Vec<TemplateVariable>,
    /// Expected output format
    pub output_format: AIOutputFormat,
    /// Is template active?
    pub is_active: bool,
}

/// AI output format
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AIOutputFormat {
    /// Plain text
    Text,
    /// Markdown
    Markdown,
    /// HTML
    Html,
    /// JSON
    Json,
    /// Code block
    Code { language: String },
}

// ============================================================================
// AI USAGE & BILLING
// ============================================================================

/// AI usage tracking
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIUsage {
    /// Unique usage ID
    pub id: String,
    /// Site this usage is for
    pub site_id: String,
    /// User who made the request
    pub user_id: String,
    /// Model used
    pub model: String,
    /// Provider used
    pub provider: String,
    /// Tokens used (input)
    pub input_tokens: u32,
    /// Tokens used (output)
    pub output_tokens: u32,
    /// Total tokens
    pub total_tokens: u32,
    /// Cost in USD
    pub cost_usd: f32,
    /// Request timestamp
    pub timestamp: i64,
}

/// Monthly AI usage summary
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AIUsageSummary {
    /// Site this summary is for
    pub site_id: String,
    /// Month (YYYY-MM)
    pub month: String,
    /// Total requests
    pub total_requests: u32,
    /// Total tokens used
    pub total_tokens: u64,
    /// Total cost
    pub total_cost_usd: f32,
    /// Breakdown by model
    pub by_model: Vec<ModelUsage>,
    /// Breakdown by user
    pub by_user: Vec<UserUsage>,
}

/// Usage by specific model
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelUsage {
    pub model: String,
    pub requests: u32,
    pub tokens: u64,
    pub cost_usd: f32,
}

/// Usage by specific user
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUsage {
    pub user_id: String,
    pub user_name: String,
    pub requests: u32,
    pub tokens: u64,
    pub cost_usd: f32,
}
