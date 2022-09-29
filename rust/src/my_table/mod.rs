use std::{
    future::{self, Future},
    ops::Deref,
    pin::Pin,
};

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
    pub async fn cached_method(&self) -> Result<&Vec<Model>, DbErr> {
        self.cache
            .get_or_try_init(|| async {
                do_something_expensive(self.id, Some(Entity::find().active()))
                    .await?
                    .try_collect()
                    .await
            })
            .await
    }
    pub async fn cached_method_trait(&self) -> Result<&Vec<Model>, DbErr> {
        self.cache
            .get_or_try_init(|| async {
                Entity::find()
                    .active()
                    .do_something_expensive(self.id)
                    .await?
                    .try_collect()
                    .await
            })
            .await
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
    Ok(stream.try_filter_map(move |model| {
        future::ready(Ok(
            (model.metadata.parent_id == Some(id)).then(|| model.into())
        ))
    }))
}

pub type ModelStream = Pin<Box<dyn Stream<Item = Result<Model, DbErr>>>>;

trait AsyncScope: Sized {
    fn do_something_expensive(
        self,
        id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<ModelStream, DbErr>>>>;
}

impl AsyncScope for Select<Entity> {
    /*
      def self.do_something_expensive(id:)
          select do |model|
              model.metadata['parent_id'] == id
          end
      end
    */
    fn do_something_expensive(
        self,
        id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<ModelStream, DbErr>>>> {
        Box::pin(async move {
            let conn = get_conn().await?;
            let stream = self.stream(conn).await?;
            Ok(Box::pin(stream.try_filter_map(move |model| {
                future::ready(Ok(
                    (model.metadata.parent_id == Some(id)).then(|| model.into())
                ))
            })) as ModelStream)
        })
    }
}
