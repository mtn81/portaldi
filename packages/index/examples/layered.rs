use async_trait::async_trait;
use portaldi::*;

#[tokio::main]
async fn main() {
    use app::*;

    let created = HogeCreateCmd::di().execute("test").await;

    let found = HogeQuery::di().load(created.id).await;

    assert_eq!(created, found)
}

mod app {
    use super::*;
    use super::{domain::*, repo::*};

    #[derive(DIPortal)]
    pub struct HogeCreateCmd {
        hoge_service: DI<HogeService>,
        hoge_repo: DI<dyn HogeRepository>,
    }

    impl HogeCreateCmd {
        pub async fn execute(&self, name: &str) -> Hoge {
            let new_hoge = self.hoge_service.create(name).await;
            self.hoge_repo.save(&new_hoge).await;
            new_hoge
        }
    }

    #[derive(DIPortal)]
    pub struct HogeQuery {
        hoge_repo: DI<dyn HogeRepository>,
    }

    impl HogeQuery {
        pub async fn load(&self, id: u8) -> Hoge {
            self.hoge_repo
                .get(id)
                .await
                .expect(format!("hoge not found for {}", id).as_str())
        }
    }
}
mod domain {
    use super::*;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Hoge {
        pub id: u8,
        pub name: String,
    }

    #[derive(DIPortal)]
    pub struct HogeService {}

    impl HogeService {
        pub async fn create(&self, name: &str) -> Hoge {
            Hoge {
                id: 1,
                name: name.into(),
            }
        }
    }

    #[async_trait]
    pub trait HogeRepository: Send + Sync {
        async fn save(&self, hoge: &Hoge) -> ();
        async fn get(&self, id: u8) -> Option<Hoge>;
    }
}
mod repo {
    use super::*;
    use crate::domain::*;
    use std::{collections::HashMap, sync::Mutex};

    #[derive(Default)]
    struct InMemoryHogeRepository {
        datas: Mutex<HashMap<u8, Hoge>>,
    }

    #[portaldi::provider(HogeRepository)]
    impl<'a> DIPortal for InMemoryHogeRepository {
        fn create_for_di(_container: &DIContainer) -> Self {
            InMemoryHogeRepository::default()
        }
    }

    #[async_trait]
    impl HogeRepository for InMemoryHogeRepository {
        async fn save(&self, hoge: &Hoge) -> () {
            self.datas.lock().unwrap().insert(hoge.id, hoge.clone());
        }
        async fn get(&self, id: u8) -> Option<Hoge> {
            self.datas.lock().unwrap().get(&id).map(|v| v.clone())
        }
    }
}
