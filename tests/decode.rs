use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// #[derive(Debug, Serialize, Some(Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct VolcanoReq {
    pub name: String,
    pub instance_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolcanoJobSpec {
    pub min_available: i32,
    pub tasks: Vec<VolcanoJobTask>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VolcanoJobTask {
    pub name: String,
    pub replicas: Option<i32>,
    pub template: VolcanoJobTemplate,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VolcanoJobTemplate {
    pub meta_data: VolcanoJobMetaData,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VolcanoJobMetaData {
    pub annotations: VolcanoJobAnnotations,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VolcanoJobAnnotations {
    #[serde(rename = "instance-id-topic")]
    pub instance_id_topic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolcanoJobRoot {
    pub spec: VolcanoJobSpec,
}

fn test_decode() {
    let data = r#"
{"meta_data":{"name":"gm-model-1000000125","namespace":"idp","uid":"11483f0c-4ac7-42f2-8dd8-8e99ae60079c","resource_version":"9105775","generation":2,"creation_timestamp":1725971515,"labels":{"app":"gm-model-1000000125"},"annotations":{"projectId":"unknown","teamId":"1821174353329995776"}},"spec":{"scheduler_name":"volcano","min_available":1,"tasks":[{"name":"spec0","template":{"meta_data":{"creation_timestamp":-62135596800,"labels":{"app":"gm-model-1000000125","model-market-deploy":"all","sidecar.istio.io/inject":"false"},"annotations":{"instance-id-topic":"/instance_ids/1000000125/4","model-market-deploy-vcjob-name":"gm-model-1000000125","service-ids":"[\"1000000125\"]","teamId":"1821174353329995776"}},"spec":{"volumes":[{"name":"model-pvc","persistent_volume_claim":{"claim_name":"idp-model-pvc-idp"}},{"name":"data-b-1821174353329995776","persistent_volume_claim":{"claim_name":"idp-storageclass-pvc-b-1821174353329995776"}},{"name":"config","config_map":{"name":"config-toml-config"}},{"name":"log","empty_dir":{"size_limit":"1Gi"}},{"name":"kaniko-secret","secret":{"secret_name":"hub-secret","items":[{"key":".dockerconfigjson","path":"config.json"}],"default_mode":420}},{"name":"log-volume","host_path":{"path":"/var/log/unsent_messages","type":"DirectoryOrCreate"}}],"containers":[{"name":"gm-model-1000000125","image":"aiharbor.icloud.cn:30002/idp-saas/nginx:1.27.1-alpine3.20-perl","ports":[{"name":"http-9093","container_port":9093,"protocol":"TCP"}],"env":[{"name":"GM_SERVER_URL","value":"http://gm.sw.com"},{"name":"SERVICE_ID","value":"1000000125"},{"name":"PRODUCT_TYPE","value":"node"}],"resources":{"limits":{"cpu":"21333m","memory":"84Gi","nvidia.com/P40":"2"},"requests":{"cpu":"16","memory":"63Gi","nvidia.com/P40":"2"}},"volume_mounts":[{"name":"model-pvc","mount_path":"/store_model","mount_propagation":"None"},{"name":"data-b-1821174353329995776","mount_path":"/store","mount_propagation":"None"},{"name":"config","mount_path":"/opt/config","mount_propagation":"None"},{"name":"log","mount_path":"/var/log/model","mount_propagation":"None"},{"name":"config","mount_path":"/root/.pip/pip.conf","sub_path":"pip.conf","mount_propagation":"None"},{"name":"kaniko-secret","mount_path":"/kaniko/.docker","mount_propagation":"None"}],"lifecycle":{"pre_stop":{"exec":{"command":["sh","-c","kill 1"]}}},"image_pull_policy":"IfNotPresent"},{"name":"k8s-hook-pod","image":"aiharbor.icloud.cn:30002/idp-saas/k8s-hook-pod:20240909_SWT_1912","env":[{"name":"RabbitMQHost","value":"172.25.135.13"},{"name":"RabbitMQPort","value":"30000"},{"name":"RabbitMQUserName","value":"edge1"},{"name":"RabbitMQPassWord","value":"kubeEDGE135"},{"name":"Job_Info","value":"{\"cpu_number\":\"16.00\",\"cpu_scheduled\":\"16.00\",\"create_timestamp\":\"2024-09-10 12:31:55\",\"gpu_number\":\"2.00\",\"gpu_scheduled\":\"2.00\",\"job_id\":\"gm-model-1000000125\",\"mem_number\":\"64.51\",\"mem_scheduled\":\"64.51\",\"module\":\"cluster\",\"name\":\"gm-model-1000000125\",\"status\":\"RUNNING\",\"team_id\":\"1821174353329995776\",\"top_app\":\"studio\"}"},{"name":"InstanceIdTopic","value":"/instance_ids/1000000125/4"},{"name":"PodName"},{"name":"PodIP"},{"name":"PodHostIP"}],"resources":{},"volume_mounts":[{"name":"log-volume","mount_path":"/var/log/unsent_messages"}],"lifecycle":{"post_start":{"exec":{"command":["/bin/bash","-c","/app/main start \u003e /usr/share/message"]}},"pre_stop":{"exec":{"command":["/bin/bash","-c","/app/main stop \u003e /usr/share/message"]}}}}],"restart_policy":"OnFailure","node_selector":{"idp.baihai.co/nvidia-p40x2-cpu32-mem126-specs":"true"},"service_account_name":"idp-studio-account","automount_service_account_token":false,"image_pull_secrets":[{"name":"sw-secret"},{"name":"swt-secret"},{"name":"hub-secret"}],"hostname":"gm-model-1000000125","scheduler_name":"volcano","tolerations":[{"key":"idp.baihai.co/taint-user-1821174353329995776","operator":"Equal","value":"true","effect":"NoExecute"}],"enable_service_links":false,"host_users":true}}}],"policies":[{"action":"RestartJob","event":"TaskFailed"}],"queue":"volcano-queue-idp-b-1821174353329995776","priority_class_name":"3-priority"},"status":{"state":{"phase":"Running","last_transition_time":1725971529},"min_available":1,"conditions":[{"status":"Pending","last_transition_time":1725971515},{"status":"Running","last_transition_time":1725971529}]}}
    "#;

    let parsed: VolcanoJobRoot = serde_json::from_str(data).unwrap();

    println!("test{:#?}", parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_decode();
    }
}
