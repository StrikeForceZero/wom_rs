use crate::helpers::{handle_response, pagination_to_query};
use crate::models::global_types::{PlayerId, Username};
use crate::models::player::{
    Achievement, AchievementProgress, AssertPlayerType, Player, PlayerDetails, SnapShot,
};
use crate::{ApiEndpoint, Pagination};
use anyhow::Result;

enum PlayerEndPoints {
    Search(Username),
    Update(Username),
    AssertType(Username),
    Details(Username),
    DetailsById(PlayerId),
    Achievements(Username),
    AchievementsProgress(Username),
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
            PlayerEndPoints::AssertType(username) => {
                format!("{}/{}/assert-type", ApiEndpoint::Player.as_str(), username)
            }
            PlayerEndPoints::Details(username) => {
                format!("{}/{}", ApiEndpoint::Player.as_str(), username)
            }
            PlayerEndPoints::DetailsById(player_id) => {
                format!("{}/id/{}", ApiEndpoint::Player.as_str(), player_id)
            }
            PlayerEndPoints::Achievements(username) => {
                format!("{}/{}/achievements", ApiEndpoint::Player.as_str(), username)
            }
            PlayerEndPoints::AchievementsProgress(username) => {
                format!(
                    "{}/{}/achievements/progress",
                    ApiEndpoint::Player.as_str(),
                    username
                )
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

    /// Search for players by username, takes an optional pagination parameter
    /// [Player Search](https://docs.wiseoldman.net/players-api/player-endpoints#search)
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

    /// Sends a request to update the players hiscore data from the offical hiscores
    /// [Player Update](https://docs.wiseoldman.net/players-api/player-endpoints#update-a-player)
    pub async fn update_player(&self, username: Username) -> Result<PlayerDetails, anyhow::Error> {
        let full_url = self.get_url(PlayerEndPoints::Update(username));
        let result = self.client.post(full_url.as_str()).send().await;
        handle_response::<PlayerDetails>(result).await
    }

    /// Asserts (and attempts to fix, if necessary) a player's game-mode type.
    /// [Assert Player Type](https://docs.wiseoldman.net/players-api/player-endpoints#assert-player-type)
    pub async fn assert_player_type(
        &self,
        username: Username,
    ) -> Result<AssertPlayerType, anyhow::Error> {
        let result = self
            .client
            .post(self.get_url(PlayerEndPoints::AssertType(username)).as_str())
            .send()
            .await;
        handle_response::<AssertPlayerType>(result).await
    }

    /// Get a player's details by username
    /// [Player Details](https://docs.wiseoldman.net/players-api/player-endpoints#get-player-details)
    pub async fn get_player_details(
        &self,
        username: Username,
    ) -> Result<PlayerDetails, anyhow::Error> {
        let result = self
            .client
            .get(self.get_url(PlayerEndPoints::Details(username)).as_str())
            .send()
            .await;
        handle_response::<PlayerDetails>(result).await
    }

    /// Get a player's details by player id
    /// [Player Details](https://docs.wiseoldman.net/players-api/player-endpoints#get-player-details-by-id)
    pub async fn get_player_details_by_id(
        &self,
        player_id: PlayerId,
    ) -> Result<PlayerDetails, anyhow::Error> {
        let result = self
            .client
            .get(
                self.get_url(PlayerEndPoints::DetailsById(player_id))
                    .as_str(),
            )
            .send()
            .await;
        handle_response::<PlayerDetails>(result).await
    }

    /// Get a player's achievements by username
    /// [Player Achievements](https://docs.wiseoldman.net/players-api/player-endpoints#get-player-achievements)
    pub async fn get_player_achievements(
        &self,
        username: Username,
    ) -> Result<Vec<Achievement>, anyhow::Error> {
        let result = self
            .client
            .get(
                self.get_url(PlayerEndPoints::Achievements(username))
                    .as_str(),
            )
            .send()
            .await;
        handle_response::<Vec<Achievement>>(result).await
    }

    /// Get a player's achievements progress by username
    /// [Player Achievements Progress](https://docs.wiseoldman.net/players-api/player-endpoints#get-player-achievement-progress)
    pub async fn get_player_achievement_progress(
        &self,
        username: Username,
    ) -> Result<Vec<AchievementProgress>, anyhow::Error> {
        let result = self
            .client
            .get(
                self.get_url(PlayerEndPoints::AchievementsProgress(username))
                    .as_str(),
            )
            .send()
            .await;
        handle_response::<Vec<AchievementProgress>>(result).await
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
                .body_from_file("./tests/mocks/player/player_search.json");
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
                .body_from_file("./tests/mocks/player/player_search.json");
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
                .body_from_file("./tests/mocks/player/player_details.json");
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
    async fn player_assert_type_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path(format!("{}/Zezima/assert-type", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player/player_assert_type.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .assert_player_type("Zezima".to_string())
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
                .body_from_file("./tests/mocks/player/player_snapshots.json");
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

    #[tokio::test]
    async fn player_details_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/IFat%20Fingers", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player/player_details.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .get_player_details("IFat Fingers".to_string())
            .await;

        mock.assert();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn player_details_by_id_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path(format!("{}/id/1", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player/player_details.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client.player_client.get_player_details_by_id(1).await;

        mock.assert();
        assert!(result.is_ok());
        let player_details = result.unwrap();
        assert_eq!(player_details.id, 1);
    }

    #[tokio::test]
    async fn player_achievements_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/IFat%20Fingers/achievements", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player/player_achievements.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .get_player_achievements("IFat Fingers".to_string())
            .await;

        mock.assert();
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 2);
    }

    #[tokio::test]
    async fn player_achievements_progress_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/IFat%20Fingers/achievements/progress", BASE_URL));
            then.status(200)
                .header(CONTENT_TYPE, APPLICATION_JSON)
                .body_from_file("./tests/mocks/player/player_achievement_progress.json");
        });

        let wom_client = WomClient::new_with_base_url(server.base_url().to_string(), None);
        let result = wom_client
            .player_client
            .get_player_achievement_progress("IFat Fingers".to_string())
            .await;

        mock.assert();
        assert!(result.is_ok());
        let achievements_progress = result.unwrap();
        assert_eq!(achievements_progress.len(), 4);
    }
}
