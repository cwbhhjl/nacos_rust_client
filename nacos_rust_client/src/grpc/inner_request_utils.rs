use tonic::transport::Channel;

use crate::{client::config_client::ConfigKey, conn_manage::conn_msg::{ConnMsgResult, ConfigResponse}};

use super::{api_model::{ConfigQueryRequest, ConfigQueryResponse, ConfigPublishRequest, BaseResponse, ConfigRemoveRequest}, utils::PayloadUtils, nacos_proto::request_client::RequestClient};



pub(crate) struct InnerRequestUtils;

impl InnerRequestUtils {

    pub async fn config_query(channel:Channel,config_key:ConfigKey) -> anyhow::Result<ConnMsgResult> {
        let request = ConfigQueryRequest {
            data_id:config_key.data_id,
            group:config_key.group,
            tenant:config_key.tenant,
            ..Default::default()
        };
        let val = serde_json::to_string(&request).unwrap();
        let payload = PayloadUtils::build_payload("ConfigQueryRequest", val);
        let  mut request_client = RequestClient::new(channel);
        let response =request_client.request(tonic::Request::new(payload)).await?;
        let body_vec = response.into_inner().body.unwrap_or_default().value;
        let response:ConfigQueryResponse= serde_json::from_slice(&body_vec)?;
        Ok(ConnMsgResult::ConfigResult(ConfigResponse::ConfigValue(response.content)))
    }

    pub async fn config_publish(channel:Channel,config_key:ConfigKey,content:String) -> anyhow::Result<ConnMsgResult> {
        let request = ConfigPublishRequest {
            data_id:config_key.data_id,
            group:config_key.group,
            tenant:config_key.tenant,
            content,
            ..Default::default()
        };
        let val = serde_json::to_string(&request).unwrap();
        let payload = PayloadUtils::build_payload("ConfigPublishRequest", val);
        let  mut request_client = RequestClient::new(channel);
        let response =request_client.request(tonic::Request::new(payload)).await?;
        let body_vec = response.into_inner().body.unwrap_or_default().value;
        let _:BaseResponse= serde_json::from_slice(&body_vec)?;
        Ok(ConnMsgResult::None)
    }

    pub async fn config_remove(channel:Channel,config_key:ConfigKey) -> anyhow::Result<ConnMsgResult> {
        let request = ConfigRemoveRequest {
            data_id:config_key.data_id,
            group:config_key.group,
            tenant:config_key.tenant,
            ..Default::default()
        };
        let val = serde_json::to_string(&request).unwrap();
        let payload = PayloadUtils::build_payload("ConfigRemoveRequest", val);
        let  mut request_client = RequestClient::new(channel);
        let response =request_client.request(tonic::Request::new(payload)).await?;
        let body_vec = response.into_inner().body.unwrap_or_default().value;
        let _:BaseResponse= serde_json::from_slice(&body_vec)?;
        Ok(ConnMsgResult::None)
    }

}
