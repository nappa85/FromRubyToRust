use std::{future, ops::Deref};

use futures_util::{Stream, TryStreamExt};

use sea_orm::{DbErr, EntityTrait, Select};

use tokio::sync::OnceCell;

use crate::get_conn;

mod orm;
pub use orm::{ActiveModel, Column, Entity, Scope};

pub struct Model {
    inner: orm::Model,
    cache: OnceCell<Vec<Model>>,
}

impl From<orm::Model> for Model {
    fn from(inner: orm::Model) -> Self {
        Model {
            inner,
            cache: OnceCell::new(),
        }
    }
}

impl Deref for Model {
    type Target = orm::Model;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Model {
    /*
    def cached_method
        @cache ||= self.class.active.do_something_expensive(id: id)
    end
    */
    pub async fn cached_method(&self) -> Result<&[Model], DbErr> {
        self.cache
            .get_or_try_init(|| async {
                do_something_expensive(self.id, Some(Entity::find().active()))
                    .await?
                    .try_collect()
                    .await
            })
            .await
            .map(|v| v.deref())
    }
}

/*
    def self.do_something_expensive(id:)
        select do |model|
            model.metadata['parent_id'] == id
        end
    end
*/
pub async fn do_something_expensive(
    id: i32,
    query: Option<Select<Entity>>,
) -> Result<impl Stream<Item = Result<Model, DbErr>>, DbErr> {
    let query = query.unwrap_or_else(Entity::find);
    let conn = get_conn().await?;
    let stream = query.stream(conn).await?;
    Ok(stream
        .try_filter(move |model| future::ready(model.metadata.parent_id == Some(id)))
        .map_ok(Into::into))
}
