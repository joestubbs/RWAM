/// Tapis Client bindings. 
/// WIP; currently, only a few endpoints within the Tapis Files API are supported.
/// 


/// Primary object for interacting with Tapis API server
pub struct TapisClient {
    /// Base URL to Tapis API server
    base_url: String,

    /// Valid Tapis JWT; Note: this client currently does not manage tokens automatically
    jwt: String, 

}

impl TapisClient {

    /// Create a new TapisClient 
    pub fn new(base_url: String, jwt: String) -> TapisClient {
        TapisClient {
            base_url,
            jwt,
        }
    }

    // List Files
    // pub fn list_files(system_id: String, path: String) -> Result<FilesListResponse> {
        
    // }
}