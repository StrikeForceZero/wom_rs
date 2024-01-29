use crate::helpers::{handle_response, pagination_to_query};
use crate::responses::player_responses::Player;
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
    Snapshots,
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
            _ => format!("{}", ApiEndpoint::Player.as_str()),
        }
    }
}

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

    pub fn get_url(&self, endpoint: PlayerEndPoints) -> String {
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
}

#[cfg(test)]
mod tests {
    use crate::WomClient;
    use httpmock::prelude::*;

    const BASE_URL: &str = "/players";
    #[tokio::test]
    async fn player_search_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/players/search")
                .query_param_exists("username");
            then.status(200)
                .header("content-type", "application/json")
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
}
