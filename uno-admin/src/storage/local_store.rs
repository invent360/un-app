use crate::models::{LicenseConfig, LicenseConfigCsv};

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

const LICENSE_CONFIGS_KEY: &str = "unity_license_configs";
const AGENT_MAPPINGS_KEY: &str = "unity_agent_mappings";
const JOBS_LAST_SEEN_KEY: &str = "jobs_last_seen_at";

/// Save license configurations to localStorage
#[cfg(target_arch = "wasm32")]
pub fn save_license_configs(configs: &[LicenseConfig]) -> Result<(), String> {
    LocalStorage::set(LICENSE_CONFIGS_KEY, configs)
        .map_err(|e| format!("Failed to save license configs: {}", e))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_license_configs(_configs: &[LicenseConfig]) -> Result<(), String> {
    Ok(())
}

/// Load license configurations from localStorage
#[cfg(target_arch = "wasm32")]
pub fn load_license_configs() -> Vec<LicenseConfig> {
    LocalStorage::get(LICENSE_CONFIGS_KEY).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_license_configs() -> Vec<LicenseConfig> {
    Vec::new()
}

/// Clear license configurations from localStorage
#[cfg(target_arch = "wasm32")]
pub fn clear_license_configs() {
    LocalStorage::delete(LICENSE_CONFIGS_KEY);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_license_configs() {}

/// Get the last time the user viewed the jobs page (as ISO timestamp string)
#[cfg(target_arch = "wasm32")]
pub fn get_jobs_last_seen() -> Option<String> {
    LocalStorage::get(JOBS_LAST_SEEN_KEY).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_jobs_last_seen() -> Option<String> {
    None
}

/// Set the last time the user viewed the jobs page (as ISO timestamp string)
#[cfg(target_arch = "wasm32")]
pub fn set_jobs_last_seen(timestamp: &str) {
    let _ = LocalStorage::set(JOBS_LAST_SEEN_KEY, timestamp);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_jobs_last_seen(_timestamp: &str) {}

/// Parse CSV content into license configurations
pub fn parse_license_csv(csv_content: &str) -> Result<Vec<LicenseConfig>, String> {
    let mut configs = Vec::new();
    let lines: Vec<&str> = csv_content.lines().collect();

    if lines.is_empty() {
        return Err("Empty CSV content".to_string());
    }

    // Parse header to find column indices
    let header = lines[0].to_lowercase();
    let headers: Vec<&str> = header.split(',').map(|s| s.trim()).collect();

    let license_id_idx = headers.iter().position(|h| h.contains("license_id") || h.contains("licenseid"));
    let lease_code_idx = headers.iter().position(|h| h.contains("lease_code") || h.contains("leasecode"));
    let uno_share_idx = headers.iter().position(|h| h.contains("uno_share") || h.contains("unoshare"));
    let agent_share_idx = headers.iter().position(|h| h.contains("agent_share") || h.contains("agentshare"));
    let ulo_share_idx = headers.iter().position(|h| h.contains("ulo_share") || h.contains("uloshare"));
    let ulo_idx = headers.iter().position(|h| *h == "ulo" || h.contains("ulo_name"));
    let agent_idx = headers.iter().position(|h| *h == "agent" || h.contains("agent_name"));

    // Verify required columns exist
    let license_id_idx = license_id_idx.ok_or("Missing license_id column")?;
    let lease_code_idx = lease_code_idx.ok_or("Missing lease_code column")?;
    let uno_share_idx = uno_share_idx.ok_or("Missing uno_share column")?;
    let agent_share_idx = agent_share_idx.ok_or("Missing agent_share column")?;
    let ulo_share_idx = ulo_share_idx.ok_or("Missing ulo_share column")?;
    let ulo_idx = ulo_idx.ok_or("Missing ulo column")?;
    let agent_idx = agent_idx.ok_or("Missing agent column")?;

    // Parse data rows
    for (line_num, line) in lines.iter().skip(1).enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() <= agent_idx.max(ulo_idx).max(ulo_share_idx).max(agent_share_idx).max(uno_share_idx).max(lease_code_idx).max(license_id_idx) {
            return Err(format!("Line {} has insufficient columns", line_num + 2));
        }

        let csv_row = LicenseConfigCsv {
            license_id: fields[license_id_idx].to_string(),
            lease_code: fields[lease_code_idx].to_string(),
            uno_share: fields[uno_share_idx].to_string(),
            agent_share: fields[agent_share_idx].to_string(),
            ulo_share: fields[ulo_share_idx].to_string(),
            ulo: fields[ulo_idx].to_string(),
            agent: fields[agent_idx].to_string(),
        };

        match csv_row.to_config() {
            Ok(config) => configs.push(config),
            Err(e) => return Err(format!("Error parsing line {}: {}", line_num + 2, e)),
        }
    }

    Ok(configs)
}

/// Build a lookup map from license_id to LicenseConfig
pub fn build_license_map(configs: Vec<LicenseConfig>) -> std::collections::HashMap<String, LicenseConfig> {
    configs.into_iter()
        .map(|c| (c.license_id.clone(), c))
        .collect()
}

/// Get unique agents from license configs
pub fn get_unique_agents(configs: &[LicenseConfig]) -> Vec<String> {
    let mut agents: Vec<String> = configs.iter()
        .filter(|c| !c.agent_name.is_empty())
        .map(|c| c.agent_name.clone())
        .collect();
    agents.sort();
    agents.dedup();
    agents
}

/// Get unique ULOs from license configs
pub fn get_unique_ulos(configs: &[LicenseConfig]) -> Vec<String> {
    let mut ulos: Vec<String> = configs.iter()
        .filter(|c| !c.ulo_name.is_empty())
        .map(|c| c.ulo_name.clone())
        .collect();
    ulos.sort();
    ulos.dedup();
    ulos
}

/// Get licenses for a specific agent
pub fn get_licenses_for_agent(configs: &[LicenseConfig], agent_name: &str) -> Vec<LicenseConfig> {
    configs.iter()
        .filter(|c| c.agent_name == agent_name)
        .cloned()
        .collect()
}

/// Get licenses for a specific ULO
pub fn get_licenses_for_ulo(configs: &[LicenseConfig], ulo_name: &str) -> Vec<LicenseConfig> {
    configs.iter()
        .filter(|c| c.ulo_name == ulo_name)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_license_csv() {
        let csv = r#"license_id,lease_code,uno_share,agent_share,ulo_share,ulo,agent
0x015311594a4601d0a8b40641eead359fff8423141d5a7959027140c8cbb245b0,c9668f39-bf21-4982-b64d-180e5cd4ef5c,47%,3%,50%,NGA Lagos1,Staffman
0x011fa3571984361b94d35fba495f03894e74a86a2361e5c8d83481634948e4ce,d93ec394-5b2a-4e5e-826c-e602af8fad2d,47%,3%,50%,NGA Lagos2,Staffman"#;

        let configs = parse_license_csv(csv).unwrap();
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].ulo_name, "NGA Lagos1");
        assert_eq!(configs[0].agent_name, "Staffman");
        assert_eq!(configs[0].splits.uno_share, 47.0);
        assert_eq!(configs[0].splits.agent_share, 3.0);
        assert_eq!(configs[0].splits.ulo_share, 50.0);
    }

    #[test]
    fn test_get_unique_agents() {
        let csv = r#"license_id,lease_code,uno_share,agent_share,ulo_share,ulo,agent
0x01,code1,47%,3%,50%,ULO1,AgentA
0x02,code2,47%,3%,50%,ULO2,AgentB
0x03,code3,47%,3%,50%,ULO3,AgentA"#;

        let configs = parse_license_csv(csv).unwrap();
        let agents = get_unique_agents(&configs);
        assert_eq!(agents, vec!["AgentA", "AgentB"]);
    }
}
