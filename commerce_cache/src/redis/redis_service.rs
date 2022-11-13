use std::time::Duration;
use async_trait::async_trait;
use commerce_command::error::{Error, Result};
use log::{error, info};
use redis::Connection;
use redis::RedisResult;
use crate::redis::cache_service::ICacheService;
///Redis缓存服务
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        info!("conncect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        info!("conncect redis success!");
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_connection();
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(Error::from(err));
        }
        return Ok(conn.unwrap());
    }
}

#[async_trait]
impl ICacheService for RedisService {
    async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        return self.set_string_ex(k, v, None).await;
    }

    async fn get_string(&self, k: &str) -> Result<String> {
        let mut conn = self.get_conn().await?;
        let result: RedisResult<Option<String>> =
            redis::cmd("GET").arg(&[k]).query(&mut conn);
        return match result {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => Err(Error::from(format!(
                "RedisService get_string({}) fail:{}",
                k,
                e.to_string()
            ))),
        };
    }

    ///set_string 自动过期
    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        let mut conn = self.get_conn().await?;
        return if ex.is_none() {
            match redis::cmd("SET").arg(&[k, v]).query(&mut conn){
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        } else {
            match redis::cmd("SET")
                .arg(&[k, v, "EX", &ex.unwrap().as_secs().to_string()])
                .query(&mut conn)
            {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService set_string_ex fail:{}",
                    e.to_string()
                ))),
            }
        };
    }

    ///set_string 自动过期
    async fn ttl(&self, k: &str) -> Result<i64> {
        let mut conn = self.get_conn().await?;
        return match redis::cmd("TTL").arg(&[k]).query(&mut conn) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisService ttl fail:{}",
                e.to_string()
            ))),
        };
    }
    ///set_string 自动过期
    async fn del(&self, k: &str) -> Result<i64> {
        let mut conn = self.get_conn().await?;
        //cmd("DEL").arg(key)
        return match redis::cmd("DEL").arg(&[k]).query(&mut conn){
            Ok(v) => Ok(v),
            Err(e) => Err(Error::from(format!(
                "RedisService del fail:{}",
                e.to_string()
            ))),
        };
    }
}
