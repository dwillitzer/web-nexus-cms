// Email & SMTP Integration Module
// Add to contracts/src/

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use garde::Validate;

// ============================================================================
// EMAIL SERVICE PROVIDERS
// ============================================================================

/// Email service provider configuration
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum EmailProvider {
    /// SendGrid
    SendGrid { api_key: String },

    /// Mailgun
    Mailgun { api_key: String, domain: String },

    /// AWS SES
    AwsSes {
        access_key: String,
        secret_key: String,
        region: String,
    },

    /// Postmark
    Postmark { api_key: String },

    /// Mailchimp Transactional
    Mailchimp { api_key: String },

    /// Custom SMTP server
    CustomSmtp {
        host: String,
        port: u16,
        username: String,
        password: String,
        use_tls: bool,
    },

    /// Cloudflare Email Routing (for receiving)
    CloudflareRouting,
}

/// Email service configuration
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailServiceConfig {
    /// Unique config ID
    pub id: String,
    /// Site this config belongs to
    pub site_id: String,
    /// Provider type
    pub provider: EmailProvider,
    /// Default from email address
    #[garde(email)]
    pub from_email: String,
    /// Default from name
    pub from_name: String,
    /// Reply-to email
    pub reply_to: Option<String>,
    /// Is this the default service?
    pub is_default: bool,
    /// Daily send limit
    pub daily_limit: Option<i32>,
    /// Current daily sends
    pub daily_sends: i32,
    /// Service status
    pub status: EmailServiceStatus,
}

/// Email service status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum EmailServiceStatus {
    /// Service is active
    Active,
    /// Service is disabled
    Disabled,
    /// Service credentials invalid
    Invalid,
    /// Service rate limited
    RateLimited,
    /// Service error
    Error(String),
}

// ============================================================================
// EMAIL TEMPLATES
// ============================================================================

/// Email template for automated emails
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailTemplate {
    /// Unique template ID
    pub id: String,
    /// Site this template belongs to
    pub site_id: String,
    /// Template name
    pub name: String,
    /// Template slug
    pub slug: String,
    /// Subject line (can use {{variables}})
    pub subject: String,
    /// HTML body (can use {{variables}})
    pub body_html: String,
    /// Plain text body
    pub body_text: String,
    /// Available variables ({{show_title}}, {{venue}}, etc.)
    pub variables: Vec<TemplateVariable>,
    /// Template category
    pub category: EmailTemplateCategory,
    /// Is this template active?
    pub is_active: bool,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
}

/// Template variable definition
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TemplateVariable {
    /// Variable name (without {{ }})
    pub name: String,
    /// Variable description
    pub description: String,
    /// Is this variable required?
    pub required: bool,
    /// Default value
    pub default_value: Option<String>,
}

/// Email template categories
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum EmailTemplateCategory {
    /// Show announcements
    ShowAnnouncement,
    /// Newsletter
    Newsletter,
    /// Contact form responses
    ContactResponse,
    /// Booking confirmations
    BookingConfirmation,
    /// Booking reminders
    BookingReminder,
    /// Password reset
    PasswordReset,
    /// Welcome email
    Welcome,
    /// Custom template
    Custom,
}

// ============================================================================
// EMAIL MESSAGE
// ============================================================================

/// Email message to be sent
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailMessage {
    /// Recipient email address
    #[garde(email)]
    pub to: String,
    /// Recipient name (optional)
    pub to_name: Option<String>,
    /// CC recipients
    pub cc: Vec<String>,
    /// BCC recipients
    pub bcc: Vec<String>,
    /// Subject line
    pub subject: String,
    /// HTML body
    pub body_html: String,
    /// Plain text body
    pub body_text: String,
    /// Attachments
    pub attachments: Vec<EmailAttachment>,
    /// Template ID (if using template)
    pub template_id: Option<String>,
    /// Template variables (if using template)
    pub template_vars: Option<serde_json::Value>,
    /// Tags for tracking
    pub tags: Vec<String>,
    /// Metadata
    pub metadata: serde_json::Value,
}

/// Email attachment
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailAttachment {
    /// Attachment filename
    pub filename: String,
    /// Attachment content (base64 or URL)
    pub content: AttachmentContent,
    /// Content type
    pub content_type: String,
    /// Size in bytes
    pub size_bytes: i64,
}

/// Attachment content source
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AttachmentContent {
    /// Base64 encoded content
    Base64 { data: String },
    /// URL to file
    Url { url: String },
    /// Storage reference
    Storage { path: String },
}

// ============================================================================
// EMAIL CAMPAIGNS
// ============================================================================

/// Email campaign for mass sends
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailCampaign {
    /// Unique campaign ID
    pub id: String,
    /// Site this campaign belongs to
    pub site_id: String,
    /// Campaign name
    pub name: String,
    /// Campaign subject
    pub subject: String,
    /// Template to use
    pub template_id: String,
    /// Recipient list IDs
    pub recipient_lists: Vec<String>,
    /// Scheduled send time (null = send immediately)
    pub scheduled_at: Option<i64>,
    /// Campaign status
    pub status: CampaignStatus,
    /// Total recipients
    pub total_recipients: i32,
    /// Sent count
    pub sent_count: i32,
    /// Open count
    pub open_count: i32,
    /// Click count
    pub click_count: i32,
    /// Created timestamp
    pub created_at: i64,
}

/// Campaign status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum CampaignStatus {
    /// Campaign is being drafted
    Draft,
    /// Scheduled to send
    Scheduled,
    /// Currently sending
    Sending,
    /// Sent successfully
    Sent,
    /// Sending failed
    Failed,
    /// Cancelled
    Cancelled,
}

// ============================================================================
// EMAIL LISTS
// ============================================================================

/// Email list for campaigns
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailList {
    /// Unique list ID
    pub id: String,
    /// Site this list belongs to
    pub site_id: String,
    /// List name
    pub name: String,
    /// List description
    pub description: Option<String>,
    /// Subscriber count
    pub subscriber_count: i32,
    /// Is this list public?
    pub is_public: bool,
    /// Created timestamp
    pub created_at: i64,
}

/// Email list subscriber
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailSubscriber {
    /// Unique subscriber ID
    pub id: String,
    /// Email address
    #[garde(email)]
    pub email: String,
    /// Subscriber name
    pub name: Option<String>,
    /// Subscription status
    pub status: SubscriptionStatus,
    /// Lists this subscriber belongs to
    pub list_ids: Vec<String>,
    /// Custom fields
    pub custom_fields: serde_json::Value,
    /// Subscribed timestamp
    pub subscribed_at: i64,
}

/// Subscription status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionStatus {
    /// Active subscriber
    Subscribed,
    /// Pending confirmation
    Pending,
    /// Unsubscribed
    Unsubscribed,
    /// Bounced
    Bounced,
    /// Marked as spam
    Spam,
}

// ============================================================================
// EMAIL LOGGING & ANALYTICS
// ============================================================================

/// Email delivery log entry
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailLog {
    /// Unique log ID
    pub id: String,
    /// Site this log belongs to
    pub site_id: String,
    /// Related message ID
    pub message_id: String,
    /// Recipient email
    pub to_email: String,
    /// Subject
    pub subject: String,
    /// Provider used
    pub provider: String,
    /// Delivery status
    pub status: EmailDeliveryStatus,
    /// Provider message ID (for tracking)
    pub provider_message_id: Option<String>,
    /// Error message (if failed)
    pub error_message: Option<String>,
    /// Sent timestamp
    pub sent_at: i64,
    /// Delivered timestamp
    pub delivered_at: Option<i64>,
    /// Opened timestamp
    pub opened_at: Option<i64>,
    /// Clicked timestamp
    pub clicked_at: Option<i64>,
}

/// Email delivery status
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum EmailDeliveryStatus {
    /// Queued for sending
    Queued,
    /// Successfully sent
    Sent,
    /// Delivered to inbox
    Delivered,
    /// Opened by recipient
    Opened,
    /// Clicked by recipient
    Clicked,
    /// Bounced
    Bounced,
    /// Failed to send
    Failed,
    /// Marked as spam
    Spam,
}

// ============================================================================
// WEBHOOK INTEGRATIONS
// ============================================================================

/// Email event webhook configuration
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailWebhook {
    /// Unique webhook ID
    pub id: String,
    /// Site this webhook belongs to
    pub site_id: String,
    /// Webhook URL
    #[garde(url)]
    pub url: String,
    /// Events to trigger on
    pub events: Vec<EmailEvent>,
    /// Secret for HMAC signature
    pub secret: String,
    /// Is webhook active?
    pub is_active: bool,
}

/// Email events for webhooks
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum EmailEvent {
    /// Email was sent
    Sent,
    /// Email was delivered
    Delivered,
    /// Email was opened
    Opened,
    /// Link was clicked
    Clicked,
    /// Email bounced
    Bounced,
    /// Email failed
    Failed,
    /// User unsubscribed
    Unsubscribed,
}
