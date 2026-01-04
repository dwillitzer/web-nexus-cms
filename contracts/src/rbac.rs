// RBAC & Permissions Module
// Add this to contracts/src/lib.rs or separate module

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashSet;

// ============================================================================
// PERMISSIONS SYSTEM
// ============================================================================

/// Granular permissions for fine-grained access control
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    // User Management
    CreateUser,
    EditUser,
    DeleteUser,
    AssignRoles,

    // Site Management
    CreateSite,
    EditSite,
    DeleteSite,
    DeploySite,

    // Content Permissions
    CreateShow,
    EditShow,
    DeleteShow,
    PublishShow,

    CreateSong,
    EditSong,
    DeleteSong,

    CreatePhoto,
    EditPhoto,
    DeletePhoto,
    PublishPhoto,

    CreateVideo,
    EditVideo,
    DeleteVideo,
    PublishVideo,

    CreatePost,
    EditPost,
    DeletePost,
    PublishPost,

    // Media Upload
    UploadPhoto,
    UploadVideo,
    UploadAudio,
    DeleteMedia,

    // Comments & Interaction
    ModerateComments,
    DeleteComment,

    // Analytics & Reports
    ViewAnalytics,
    ExportReports,

    // Settings
    EditSettings,
    ManageApiKeys,
    ManageWebhooks,

    // Email & Communications
    SendEmail,
    ManageTemplates,
    ViewEmailLogs,

    // AI Features
    UseAIAssistant,
    TrainAIModel,
    ViewAIInsights,
}

/// Permission set for a role
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissions {
    pub role: Role,
    pub permissions: HashSet<Permission>,
}

/// Check if user has specific permission
#[wasm_bindgen]
pub fn has_permission(user: &User, permission: Permission) -> bool {
    match permission {
        // Admins have everything
        _ if user.is_admin() => true,

        // Content editors
        Permission::CreateShow | Permission::EditShow | Permission::DeleteShow
        | Permission::CreateSong | Permission::EditSong | Permission::DeleteSong
        | Permission::CreatePhoto | Permission::EditPhoto | Permission::DeletePhoto
        | Permission::CreateVideo | Permission::EditVideo | Permission::DeleteVideo
        | Permission::CreatePost | Permission::EditPost | Permission::DeletePost
            if user.can_edit_content() => true,

        // Media uploads
        Permission::UploadPhoto | Permission::UploadVideo | Permission::UploadAudio
            if user.can_upload_media() => true,

        // View analytics for any authenticated user
        Permission::ViewAnalytics if user.status == UserStatus::Active => true,

        _ => false,
    }
}

/// Check if user has ANY of the specified permissions
#[wasm_bindgen]
pub fn has_any_permission(user: &User, permissions: &[Permission]) -> bool {
    permissions.iter().any(|p| has_permission(user, *p))
}

/// Check if user has ALL of the specified permissions
#[wasm_bindgen]
pub fn has_all_permissions(user: &User, permissions: &[Permission]) -> bool {
    permissions.iter().all(|p| has_permission(user, *p))
}

// ============================================================================
// RESOURCE-LEVEL PERMISSIONS
// ============================================================================

/// Access control for specific resources
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAccess {
    pub resource_type: String,
    pub resource_id: String,
    pub user_id: String,
    pub permissions: HashSet<Permission>,
}

/// Check if user can access specific resource
#[wasm_bindgen]
pub fn can_access_resource(
    user: &User,
    resource_type: &str,
    resource_id: &str,
    access_lists: &[ResourceAccess],
) -> bool {
    // Admins can access everything
    if user.is_admin() {
        return true;
    }

    // Check site-specific access
    if let Some(Role::SiteEditor { site_id }) = user.roles.iter().find(|r| {
        matches!(r, Role::SiteEditor { .. })
    }) {
        if resource_type == "site" && resource_id == site_id {
            return true;
        }
    }

    // Check explicit access lists
    access_lists
        .iter()
        .any(|acl| {
            acl.user_id == user.id
                && acl.resource_type == resource_type
                && acl.resource_id == resource_id
        })
}

// ============================================================================
// ROLE PERMISSIONS MAPPING
// ============================================================================

impl Role {
    /// Get all permissions for this role
    pub fn permissions(&self) -> HashSet<Permission> {
        match self {
            Role::Admin => {
                let mut perms = HashSet::new();
                // Admins have all permissions
                perms.insert(Permission::CreateUser);
                perms.insert(Permission::EditUser);
                perms.insert(Permission::DeleteUser);
                perms.insert(Permission::AssignRoles);
                perms.insert(Permission::CreateSite);
                perms.insert(Permission::EditSite);
                perms.insert(Permission::DeleteSite);
                perms.insert(Permission::DeploySite);
                perms.insert(Permission::CreateShow);
                perms.insert(Permission::EditShow);
                perms.insert(Permission::DeleteShow);
                perms.insert(Permission::PublishShow);
                perms.insert(Permission::CreateSong);
                perms.insert(Permission::EditSong);
                perms.insert(Permission::DeleteSong);
                perms.insert(Permission::CreatePhoto);
                perms.insert(Permission::EditPhoto);
                perms.insert(Permission::DeletePhoto);
                perms.insert(Permission::PublishPhoto);
                perms.insert(Permission::CreateVideo);
                perms.insert(Permission::EditVideo);
                perms.insert(Permission::DeleteVideo);
                perms.insert(Permission::PublishVideo);
                perms.insert(Permission::CreatePost);
                perms.insert(Permission::EditPost);
                perms.insert(Permission::DeletePost);
                perms.insert(Permission::PublishPost);
                perms.insert(Permission::UploadPhoto);
                perms.insert(Permission::UploadVideo);
                perms.insert(Permission::UploadAudio);
                perms.insert(Permission::DeleteMedia);
                perms.insert(Permission::ModerateComments);
                perms.insert(Permission::DeleteComment);
                perms.insert(Permission::ViewAnalytics);
                perms.insert(Permission::ExportReports);
                perms.insert(Permission::EditSettings);
                perms.insert(Permission::ManageApiKeys);
                perms.insert(Permission::ManageWebhooks);
                perms.insert(Permission::SendEmail);
                perms.insert(Permission::ManageTemplates);
                perms.insert(Permission::ViewEmailLogs);
                perms.insert(Permission::UseAIAssistant);
                perms.insert(Permission::TrainAIModel);
                perms.insert(Permission::ViewAIInsights);
                perms
            }

            Role::Content => {
                let mut perms = HashSet::new();
                perms.insert(Permission::CreateShow);
                perms.insert(Permission::EditShow);
                perms.insert(Permission::DeleteShow);
                perms.insert(Permission::CreateSong);
                perms.insert(Permission::EditSong);
                perms.insert(Permission::DeleteSong);
                perms.insert(Permission::CreatePhoto);
                perms.insert(Permission::EditPhoto);
                perms.insert(Permission::DeletePhoto);
                perms.insert(Permission::CreateVideo);
                perms.insert(Permission::EditVideo);
                perms.insert(Permission::DeleteVideo);
                perms.insert(Permission::CreatePost);
                perms.insert(Permission::EditPost);
                perms.insert(Permission::DeletePost);
                perms.insert(Permission::UploadPhoto);
                perms.insert(Permission::UploadVideo);
                perms.insert(Permission::UploadAudio);
                perms.insert(Permission::ViewAnalytics);
                perms.insert(Permission::UseAIAssistant);
                perms
            }

            Role::Media => {
                let mut perms = HashSet::new();
                perms.insert(Permission::UploadPhoto);
                perms.insert(Permission::UploadVideo);
                perms.insert(Permission::UploadAudio);
                perms.insert(Permission::DeleteMedia);
                perms
            }

            Role::ReadOnly => {
                let mut perms = HashSet::new();
                perms.insert(Permission::ViewAnalytics);
                perms
            }

            Role::SiteEditor { .. } => {
                let mut perms = HashSet::new();
                perms.insert(Permission::CreateShow);
                perms.insert(Permission::EditShow);
                perms.insert(Permission::DeleteShow);
                perms.insert(Permission::CreateSong);
                perms.insert(Permission::EditSong);
                perms.insert(Permission::DeleteSong);
                perms.insert(Permission::CreatePhoto);
                perms.insert(Permission::EditPhoto);
                perms.insert(Permission::DeletePhoto);
                perms.insert(Permission::CreatePost);
                perms.insert(Permission::EditPost);
                perms.insert(Permission::DeletePost);
                perms.insert(Permission::UploadPhoto);
                perms.insert(Permission::UploadVideo);
                perms.insert(Permission::ViewAnalytics);
                perms
            }
        }
    }
}
