use crate::links;

#[derive(Debug)]
#[allow(dead_code)]

pub enum Gaming {
    AMDAutoDetect,
    BattleNetLauncher,
    BluestacksEmulator,
    CPUZ,
    CurseForge,
    EpicGamesLauncher,
    GPUZ,
    HWiNFO,
    HWMonitor,
    MSIAfterburner,
    NvidiaApp,
    OBSStudio,
    Parsec,
    PingPlotter,
    ProcessLasso,
    RazerCortex,
    Reshade,
    RockstarLauncher,
    RobloxLauncher,
    StreamlabsOBS,
    SteamLauncher,
    WTFast,
}

#[allow(dead_code)]
impl Gaming {
    pub fn get_link(&self) -> Option<String> {
        match self {
            Gaming::AMDAutoDetect => Some(links::get_amd_autodetect_link()),
            Gaming::BattleNetLauncher => Some(links::get_battlenet_link()),
            Gaming::BluestacksEmulator => Some(links::get_bluestacks_link()),
            Gaming::CPUZ => Some(links::get_cpuz_link()),
            Gaming::CurseForge => Some(links::get_curseforge_link()),
            Gaming::EpicGamesLauncher => Some(links::get_epic_games_link()),
            Gaming::GPUZ => Some(links::get_gpuz_link()),
            Gaming::HWiNFO => Some(links::get_hwinfo_link()),
            Gaming::HWMonitor => Some(links::get_hwmonitor_link()),
            Gaming::MSIAfterburner => Some(links::get_msi_afterburner_link()),
            Gaming::NvidiaApp => Some(links::get_nvda_app_link()),
            Gaming::OBSStudio => Some(links::get_obs_studio_link()),
            Gaming::Parsec => Some(links::get_parsec_link()),
            Gaming::PingPlotter => Some(links::get_pingplotter_link()),
            Gaming::ProcessLasso => Some(links::get_process_lasso_link()),
            Gaming::RazerCortex => Some(links::get_razer_cortex_link()),
            Gaming::Reshade => Some(links::get_reshade_link()),
            Gaming::RockstarLauncher => Some(links::get_rockstar_launcher_link()),
            Gaming::RobloxLauncher => Some(links::get_roblox_launcher_link()),
            Gaming::StreamlabsOBS => Some(links::get_streamlabs_obs_link()),
            Gaming::SteamLauncher => Some(links::get_steam_link()),
            Gaming::WTFast => Some(links::get_wtfast_link()),
        }
    }
}
