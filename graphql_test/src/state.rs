use async_graphql::{Value, dynamic::*};

pub struct AppState {
    pub graphql_schema: Schema,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let query = Object::new("Query").field(Field::new(
            "value",
            TypeRef::named_nn(TypeRef::INT),
            |_| FieldFuture::new(async move { Ok(Some(Value::from(100))) }),
        ));

        Ok(Self {
            graphql_schema: Schema::build(query.type_name(), None, None)
                .register(query)
                .finish()?,
        })
    }
}
