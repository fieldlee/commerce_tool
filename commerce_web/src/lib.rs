#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cached;

//配置
pub mod config;
//配置
pub mod initialize;

use log::info;
use state::Container;

/*
    整个项目上下文ApplicationContext
    包括:
        ApplicationConfig 配置
        Database mongodb数据库
        Rbatis  mysql orm
        ServiceContext 服务上下文
        CasbinService 权限服务
*/

use crate::initialize::config::init_config;
use commerce_config::config::ApplicationConfig;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();


/*初始化环境上下文*/
pub async fn init_context() {
    print_banner();
    //第一步加载配置
    init_config().await;
    
    let commerce_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!(
        " - Local:   http://{}:{}",
        commerce_config
            .server()
            .host()
            .replace("0.0.0.0", "127.0.0.1"),
            commerce_config.server().port()
    );
}

fn print_banner() {
    let banner = r#"
     ____
    |      。   ———————     |                |     |        _____    ____
    |___   |   |            |                |     |       |        |
    |      |   |_______     |         —————— |     |       |_____   |____
    |      |   |            |         |      |     |       |        |
    |      |   |————————    |______   |_____ |     |____   |_____   |____
"#;
    println!("{}", banner);
}