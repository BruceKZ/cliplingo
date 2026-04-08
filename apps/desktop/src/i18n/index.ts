import { computed } from "vue";
import { useUiStore, type AppLocale } from "@/stores/ui";

type MessageKey =
  | "nav.translate"
  | "nav.settings"
  | "nav.providers"
  | "nav.themeLight"
  | "nav.themeDark"
  | "nav.toggleSidebar"
  | "settings.title"
  | "settings.general"
  | "settings.trigger"
  | "settings.languageRouting"
  | "settings.providers"
  | "settings.modelParams"
  | "settings.privacy"
  | "settings.language"
  | "settings.themeMode"
  | "settings.showTrayIcon"
  | "settings.resetDefaults"
  | "settings.doubleCopyWindowMs"
  | "settings.fallbackShortcut"
  | "settings.replacePopupOnNewTrigger"
  | "settings.routingMode"
  | "settings.englishSource"
  | "settings.englishTargets"
  | "settings.chineseSource"
  | "settings.chineseTargets"
  | "settings.fallbackTargets"
  | "settings.primarySource"
  | "settings.primaryTargets"
  | "settings.secondarySource"
  | "settings.secondaryTargets"
  | "settings.targetLanguages"
  | "settings.userTranslationRules"
  | "settings.preserveParagraphs"
  | "settings.addProvider"
  | "settings.syncActive"
  | "settings.select"
  | "settings.makeActive"
  | "settings.clear"
  | "settings.edit"
  | "settings.duplicate"
  | "settings.remove"
  | "settings.addHeader"
  | "settings.providerId"
  | "settings.displayName"
  | "settings.providerKind"
  | "settings.authScheme"
  | "settings.baseUrl"
  | "settings.requestPath"
  | "settings.organization"
  | "settings.apiKeyDraft"
  | "settings.customHeaders"
  | "settings.activeProvider"
  | "settings.model"
  | "settings.temperature"
  | "settings.topP"
  | "settings.maxTokens"
  | "settings.requestTimeout"
  | "settings.enableLocalHistory"
  | "settings.historyLimit"
  | "settings.storeFullTextInHistory"
  | "settings.logRawNetworkErrors"
  | "settings.save"
  | "settings.saving"
  | "settings.testTranslation"
  | "settings.testing"
  | "settings.loading"
  | "settings.statusSaving"
  | "settings.statusTesting"
  | "settings.manageProviders"
  | "settings.noProvider"
  | "settings.statusAt"
  | "settings.refreshProviders"
  | "settings.autoRefreshOff"
  | "settings.noProvidersYet"
  | "settings.apiKeySaved"
  | "settings.replaceApiKey"
  | "settings.clearApiKey"
  | "settings.providersHelp"
  | "settings.providerListHelp"
  | "settings.providerBasics"
  | "settings.providerBasicsHint"
  | "settings.connectionSettings"
  | "settings.connectionSettingsHint"
  | "settings.requestSettings"
  | "settings.requestSettingsHint"
  | "settings.advancedSettings"
  | "settings.advancedSettingsHint"
  | "settings.headersHint"
  | "settings.providerActions"
  | "settings.providerActionsHint"
  | "settings.providerRequiredHint"
  | "settings.providerIdHint"
  | "settings.baseUrlHint"
  | "settings.requestPathHint"
  | "settings.authSchemeHint"
  | "settings.apiKeyHint"
  | "settings.modelHint"
  | "settings.requestTimeoutHint"
  | "settings.temperatureHint"
  | "settings.topPHint"
  | "settings.maxTokensHint"
  | "settings.noHeaders"
  | "settings.headerName"
  | "settings.headerValue"
  | "settings.statusSuccess"
  | "settings.statusInfo"
  | "settings.statusWarning"
  | "settings.statusError"
  | "settings.sectionNavigation"
  | "settings.sectionNavigationHint"
  | "settings.providerSummary"
  | "settings.providerSummaryHint"
  | "settings.providersDirectoryHelp"
  | "settings.providersDirectoryEmpty"
  | "settings.backToProviders"
  | "settings.providerEditorHelp"
  | "settings.editorMode"
  | "settings.editorModeHint"
  | "settings.formEditor"
  | "settings.jsonEditor"
  | "settings.jsonEditorHelp"
  | "settings.providerJsonLabel"
  | "settings.providerNotFound"
  | "settings.providerNotFoundHint"
  | "settings.invalidJsonObject"
  | "settings.invalidJsonField"
  | "settings.unnamedProvider"
  | "settings.providerCardFallback"
  | "settings.otherProviders"
  | "settings.providerReadiness"
  | "settings.providerConfigured"
  | "settings.providerNeedsSetup"
  | "settings.providerValidation"
  | "settings.providerVerified"
  | "settings.providerNeedsVerification"
  | "settings.triggerPermissions"
  | "settings.triggerPermissionsHint"
  | "settings.accessibilityStatusGranted"
  | "settings.accessibilityStatusMissing"
  | "settings.openAccessibilitySettings"
  | "settings.refreshPermissionStatus"
  | "settings.doubleCopyReady"
  | "settings.doubleCopyNeedsPermission"
  | "settings.platformPermissionNotRequired";

const MESSAGES: Record<AppLocale, Record<MessageKey, string>> = {
  en: {
    "nav.translate": "Translate",
    "nav.settings": "Settings",
    "nav.providers": "Providers",
    "nav.themeLight": "Theme",
    "nav.themeDark": "Theme",
    "nav.toggleSidebar": "Sidebar",
    "settings.title": "Settings",
    "settings.general": "General",
    "settings.trigger": "Trigger",
    "settings.languageRouting": "Language routing",
    "settings.providers": "Providers",
    "settings.modelParams": "Model params",
    "settings.privacy": "Privacy",
    "settings.language": "Language",
    "settings.themeMode": "Theme mode",
    "settings.showTrayIcon": "Show tray icon",
    "settings.resetDefaults": "Reset defaults",
    "settings.doubleCopyWindowMs": "Double copy window (ms)",
    "settings.fallbackShortcut": "Fallback shortcut",
    "settings.replacePopupOnNewTrigger": "Replace popup on new trigger",
    "settings.routingMode": "Routing mode",
    "settings.englishSource": "English source",
    "settings.englishTargets": "English targets",
    "settings.chineseSource": "Chinese source",
    "settings.chineseTargets": "Chinese targets",
    "settings.fallbackTargets": "Fallback targets",
    "settings.primarySource": "Primary source",
    "settings.primaryTargets": "Primary targets",
    "settings.secondarySource": "Secondary source",
    "settings.secondaryTargets": "Secondary targets",
    "settings.targetLanguages": "Target languages",
    "settings.userTranslationRules": "User translation rules",
    "settings.preserveParagraphs": "Preserve paragraphs",
    "settings.addProvider": "Add provider",
    "settings.syncActive": "Sync active",
    "settings.select": "Select",
    "settings.makeActive": "Use now",
    "settings.clear": "Clear",
    "settings.edit": "Edit",
    "settings.duplicate": "Duplicate",
    "settings.remove": "Remove",
    "settings.addHeader": "Add header",
    "settings.providerId": "Provider id",
    "settings.displayName": "Display name",
    "settings.providerKind": "Provider kind",
    "settings.authScheme": "Auth scheme",
    "settings.baseUrl": "Base URL",
    "settings.requestPath": "Request path",
    "settings.organization": "Organization",
    "settings.apiKeyDraft": "API key",
    "settings.customHeaders": "Custom headers",
    "settings.activeProvider": "In use",
    "settings.model": "Model",
    "settings.temperature": "Temperature",
    "settings.topP": "Top P",
    "settings.maxTokens": "Max tokens",
    "settings.requestTimeout": "Request timeout",
    "settings.enableLocalHistory": "Enable local history",
    "settings.historyLimit": "History limit",
    "settings.storeFullTextInHistory": "Store full text in history",
    "settings.logRawNetworkErrors": "Log raw network errors",
    "settings.save": "Save",
    "settings.saving": "Saving...",
    "settings.testTranslation": "Test translation",
    "settings.testing": "Testing...",
    "settings.loading": "Loading settings...",
    "settings.statusSaving": "Saving settings...",
    "settings.statusTesting": "Testing provider connection...",
    "settings.manageProviders": "Manage providers",
    "settings.noProvider": "No provider selected.",
    "settings.statusAt": "Last update",
    "settings.refreshProviders": "Refresh",
    "settings.autoRefreshOff": "Auto refresh: off",
    "settings.noProvidersYet": "No providers yet.",
    "settings.apiKeySaved": "API key saved securely",
    "settings.replaceApiKey": "Replace API key",
    "settings.clearApiKey": "Clear saved API key",
    "settings.providersHelp": "Set up provider connections, credentials, and request behavior.",
    "settings.providerListHelp": "Pick an existing provider or add a new one before editing details.",
    "settings.providerBasics": "Basics",
    "settings.providerBasicsHint": "Stable identity fields for this provider entry.",
    "settings.connectionSettings": "Connection",
    "settings.connectionSettingsHint": "Required fields for reaching the API endpoint securely.",
    "settings.requestSettings": "Model and request",
    "settings.requestSettingsHint": "Core runtime values used for translation requests.",
    "settings.advancedSettings": "Advanced parameters",
    "settings.advancedSettingsHint": "Tuning controls. Defaults usually work well.",
    "settings.headersHint": "Optional request headers for provider-specific routing or auth.",
    "settings.providerActions": "Actions",
    "settings.providerActionsHint": "Save changes before testing or switching the active provider.",
    "settings.providerRequiredHint": "Complete the connection fields first, then fine-tune only if needed.",
    "settings.providerIdHint": "Used as the stable internal identifier. Changing it later may require re-entering the API key.",
    "settings.baseUrlHint": "Use an HTTPS base URL. HTTP is allowed only for localhost during development.",
    "settings.requestPathHint": "Relative API path appended to the base URL, for example /chat/completions.",
    "settings.authSchemeHint": "Bearer is the default for OpenAI-compatible APIs.",
    "settings.apiKeyHint": "Stored securely in the system keychain or credential manager, never in plaintext config.",
    "settings.modelHint": "Example: gpt-4o-mini or your provider's deployed model id.",
    "settings.requestTimeoutHint": "Maximum wait time for one request, in seconds.",
    "settings.temperatureHint": "Lower values are more deterministic. Keep near the default unless you need variation.",
    "settings.topPHint": "Alternative sampling control. Leave at the default in most cases.",
    "settings.maxTokensHint": "Upper bound for the model response length.",
    "settings.noHeaders": "No custom headers configured.",
    "settings.headerName": "Header name",
    "settings.headerValue": "Header value",
    "settings.statusSuccess": "Success",
    "settings.statusInfo": "Info",
    "settings.statusWarning": "Warning",
    "settings.statusError": "Error",
    "settings.sectionNavigation": "Sections",
    "settings.sectionNavigationHint": "Switch between configuration areas for the selected provider.",
    "settings.providerSummary": "Provider summary",
    "settings.providerSummaryHint": "A quick snapshot of the current provider before editing details.",
    "settings.providersDirectoryHelp": "Choose a provider to edit its settings, or create a new one.",
    "settings.providersDirectoryEmpty": "Create your first provider to configure an OpenAI-compatible endpoint.",
    "settings.backToProviders": "Back to providers",
    "settings.providerEditorHelp": "Edit one provider at a time. Use the form for guided changes or switch to JSON for direct editing.",
    "settings.editorMode": "Editing mode",
    "settings.editorModeHint": "Choose between the guided form and direct JSON editing for this provider.",
    "settings.formEditor": "Form",
    "settings.jsonEditor": "JSON",
    "settings.jsonEditorHelp": "Edit the provider as JSON. API keys are only replaced when the `apiKey` field is present.",
    "settings.providerJsonLabel": "Provider Configuration JSON",
    "settings.providerNotFound": "Provider not found.",
    "settings.providerNotFoundHint": "The selected provider may have been removed or renamed.",
    "settings.invalidJsonObject": "JSON must be an object.",
    "settings.invalidJsonField": "One or more JSON fields have an invalid type.",
    "settings.unnamedProvider": "Untitled provider",
    "settings.providerCardFallback": "OpenAI-compatible provider",
    "settings.otherProviders": "Other providers",
    "settings.providerReadiness": "Status",
    "settings.providerConfigured": "Configured",
    "settings.providerNeedsSetup": "Needs setup",
    "settings.providerValidation": "Connectivity",
    "settings.providerVerified": "Reachable",
    "settings.providerNeedsVerification": "Untested",
    "settings.triggerPermissions": "Double-copy permission",
    "settings.triggerPermissionsHint":
      "ClipLingo uses macOS Accessibility permission for the global double-copy trigger. The fallback shortcut does not depend on it.",
    "settings.accessibilityStatusGranted": "Accessibility access granted",
    "settings.accessibilityStatusMissing": "Accessibility access required",
    "settings.openAccessibilitySettings": "Open Accessibility Settings",
    "settings.refreshPermissionStatus": "Refresh status",
    "settings.doubleCopyReady": "Double-copy trigger is available.",
    "settings.doubleCopyNeedsPermission":
      "Grant Accessibility permission to enable Cmd+C+C in other apps.",
    "settings.platformPermissionNotRequired":
      "This platform does not require an extra permission for double-copy.",
  },
  "zh-CN": {
    "nav.translate": "翻译",
    "nav.settings": "设置",
    "nav.providers": "服务商",
    "nav.themeLight": "主题",
    "nav.themeDark": "主题",
    "nav.toggleSidebar": "折叠",
    "settings.title": "设置",
    "settings.general": "通用",
    "settings.trigger": "触发器",
    "settings.languageRouting": "语言路由",
    "settings.providers": "服务商",
    "settings.modelParams": "模型参数",
    "settings.privacy": "隐私",
    "settings.language": "界面语言",
    "settings.themeMode": "主题模式",
    "settings.showTrayIcon": "显示托盘图标",
    "settings.resetDefaults": "恢复默认",
    "settings.doubleCopyWindowMs": "双击复制窗口（毫秒）",
    "settings.fallbackShortcut": "备用快捷键",
    "settings.replacePopupOnNewTrigger": "新触发时替换弹窗",
    "settings.routingMode": "路由模式",
    "settings.englishSource": "英文源语言",
    "settings.englishTargets": "英文目标语言",
    "settings.chineseSource": "中文源语言",
    "settings.chineseTargets": "中文目标语言",
    "settings.fallbackTargets": "兜底目标语言",
    "settings.primarySource": "主源语言",
    "settings.primaryTargets": "主目标语言",
    "settings.secondarySource": "次源语言",
    "settings.secondaryTargets": "次目标语言",
    "settings.targetLanguages": "目标语言",
    "settings.userTranslationRules": "用户翻译规则",
    "settings.preserveParagraphs": "保留段落",
    "settings.addProvider": "新增服务商",
    "settings.syncActive": "同步活动项",
    "settings.select": "选择",
    "settings.makeActive": "立即使用",
    "settings.clear": "清除",
    "settings.edit": "编辑",
    "settings.duplicate": "复制",
    "settings.remove": "移除",
    "settings.addHeader": "新增 Header",
    "settings.providerId": "服务商 ID",
    "settings.displayName": "显示名称",
    "settings.providerKind": "服务商类型",
    "settings.authScheme": "认证方式",
    "settings.baseUrl": "基础 URL",
    "settings.requestPath": "请求路径",
    "settings.organization": "组织",
    "settings.apiKeyDraft": "API Key",
    "settings.customHeaders": "自定义 Headers",
    "settings.activeProvider": "使用中",
    "settings.model": "模型",
    "settings.temperature": "温度",
    "settings.topP": "Top P",
    "settings.maxTokens": "最大 Tokens",
    "settings.requestTimeout": "请求超时",
    "settings.enableLocalHistory": "启用本地历史",
    "settings.historyLimit": "历史上限",
    "settings.storeFullTextInHistory": "历史中保存完整文本",
    "settings.logRawNetworkErrors": "记录原始网络错误",
    "settings.save": "保存",
    "settings.saving": "保存中...",
    "settings.testTranslation": "测试翻译",
    "settings.testing": "测试中...",
    "settings.loading": "正在加载设置...",
    "settings.statusSaving": "正在保存设置...",
    "settings.statusTesting": "正在测试服务连接...",
    "settings.manageProviders": "管理服务商",
    "settings.noProvider": "未选择服务商。",
    "settings.statusAt": "最近更新",
    "settings.refreshProviders": "手动刷新",
    "settings.autoRefreshOff": "自动刷新：关闭",
    "settings.noProvidersYet": "还没有服务商。",
    "settings.apiKeySaved": "API Key 已保存",
    "settings.replaceApiKey": "替换 API Key",
    "settings.clearApiKey": "清除已保存 API Key",
    "settings.providersHelp": "配置服务商连接、凭据和请求行为。",
    "settings.providerListHelp": "先选择已有服务商，或新增一个服务商，再编辑详细配置。",
    "settings.providerBasics": "基础信息",
    "settings.providerBasicsHint": "当前服务商条目的稳定标识信息。",
    "settings.connectionSettings": "连接配置",
    "settings.connectionSettingsHint": "用于安全访问 API 的必填字段。",
    "settings.requestSettings": "模型与请求",
    "settings.requestSettingsHint": "翻译请求会直接使用的核心参数。",
    "settings.advancedSettings": "高级参数",
    "settings.advancedSettingsHint": "调优项。大多数情况下默认值即可。",
    "settings.headersHint": "仅在服务商要求额外路由或认证 Header 时使用。",
    "settings.providerActions": "操作",
    "settings.providerActionsHint": "测试或切换活动服务商前，先保存当前修改。",
    "settings.providerRequiredHint": "先完成连接必填项，再按需微调高级参数。",
    "settings.providerIdHint": "作为稳定内部标识使用。后续如果修改，可能需要重新输入 API Key。",
    "settings.baseUrlHint": "优先使用 HTTPS。开发环境下仅允许 localhost 使用 HTTP。",
    "settings.requestPathHint": "拼接在基础 URL 之后的相对路径，例如 /chat/completions。",
    "settings.authSchemeHint": "OpenAI-compatible API 默认使用 Bearer。",
    "settings.apiKeyHint": "会安全保存到系统钥匙串或凭据管理器，不会明文写入配置文件。",
    "settings.modelHint": "例如 gpt-4o-mini，或你的服务商部署模型 ID。",
    "settings.requestTimeoutHint": "单次请求的最长等待时间，单位为秒。",
    "settings.temperatureHint": "值越低越稳定。除非确有需要，否则保持默认值。",
    "settings.topPHint": "另一种采样控制参数。多数情况下保持默认即可。",
    "settings.maxTokensHint": "限制模型输出的最大长度。",
    "settings.noHeaders": "当前没有自定义 Header。",
    "settings.headerName": "Header 名称",
    "settings.headerValue": "Header 值",
    "settings.statusSuccess": "成功",
    "settings.statusInfo": "信息",
    "settings.statusWarning": "警告",
    "settings.statusError": "错误",
    "settings.sectionNavigation": "配置分组",
    "settings.sectionNavigationHint": "切换当前服务商的不同配置区域。",
    "settings.providerSummary": "服务商概览",
    "settings.providerSummaryHint": "在编辑详细配置前先快速确认当前服务商信息。",
    "settings.providersDirectoryHelp": "选择一个服务商进入配置，或新建一个服务商。",
    "settings.providersDirectoryEmpty": "先创建第一个服务商，再配置 OpenAI-compatible 接口。",
    "settings.backToProviders": "返回服务商列表",
    "settings.providerEditorHelp": "一次只编辑一个服务商。可使用表单模式，也可切换到 JSON 直接编辑。",
    "settings.editorMode": "编辑方式",
    "settings.editorModeHint": "可在表单编辑和直接编辑 JSON 之间切换。",
    "settings.formEditor": "表单",
    "settings.jsonEditor": "JSON",
    "settings.jsonEditorHelp": "以 JSON 方式直接编辑服务商。只有在提供 `apiKey` 字段时才会替换当前密钥。",
    "settings.providerJsonLabel": "服务商配置 JSON",
    "settings.providerNotFound": "未找到该服务商。",
    "settings.providerNotFoundHint": "该服务商可能已被删除，或其 ID 已发生变化。",
    "settings.invalidJsonObject": "JSON 内容必须是一个对象。",
    "settings.invalidJsonField": "JSON 中存在字段类型错误。",
    "settings.unnamedProvider": "未命名服务商",
    "settings.providerCardFallback": "OpenAI-compatible 服务商",
    "settings.otherProviders": "其他服务商",
    "settings.providerReadiness": "状态",
    "settings.providerConfigured": "已配置",
    "settings.providerNeedsSetup": "待完善",
    "settings.providerValidation": "连通性",
    "settings.providerVerified": "已连通",
    "settings.providerNeedsVerification": "待检测",
    "settings.triggerPermissions": "双击复制权限",
    "settings.triggerPermissionsHint":
      "ClipLingo 在 macOS 上使用辅助功能权限启用全局双击复制触发器，备用快捷键不依赖这个权限。",
    "settings.accessibilityStatusGranted": "已授予辅助功能权限",
    "settings.accessibilityStatusMissing": "需要辅助功能权限",
    "settings.openAccessibilitySettings": "打开辅助功能设置",
    "settings.refreshPermissionStatus": "刷新状态",
    "settings.doubleCopyReady": "双击复制触发器可用。",
    "settings.doubleCopyNeedsPermission":
      "请授予辅助功能权限，以便在其他应用中使用 Cmd+C+C。",
    "settings.platformPermissionNotRequired": "当前平台不需要额外权限即可使用双击复制。",
  },
};

export function useI18n() {
  const uiStore = useUiStore();
  const dictionary = computed(() => MESSAGES[uiStore.locale] ?? MESSAGES.en);

  function t(key: MessageKey): string {
    return dictionary.value[key] ?? MESSAGES.en[key] ?? key;
  }

  return { t };
}
