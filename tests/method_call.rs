use rhai::{Engine, EvalAltResult, RegisterFn};

#[test]
fn test_method_call() -> Result<(), EvalAltResult> {
    #[derive(Clone)]
    struct TestStruct {
        x: i64,
    }

    impl TestStruct {
        fn update(&mut self) {
            self.x += 1000;
        }

        fn new() -> TestStruct {
            TestStruct { x: 1 }
        }
    }

    let mut engine = Engine::new();

    engine.register_type::<TestStruct>();

    engine.register_fn("update", TestStruct::update);
    engine.register_fn("new_ts", TestStruct::new);

    let ts = engine.eval::<TestStruct>("let x = new_ts(); x.update(); x")?;

    assert_eq!(ts.x, 1001);

    Ok(())
}