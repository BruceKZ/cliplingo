import { computed } from "vue";
import { useUiStore, type AppLocale } from "@/stores/ui";

type MessageKey =
  | "nav.translate"
  | "nav.settings"
  | "nav.providers"
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
  | "settings.clearApiKey";

const MESSAGES: Record<AppLocale, Record<MessageKey, string>> = {
  en: {
    "nav.translate": "Translate",
    "nav.settings": "Settings",
    "nav.providers": "Providers",
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
    "settings.makeActive": "Make active",
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
    "settings.activeProvider": "Active provider",
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
  },
  "zh-CN": {
    "nav.translate": "翻译",
    "nav.settings": "设置",
    "nav.providers": "服务商",
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
    "settings.makeActive": "设为活动项",
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
    "settings.activeProvider": "活动服务商",
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
