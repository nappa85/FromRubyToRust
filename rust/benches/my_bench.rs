use criterion::{
    Criterion, {criterion_group, criterion_main},
};

use sea_orm::EntityTrait;

use tokio::runtime::Builder;

// crate
use rust::{get_conn, my_table};

// benchmark
async fn cached_method() {
    let conn = get_conn()
        .await
        .expect("Retrieving database connection failed");
    let first = my_table::Entity::find()
        .one(conn)
        .await
        .expect("Sqlite query failed")
        .expect("Unexpected empty table");
    my_table::Model::from(first)
        .cached_method()
        .await
        .expect("Method call failed");
}

// benchmark
async fn cached_method_trait() {
    let conn = get_conn()
        .await
        .expect("Retrieving database connection failed");
    let first = my_table::Entity::find()
        .one(conn)
        .await
        .expect("Sqlite query failed")
        .expect("Unexpected empty table");
    my_table::Model::from(first)
        .cached_method_trait()
        .await
        .expect("Method call failed");
}

fn from_elem(c: &mut Criterion) {
    c.bench_function("cached_method", |b| {
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Creating runtime failed");

        b.to_async(&rt).iter(cached_method);
    });
    c.bench_function("cached_method_trait", |b| {
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Creating runtime failed");

        b.to_async(&rt).iter(cached_method_trait);
    });
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
