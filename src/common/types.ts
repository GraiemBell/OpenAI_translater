import { Theme } from 'baseui-sd/theme'
import { TranslateMode } from './translate'
import { TTSProvider } from './tts/types'
import { Provider } from './engines'
import { LangCode } from './lang'

/* eslint-disable @typescript-eslint/no-explicit-any */
interface ISync {
    get(keys: string[]): Promise<Record<string, any>>
    set(items: Record<string, any>): Promise<void>
}

interface IStorage {
    sync: ISync
}

interface IRuntimeOnMessage {
    addListener(callback: (message: any, sender: any, sendResponse: any) => void): void
    removeListener(callback: (message: any, sender: any, sendResponse: any) => void): void
}

interface IRuntime {
    onMessage: IRuntimeOnMessage
    sendMessage(message: any): void
    getURL(path: string): string
}

interface II18n {
    detectLanguage(text: string): Promise<{ languages: { language: string; percentage: number }[] }>
}

export interface IBrowser {
    storage: IStorage
    runtime: IRuntime
    i18n: II18n
}

export type BaseThemeType = 'light' | 'dark'
export type ThemeType = BaseThemeType | 'followTheSystem'

export interface IThemedStyleProps {
    theme: Theme
    themeType: BaseThemeType
    isDesktopApp?: boolean
    showLogo?: boolean
}

export interface ISettings {
    automaticCheckForUpdates: boolean
    apiKeys: string
    apiURL: string
    apiURLPath: string
    apiModel: string
    provider: Provider
    chatgptModel: string
    azureAPIKeys: string
    azureAPIURL: string
    azureAPIURLPath: string
    azureAPIModel: string
    miniMaxGroupID: string
    miniMaxAPIKey: string
    moonshotAPIKey: string
    moonshotAPIModel: string
    autoTranslate: boolean
    defaultTranslateMode: Exclude<TranslateMode, 'big-bang'> | 'nop'
    defaultTargetLanguage: string
    alwaysShowIcons: boolean
    hotkey?: string
    displayWindowHotkey?: string
    ocrHotkey?: string
    writingTargetLanguage: string
    writingHotkey?: string
    writingNewlineHotkey?: string
    themeType?: ThemeType
    i18n?: string
    tts?: {
        voices?: {
            lang: LangCode
            voice: string
        }[]
        provider?: TTSProvider
        volume?: number
        rate?: number
    }
    restorePreviousPosition?: boolean
    selectInputElementsText?: boolean
    readSelectedWordsFromInputElementsText?: boolean
    runAtStartup?: boolean
    disableCollectingStatistics?: boolean
    allowUsingClipboardWhenSelectedTextNotAvailable?: boolean
    pinned?: boolean
    autoCollect?: boolean
    hideTheIconInTheDock?: boolean
}
