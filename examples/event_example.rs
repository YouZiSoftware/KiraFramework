/// OneBot Document:
/// ## Lifecycle
///
/// | Field name | Type | Possible value | Description |
/// | ----- | ------ | -------- | --- |
/// | `time` | number (int64) | - | Event timestamp |
/// | `self_id` | number (int64) | - | Bot QQ number |
/// | `post_type` | string | `meta_event` | Post type |
/// | `meta_event_type` | string | `lifecycle` | Meta event type |
/// | `sub_type` | string | `enable`、`disable`、`connect` | sub type of event. `enable`: Bot enabled, `disable`: Bot disabled, `connect`: WebSocket connected |
///
/// ## onebot_event_type format:
///
/// #[onebot_event_type(post_name = "event_type")]
///
/// ### or
///
/// #[onebot_event_type(post_name = "event_type.sub_type")]
///
/// ### for example:
/// ```
/// #[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
/// #[onebot_event_type(meta_event = "lifecycle.enable")]
/// pub struct LifecycleEnable;
/// ```

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "lifecycle.enable")]
pub struct LifecycleEnable;

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "lifecycle.disable")]
pub struct LifecycleDisable;

#[derive(Serialize, Deserialize, OneBotEvent, Debug, Clone)]
#[onebot_event_type(meta_event = "lifecycle.connect")]
pub struct LifecycleConnect;