use crate::helpers::{handle_response, pagination_to_query};
use crate::responses::player_responses::{Player, SnapShot};
use crate::{ApiEndpoint, Pagination};
use reqwest::Error;

type Username = String;

enum PlayerEndPoints {
    Search(Username),
    Update,
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
    ) -> Result<Vec<Player>, Error> {
        let pagination_query = pagination_to_query(pagination);

        let full_url = format!(
            "{}{}",
            self.get_url(PlayerEndPoints::Search(username)),
            pagination_query
        );
        let result = self.client.get(full_url.as_str()).send().await;
        handle_response::<Vec<Player>>(result).await
    }

    pub async fn get_player_snap_shots(&self, username: Username) -> Result<Vec<SnapShot>, Error> {
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
    use crate::WomClient;
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
    async fn player_snapshots_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/Zezima/snapshots", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player_snapshots.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .get_player_snap_shots("Zezima".to_string())
            .await;

        mock.assert();
        println!("{:?}", result);
        assert!(result.is_ok());
        let snapshots = result.unwrap();
        assert_eq!(snapshots.len(), 1);
    }
}
