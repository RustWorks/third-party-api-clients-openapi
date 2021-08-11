use anyhow::Result;

use crate::Client;

pub struct Card {
    client: Client,
}

impl Card {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Card {
            client,
        }
    }

    /**
* List cards.
*
* This function performs a `GET` to the `/cards` endpoint.
*
* Retrieve all cards.
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
* * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
* * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
* * `user_id: &str`
* * `card_program_id: &str`
*/
pub async fn get_cards(
&self,
authorization: &str, start: &str, page_size: f64, user_id: &str, card_program_id: &str,
) -> Result<crate::types::GetCardsResponse> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !card_program_id.is_empty() { query_args.push(format!("card_program_id={}", card_program_id)); }
query_args.push(format!("page_size={}", page_size));
if !start.is_empty() { query_args.push(format!("start={}", start)); }
if !user_id.is_empty() { query_args.push(format!("user_id={}", user_id)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/cards?{}",
query);

self.client.get(&url, None).await
}

/**
* GET a card.
*
* This function performs a `GET` to the `/cards/<id>` endpoint.
*
* Retrieve a single card.
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn get_card(
&self,
authorization: &str,
) -> Result<crate::types::Cards> {
let url =
"/cards/<id>".to_string();
self.client.get(&url, None).await
}

/**
* Update card.
*
* This function performs a `PATCH` to the `/cards/<id>` endpoint.
*
* Update card details
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn patch_resources_cards(
&self,
authorization: &str,
body: &crate::types::PatchResourcesCardsRequest
) -> Result<()> {
let url =
"/cards/<id>".to_string();
self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Create a physical card.
*
* This function performs a `POST` to the `/cards/deferred/physical` endpoint.
*
* 
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn post_resources_physical(
&self,
authorization: &str,
body: &crate::types::PostResourcesPhysicalRequest
) -> Result<crate::types::TaskResponse> {
let url =
"/cards/deferred/physical".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Create a virtual card.
*
* This function performs a `POST` to the `/cards/deferred/virtual` endpoint.
*
* 
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn post_resources_virtual(
&self,
authorization: &str,
body: &crate::types::PostResourcesVirtualRequest
) -> Result<crate::types::TaskResponse> {
let url =
"/cards/deferred/virtual".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Delete a card.
*
* This function performs a `POST` to the `/cards/<id>/deferred/termination` endpoint.
*
* Terminates a card permanently.
*/
pub async fn post_resources_cards_termination(
&self,
body: &crate::types::PostResourcesCardsSuspensionRequest
) -> Result<crate::types::TaskResponse> {
let url =
"/cards/<id>/deferred/termination".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Suspend a card.
*
* This function performs a `POST` to the `/cards/<id>/deferred/suspension` endpoint.
*
* Suspends a card so that it is locked from use. The suspension is revertable.
*/
pub async fn post_resources_cards_suspension(
&self,
body: &crate::types::PostResourcesCardsSuspensionRequest
) -> Result<crate::types::TaskResponse> {
let url =
"/cards/<id>/deferred/suspension".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Removes a card's suspension.
*
* This function performs a `POST` to the `/cards/<id>/deferred/unsuspension` endpoint.
*
* Removes a card's suspension so that it may be used again.
*/
pub async fn post_resources_cards_unsuspension(
&self,
body: &crate::types::PostResourcesCardsSuspensionRequest
) -> Result<crate::types::TaskResponse> {
let url =
"/cards/<id>/deferred/unsuspension".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Get status of a deferred card task.
*
* This function performs a `GET` to the `/cards/deferred/status/<id>` endpoint.
*
* Gets status of a deferred task for cards
*
* **Parameters:**
*
* * `authorization: &str` -- The OAuth2 token header.
*/
pub async fn get_resources_cards_deferred(
&self,
authorization: &str,
) -> Result<crate::types::GetResourcesCardsDeferredResponse> {
let url =
"/cards/deferred/status/<id>".to_string();
self.client.get(&url, None).await
}


}