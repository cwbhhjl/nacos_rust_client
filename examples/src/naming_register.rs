use nacos_rust_client::client::naming_client::{ServiceInstanceKey, InstanceDefaultListener};
use std::sync::Arc;

use std::time::Duration;

use nacos_rust_client::client::{HostInfo, AuthInfo, naming_client::{NamingClient, Instance,QueryInstanceListParams}};



#[tokio::main]
async fn main(){
    //std::env::set_var("RUST_LOG","INFO");
    std::env::set_var("RUST_LOG","INFO");
    env_logger::init();
    //let host = HostInfo::parse("127.0.0.1:8848");
    //let client = NamingClient::new(host,"".to_owned());
    let namespace_id = "public".to_owned(); //default teant
    //let auth_info = Some(AuthInfo::new("nacos","nacos"));
    let auth_info = None;
    let client = NamingClient::new_with_addrs("127.0.0.1:8848,127.0.0.1:8848", namespace_id, auth_info);
    let servcie_key = ServiceInstanceKey::new("foo","DEFAULT_GROUP");
    //可以通过监听器获取指定服务的最新实现列表，并支持触发变更回调函数,可用于适配微服务地址选择器。
    let default_listener = InstanceDefaultListener::new(servcie_key,Some(Arc::new(
        |instances,add_list,remove_list| {
            println!("service instances change,count:{},add count:{},remove count:{}",instances.len(),add_list.len(),remove_list.len());
        })));
    client.subscribe(Box::new(default_listener.clone())).await.unwrap();
    let ip = local_ipaddress::get().unwrap();
    for i in 0..10{
        let port=10000+i;
        let instance = Instance::new(&ip,port,"foo","DEFAULT_GROUP","","",None);
        //注册
        client.register(instance);
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    //tokio::spawn(async{query_params2().await.unwrap();});
    let client2 = client.clone();
    tokio::spawn(
        async move {
            query_params(client2.clone()).await;
        }
    );

    //let mut buf = vec![0u8;1];
    //stdin().read(&mut buf).unwrap();
    tokio::signal::ctrl_c().await.expect("failed to listen for event");
    println!("n:{}",&client.namespace_id);
}

async fn query_params(client:Arc<NamingClient>) -> anyhow::Result<()>{
    let params = QueryInstanceListParams::new("","","foo",None,true);
    // 模拟每秒钟获取一次实例
    loop{
        //查询并按权重随机选择其中一个实例
        match client.select_instance(params.clone()).await{
            Ok(instances) =>{
                println!("select instance {}:{}",&instances.ip,&instances.port);
            },
            Err(e) => {
                println!("select_instance error {:?}",&e)
            },
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
    Ok(())
}