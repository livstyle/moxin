use crate::data::*;
use std::sync::mpsc::Sender;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub enum ContextOverflowPolicy {
    StopAtLimit,
    TruncateMiddle,
    TruncatePastMessages,
}

#[derive(Clone, Debug)]
pub enum GPULayers {
    Specific(u32),
    Max,
}

#[derive(Clone, Debug)]
pub struct LoadModelOptions {
    pub prompt_template: Option<String>,
    pub gpu_layers: GPULayers,
    pub use_mlock: bool,
    pub n_batch: u32,
    pub n_ctx: u32,
    pub rope_freq_scale: f32,
    pub rope_freq_base: f32,

    // TBD Not really sure if this is something backend manages or if it is matter of
    // the client (if it is done by tweaking the JSON payload for the chat completition)
    pub context_overflow_policy: ContextOverflowPolicy
}

#[derive(Clone, Debug)]
pub struct LocalServerConfig {
    pub port: u16,
    pub cors: bool,
    pub request_queuing: bool,
    pub verbose_server_logs: bool,
    pub apply_prompt_formatting: bool,
}

#[derive(Clone, Debug)]
pub enum Command {
    GetFeaturedModels(Sender<Vec<Model>>),

    // The argument is a string with the keywords to search for.
    SearchModels(String, Sender<Vec<Model>>),

    DownloadFile(FileID, Sender<FileDownloadResponse>),
    GetDownloadedFiles(Sender<Vec<DownloadedFile>>),

    LoadModel(FileID, LoadModelOptions, Sender<LoadModelResponse>),
    EjectModel(FileID),
    GetLoadedModel(Sender<Option<ModelResourcesInfo>>),

    // The argument is the chat message in JSON format, following https://platform.openai.com/docs/api-reference/chat/create
    Chat(String, Sender<Result<ChatResponse, ChatError>>),
    // Command to stop the current chat completion
    StopChatCompletion,

    // Command to start a local server to interact with chat models
    StartLocalServer(LocalServerConfig, Sender<LocalServerResponse>),
    // Command to stop the local server
    StopLocalServer,
}

#[derive(Clone, Debug)]
pub struct LoadedModelInfo {
    pub file_id: FileID,
    pub model_id: ModelID,

    // JSON formatted string with the model information. See "Model Inspector" in LMStudio.
    pub information: String,
}

#[derive(Clone, Debug)]
pub struct ModelResourcesInfo {
    ram_usage: f32,
    cpu_usage: f32,
}

#[derive(Clone, Debug)]
pub enum StopReason {
    Completed,
    Stopped
}

#[derive(Clone, Debug)]
pub struct ChatCompletionData {
    // The response from the model in JSON format, following https://platform.openai.com/docs/api-reference/chat/create
    response: String,

    // The remaining fields are stats about the chat completion process
    time_to_first_token: f32,
    time_to_generate: f32,
    speed: f32,
    gpu_layers: u32,
    cpu_threads: u32,
    mlock: bool,
    token_count: u32,
    token_limit: u32,
    stop_reason: StopReason,
}

#[derive(Clone, Debug)]
pub enum CompatibilityGuess {
    PossiblySupported,
    NotSupported,
}

#[derive(Clone, Debug)]
pub struct DownloadedFile {
    pub file: File,
    pub model: Model,
    pub downloaded_at: NaiveDate,
    pub compatibility_guess: CompatibilityGuess,
    pub information: String,
}

#[derive(Clone, Debug)]
pub enum FileDownloadResponse {
    Progress(FileID, f32),
    Completed(File),
}

#[derive(Clone, Debug)]
pub enum LoadModelResponse {
    Progress(FileID, f32),
    Completed(LoadedModelInfo),
    ModelResoucesUsage(ModelResourcesInfo),
}

pub enum ChatError {
    BackendNotRun,
    EndOfSequence,
    ContextFull,
    PromptTooLong,
    TooLarge,
    InvalidEncoding,
    Other,
}

#[derive(Clone, Debug)]
pub enum ChatResponse {
    // https://platform.openai.com/docs/api-reference/chat/object
    ChatCompletion(String),
    // https://platform.openai.com/docs/api-reference/chat/streaming
    ChatCompletionChunk(String),
}

#[derive(Clone, Debug)]
pub enum LocalServerResponse {
    Started,
    Log(String),
}