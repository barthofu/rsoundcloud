use async_trait::async_trait;
use http::Query;
use models::{comment::Comment, conversation::Conversation, message::Message, playlist::BasicAlbumPlaylist, track::BasicTrack, user::{User, UserEmail}, web_profile::WebProfile, LikeItem, RepostItem, StreamItem};

use crate::{client::SoundCloudClient, errors::{convert_404_to_invalid_id, ClientError}, need_authentication, utils::schemas::ResourceId, ClientResult};

use super::{convert_collection, convert_like_items, convert_repost_items, convert_result, convert_stream_items, misc::MiscApi};

#[async_trait]
pub trait UsersApi {

    /// Returns the user with the given user_id.
    async fn get_user(&self, user_id: ResourceId) -> ClientResult<User>;

    /// Returns the user with the given username.
    async fn get_user_by_username(&self, username: &str) -> ClientResult<User>;

    /// Get comments by this user.
    async fn get_user_comments(&self, user_id: ResourceId) -> ClientResult<Vec<Comment>>;

    /// Get user's email addresses. 
    /// Requires authentication.
    async fn get_user_emails(&self, user_id: ResourceId) -> ClientResult<Vec<UserEmail>>;

    /// Get profiles featured by this user.
    async fn get_user_featured_profiles(&self, user_id: ResourceId) -> ClientResult<Vec<User>>;

    /// Get followers of this user.
    async fn get_user_followers(&self, user_id: ResourceId) -> ClientResult<Vec<User>>;

    /// Get users followed by this user.
    async fn get_user_followings(&self, user_id: ResourceId) -> ClientResult<Vec<User>>;

    /// Get user's likes.
    async fn get_user_likes(&self, user_id: ResourceId) -> ClientResult<Vec<LikeItem>>;

    /// Get artists related to this user.
    async fn get_user_related_artists(&self, user_id: ResourceId) -> ClientResult<Vec<User>>;

    /// Get user's reposts.
    async fn get_user_reposts(&self, user_id: ResourceId) -> ClientResult<Vec<RepostItem>>;
    
    /// Get user's streams    
    async fn get_user_streams(&self, user_id: ResourceId) -> ClientResult<Vec<StreamItem>>;
    
    /// Get user's tracks.
    async fn get_user_tracks(&self, user_id: ResourceId) -> ClientResult<Vec<BasicTrack>>;

    /// Get user's popular tracks
    async fn get_user_popular_tracks(&self, user_id: ResourceId) -> ClientResult<Vec<BasicTrack>>;

    /// Get user's albums.
    async fn get_user_albums(&self, user_id: ResourceId) -> ClientResult<Vec<BasicAlbumPlaylist>>;

    /// Get user's playlists.
    async fn get_user_playlists(&self, user_id: ResourceId) -> ClientResult<Vec<BasicAlbumPlaylist>>;

    /// Get user's links.
    async fn get_user_links(&self, user_id: ResourceId) -> ClientResult<Vec<WebProfile>>;

    /// Get user's conversations
    /// ! Requires authentication.
    async fn get_user_conversations(&self, user_id: ResourceId) -> ClientResult<Vec<Conversation>>;

    /// Get conversation messages.
    /// ! Requires authentication.
    async fn get_conversation_messages(&self, user_id: ResourceId, conversation_id: u64) -> ClientResult<Vec<Message>>;

    /// Get unread conversations
    /// ! Requires authentication.
    async fn get_unread_conversations(&self, user_id: ResourceId) -> ClientResult<Vec<Conversation>>;    

    // Utils

    /// Extracts the user id from the given user_id.
    async fn extract_user_id(&self, user_id: ResourceId) -> ClientResult<u64>;
}

#[async_trait]
impl UsersApi for SoundCloudClient {

    async fn get_user(&self, user_id: ResourceId) -> ClientResult<User> {
        let uri = format!("/users/{}", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_result(&result)
    }

    async fn get_user_by_username(&self, username: &str) -> ClientResult<User> {
        let resource = self.resolve(format!("https://soundcloud.com/{}", username).as_str()).await?;
        match resource {
            models::SearchItem::User(user) => Ok(user),
            _ => Err(ClientError::NotFound)
        }
    }

    async fn get_user_comments(&self, user_id: ResourceId) -> ClientResult<Vec<Comment>> {
        let uri = format!("/users/{}/comments", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_emails(&self, user_id: ResourceId) -> ClientResult<Vec<UserEmail>> {
        need_authentication!(self);
        let uri = format!("/users/{}/emails", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_featured_profiles(&self, user_id: ResourceId) -> ClientResult<Vec<User>> {
        let uri = format!("/users/{}/featured-profiles", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_followers(&self, user_id: ResourceId) -> ClientResult<Vec<User>> {
        let uri = format!("/users/{}/followers", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_followings(&self, user_id: ResourceId) -> ClientResult<Vec<User>> {
        let uri = format!("/users/{}/followings", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_likes(&self, user_id: ResourceId) -> ClientResult<Vec<LikeItem>> {
        let uri = format!("/users/{}/likes", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_like_items(&result)
    }

    async fn get_user_related_artists(&self, user_id: ResourceId) -> ClientResult<Vec<User>> {
        let uri = format!("/users/{}/relatedartists", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_reposts(&self, user_id: ResourceId) -> ClientResult<Vec<RepostItem>> {
        let uri = format!("/stream/users/{}/reposts", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_repost_items(&result)
    }

    async fn get_user_streams(&self, user_id: ResourceId) -> ClientResult<Vec<StreamItem>> {
        let uri = format!("/stream/users/{}", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_stream_items(&result)
    }

    async fn get_user_tracks(&self, user_id: ResourceId) -> ClientResult<Vec<BasicTrack>> {
        let uri = format!("/users/{}/tracks", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_popular_tracks(&self, user_id: ResourceId) -> ClientResult<Vec<BasicTrack>> {
        let uri = format!("/users/{}/toptracks", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_albums(&self, user_id: ResourceId) -> ClientResult<Vec<BasicAlbumPlaylist>> {
        let uri = format!("/users/{}/albums", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_playlists(&self, user_id: ResourceId) -> ClientResult<Vec<BasicAlbumPlaylist>> {
        let uri = format!("/users/{}/playlists_without_albums", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_user_links(&self, user_id: ResourceId) -> ClientResult<Vec<WebProfile>> {
        let uri = format!("/users/soundcloud:users:{}/web-profiles", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_result(&result) // This endpoint returns a classic array, no need for collection conversion.
    }

    async fn get_user_conversations(&self, user_id: ResourceId) -> ClientResult<Vec<Conversation>> {
        need_authentication!(self);
        let uri = format!("/users/{}/conversations", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_conversation_messages(&self, user_id: ResourceId, conversation_id: u64) -> ClientResult<Vec<Message>> {
        need_authentication!(self);
        let uri = format!("/users/{}/conversations/{}/messages", self.extract_user_id(user_id).await?, conversation_id);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    async fn get_unread_conversations(&self, user_id: ResourceId) -> ClientResult<Vec<Conversation>> {
        need_authentication!(self);
        let uri = format!("/users/{}/conversations/unread", self.extract_user_id(user_id).await?);
        
        let result = self.api_get(&uri, Query::new()).await.map_err(convert_404_to_invalid_id)?;
        convert_collection(&result)
    }

    // Utils

    async fn extract_user_id(&self, user_id: ResourceId) -> ClientResult<u64> {
        match user_id {
            ResourceId::Id(id) => Ok(id),
            ResourceId::Url(url) => {
                self.resolve_user(&url).await
                    .map(|u| u.user.id)
            },
            ResourceId::Uri(uri) => {
                self.resolve_user(&format!("https://soundcloud.com/{}", uri)).await
                    .map(|u| u.user.id)
            }
        }
    }
}