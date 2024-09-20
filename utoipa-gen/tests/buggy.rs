use utoipa::{OpenApi, ToSchema};

// notes:
//
// can check the code generation via
// cargo expand -p utoipa-gen --test buggy
// cargo test -p utoipa-gen --test buggy

#[test]
fn double_use_of_generics() {
    #![allow(unused)]

    #[derive(ToSchema)]
    struct MyStruct {
        field: String,
    };

    #[derive(ToSchema)]
    enum DoubleUseOfGenerics<T>
    where
        T: ToSchema,
    {
        Next(T),
        Done(T),
    }

    #[utoipa::path(
        get,
        path = "/handler",
        responses(
            (status = OK, body = DoubleUseOfGenerics<MyStruct>),
        )
    )]
    async fn handler() {}

    #[derive(OpenApi)]
    #[openapi(
        components(
            schemas(
                DoubleUseOfGenerics<MyStruct>
            )
        ),
        paths(
            handler
        )
    )]
    struct ApiDoc;

    let actual = ApiDoc::openapi()
        .to_pretty_json()
        .expect("ApiDoc is JSON serializable");
    println!("{actual}");

    let expected = include_str!("./testdata/double_use_of_generics");

    println!("{actual}");

    assert_eq!(expected.trim(), actual.trim());
}
