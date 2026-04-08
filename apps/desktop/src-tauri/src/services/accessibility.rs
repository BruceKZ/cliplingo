use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerPermissionStatus {
    pub platform: String,
    pub accessibility_required: bool,
    pub accessibility_granted: bool,
    pub double_copy_available: bool,
    pub settings_url: Option<String>,
}

pub fn trigger_permission_status() -> TriggerPermissionStatus {
    #[cfg(target_os = "macos")]
    {
        let granted = macos_accessibility_granted();
        return TriggerPermissionStatus {
            platform: "macos".to_string(),
            accessibility_required: true,
            accessibility_granted: granted,
            double_copy_available: granted,
            settings_url: Some(
                "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"
                    .to_string(),
            ),
        };
    }

    #[cfg(not(target_os = "macos"))]
    {
        TriggerPermissionStatus {
            platform: std::env::consts::OS.to_string(),
            accessibility_required: false,
            accessibility_granted: true,
            double_copy_available: true,
            settings_url: None,
        }
    }
}

pub fn open_trigger_permission_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .status()
            .map_err(|error| error.to_string())
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "failed to open Accessibility settings (exit status: {status})"
                    ))
                }
            })
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("trigger permission settings are only available on macOS".to_string())
    }
}

#[cfg(target_os = "macos")]
pub fn macos_accessibility_granted() -> bool {
    unsafe extern "C" {
        fn AXIsProcessTrusted() -> bool;
    }

    unsafe { AXIsProcessTrusted() }
}
