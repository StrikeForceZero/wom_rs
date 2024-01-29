use crate::helpers::{handle_empty_response, handle_response, pagination_to_query};
use crate::responses::player_responses::{Player, PlayerDetails, SnapShot};
use crate::{ApiEndpoint, Pagination};
use anyhow::Result;
use reqwest::{Error, StatusCode};

type Username = String;

enum PlayerEndPoints {
    Search(Username),
    Update(Username),
    AssertType,
    Details,
    DetailsById,
    Achievements,
    AchievementsProgress,
    Competitions,
    CompetitionsStandings,
    GroupMembership,
    Gains,
    Records,
    Snapshots(Username),
    SnapshotsTimeline,
    NameChange,
    Archives,
}

impl PlayerEndPoints {
    fn url(&self) -> String {
        match self {
            PlayerEndPoints::Search(username) => {
                format!(
                    "{}/search?username={}",
                    ApiEndpoint::Player.as_str(),
                    username
                )
            }
            PlayerEndPoints::Update(username) => {
                format!("{}/{}", ApiEndpoint::Player.as_str(), username)
            }
            PlayerEndPoints::Snapshots(username) => {
                format!("{}/{}/snapshots", ApiEndpoint::Player.as_str(), username)
            }
            _ => format!("{}", ApiEndpoint::Player.as_str()),
        }
    }
}

/// Handles all requests to the [Player Endpoints](https://docs.wiseoldman.net/players-api/player-endpoints)
pub struct PlayerClient {
    client: reqwest::Client,
    base_url: String,
}

impl PlayerClient {
    pub fn new(client: reqwest::Client, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    fn get_url(&self, endpoint: PlayerEndPoints) -> String {
        format!("{}{}", self.base_url, endpoint.url())
    }

    pub async fn search(
        &self,
        username: Username,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Player>, anyhow::Error> {
        let pagination_query = pagination_to_query(pagination);

        let full_url = format!(
            "{}{}",
            self.get_url(PlayerEndPoints::Search(username)),
            pagination_query
        );
        let result = self.client.get(full_url.as_str()).send().await;
        handle_response::<Vec<Player>>(result).await
    }

    pub async fn update_player(&self, username: Username) -> Result<PlayerDetails, anyhow::Error> {
        let full_url = self.get_url(PlayerEndPoints::Update(username));
        let result = self.client.post(full_url.as_str()).send().await;
        handle_response::<PlayerDetails>(result).await
    }

    pub async fn get_player_snap_shots(
        &self,
        username: Username,
    ) -> Result<Vec<SnapShot>, anyhow::Error> {
        let result = self
            .client
            .get(self.get_url(PlayerEndPoints::Snapshots(username)).as_str())
            .send()
            .await;
        handle_response::<Vec<SnapShot>>(result).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pagination, WomClient};
    use httpmock::prelude::*;
    use httpmock::{Mock, Then, When};

    const BASE_URL: &str = "/players";
    const CONTENT_TYPE: &str = "content-type";
    const APPLICATION_JSON: &str = "application/json";

    #[tokio::test]
    async fn player_search_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/search", BASE_URL))
                .query_param_exists("username");
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player_search.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .search("Zezima".to_string(), None)
            .await;

        mock.assert();
        assert!(result.is_ok());
        let players = result.unwrap();
        assert_eq!(players.len(), 2);
    }

    #[tokio::test]
    async fn player_search_check_pagination_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/search", BASE_URL))
                .query_param("limit", "10")
                .query_param("offset", "10")
                .query_param_exists("username");
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player_search.json");
        });

        let pagination = Some(Pagination {
            limit: Some(10),
            offset: Some(10),
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .search("Zezima".to_string(), pagination)
            .await;

        mock.assert();
        assert!(result.is_ok());
        let players = result.unwrap();
        assert_eq!(players.len(), 2);
    }

    #[tokio::test]
    async fn player_update_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path(format!("{}/Zezima", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player_details.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .update_player("Zezima".to_string())
            .await;
        println!("{:?}", result);

        mock.assert();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn player_snapshots_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/IFat%20Fingers/snapshots", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player_snapshots.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .get_player_snap_shots("IFat Fingers".to_string())
            .await;

        mock.assert();
        println!("{:?}", result);
        assert!(result.is_ok());
        let snapshots = result.unwrap();
        assert_eq!(snapshots.len(), 1);
    }
}
