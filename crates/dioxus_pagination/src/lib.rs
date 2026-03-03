use std::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub number_of_items: u64,
    pub number_of_pages: u64,
    pub items: Vec<T>,
}

impl<T> IntoIterator for Pagination<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> Pagination<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }
}

pub trait PaginationIter<T> {
    fn items_iter(&self) -> dioxus::Result<Iter<'_, T>>;
}

impl<T> PaginationIter<T> for &dioxus::Result<Pagination<T>> {
    fn items_iter(&self) -> dioxus::Result<Iter<'_, T>> {
        self.as_ref().map(|i| i.iter()).map_err(|e| e.clone())
    }
}

#[cfg(feature = "server")]
mod server {
    use sea_orm::compound::EntityLoaderPaginator;
    use sea_orm::{ConnectionTrait, DbErr, EntityLoaderTrait, EntityTrait};
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::Pagination;

    impl<T: Serialize + DeserializeOwned> Pagination<T> {
        pub async fn load_page<
            'db,
            C: ConnectionTrait,
            E: EntityTrait,
            EL: EntityLoaderTrait<E>>(
            pagination: EntityLoaderPaginator<'db, C, E, EL>,
            page: u64,
            transform: impl Fn(EL::ModelEx) -> T,
        ) -> Result<Pagination<T>, DbErr> {
            let nums = pagination.num_items_and_pages().await?;
            let page = pagination.fetch_page(page).await?;

            Ok(Pagination {
                number_of_items: nums.number_of_items,
                number_of_pages: nums.number_of_pages,
                items: page.into_iter().map(transform).collect(),
            })
        }
    }
}
