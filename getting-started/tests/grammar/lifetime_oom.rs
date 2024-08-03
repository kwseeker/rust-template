/// 测试使用 lifetime specifier，是否会导致内存泄漏 （并不会）
/// 提出这个问题主要是之前对生命周期标注有误解，以为生命周期标准会修改包含引用的变量的生命周期，其实并不会，
/// 生命周期在函数参数列表以及结构体字段中只是一个约束标识，并不会修改实际的作用域，其内存该释放时还是会释放。
#[test]
fn lifetime_oom() {
    struct Config {
        name: String,
    }

    struct Container<'c> {  // 这里声明一个生命周期'c
        config: &'c Config, // 这里'c只是个约束条件，要求config引用指向Config对象的生命周期要大于'c，否则可能会产生悬垂引用，
                            // 即'c是小于被引用Config对象生命周期的一个约束条件，能否理解为,假设config和被引用的Config对象的实际生命周期分别是’m、'n，'c('m,'n) 执行'm是否小于'n的判断？
        container: Vec<u8>,
    }

    impl Container<'_> {
        // fn from_config<'a>(config: &'a Config) -> Container<'a> {
        fn from_config(config: &Config) -> Container {
            Container {
                config,
                container: vec![0u8; 1024 * 1024],
            }
        }
    }

    let config = Config {
        name: "test".to_string(),
    };
    loop {
        Container::from_config(&config);    //config的生命周期比这里创建的Container实例的生命周期长，所以对'c生命周期约束是合法的
    }
}