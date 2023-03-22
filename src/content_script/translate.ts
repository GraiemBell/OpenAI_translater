/* eslint-disable camelcase */
import * as utils from '../common/utils'
import * as lang from './lang'
import { fetchSSE } from './utils'

export type TranslateMode = 'translate' | 'polishing' | 'summarize' | 'analyze' | 'explain-code'
export type Provider = 'OpenAI' | 'Azure'
export type APIModel = 'gpt-3.5-turbo' | 'gpt-3.5-turbo-0301' | 'gpt-4' | 'gpt-4-0314' | 'gpt-4-32k' | 'gpt-4-32k-0314'

export interface TranslateQuery {
    text: string
    selectedWord: string
    detectFrom: string
    detectTo: string
    mode: TranslateMode
    onMessage: (message: { content: string; role: string }) => void
    onError: (error: string) => void
    onFinish: (reason: string) => void
    signal: AbortSignal
}

export interface TranslateResult {
    text?: string
    from?: string
    to?: string
    error?: string
}

const chineseLangs = ['zh-Hans', 'zh-Hant', 'wyw', 'yue']

export async function translate(query: TranslateQuery) {
    const trimFirstQuotation = !query.selectedWord
    const settings = await utils.getSettings()
    const fromChinese = chineseLangs.indexOf(query.detectFrom) >= 0
    const toChinese = chineseLangs.indexOf(query.detectTo) >= 0
    let systemPrompt = 'You are a translation engine that can only translate text and cannot interpret it.'
    let assistantPrompt = `translate from ${lang.langMap.get(query.detectFrom) || query.detectFrom} to ${
        lang.langMap.get(query.detectTo) || query.detectTo
    }`
    switch (query.mode) {
        case 'translate':
            if (query.detectTo === 'wyw' || query.detectTo === 'yue') {
                assistantPrompt = `翻译成${lang.langMap.get(query.detectTo) || query.detectTo}`
            }
            if (fromChinese) {
                if (query.detectTo === 'zh-Hant') {
                    assistantPrompt = '翻譯成台灣常用用法之繁體中文白話文'
                } else if (query.detectTo === 'zh-Hans') {
                    assistantPrompt = '翻译成简体白话文'
                } else if (query.text.length < 5 && settings.defaultTargetLanguage === 'zh-Hans') {
                    // 当用户的默认语言为中文时，查询中文词组（不超过5个字），展示多种翻译结果，并阐述适用语境。
                    systemPrompt = `你是一个翻译引擎，请将给到的文本翻译成${
                        lang.langMap.get(query.detectTo) || query.detectTo
                    }。请列出3种（如果有）最常用翻译结果：单词或短语，并列出对应的适用语境（用中文阐述）、音标、词性、双语示例。按照下面格式用中文阐述：
                        <序号><单词或短语> · /<音标>
                        [<词性缩写>] <适用语境（用中文阐述）>
                        例句：<例句>(例句翻译)`
                    assistantPrompt = ''
                }
            }
            if (toChinese && !query.text.includes(' ')) {
                // 翻译为中文时，增加单词模式，可以更详细的翻译结果，包括：音标、词性、含义、双语示例。
                systemPrompt = `你是一个翻译引擎，请将翻译给到的文本，只需要翻译不需要解释。当且仅当文本只有一个单词时，请给出单词原始形态（如果有）、单词的语种、对应的音标（如果有）、所有含义（含词性）、双语示例，至少三条例句，请严格按照下面格式给到翻译结果：
                <原始文本>
                [<语种>] · / <单词音标>
                [<词性缩写>] <中文含义>]
                例句：
                <序号><例句>(例句翻译)`
            }
            if (query.selectedWord) {
                // 在选择的句子中，选择特定的单词。触发语境学习功能。
                systemPrompt = `你是一位${
                    lang.langMap.get(query.detectFrom) || query.detectFrom
                }词义语法专家，你在教我${lang.langMap.get(query.detectFrom) || query.detectFrom}，我给你一句${
                    lang.langMap.get(query.detectFrom) || query.detectFrom
                }句子，和这个句子中的一个单词，请用${
                    lang.langMap.get(query.detectTo) || query.detectTo
                }帮我解释一下，这个单词在句子中的意思和句子本身的意思,如果单词在这个句子中是习话的一部分，请解释这句句子中的习话，并举几个相同意思的${
                    lang.langMap.get(query.detectFrom) || query.detectFrom
                }例句,并用${
                    lang.langMap.get(query.detectTo) || query.detectTo
                }解释例句。如果你明白了请说同意，然后我们开始。`
                assistantPrompt = '好的，我明白了，请给我这个句子和单词。'
                query.text = `句子是：${query.text}\n单词是：${query.selectedWord}`
            }
            break
        case 'polishing':
            systemPrompt = 'Revise the following sentences to make them more clear, concise, and coherent.'
            if (fromChinese) {
                assistantPrompt = `使用 ${lang.langMap.get(query.detectFrom) || query.detectFrom} 语言润色此段文本`
            } else {
                assistantPrompt = `polish this text in ${lang.langMap.get(query.detectFrom) || query.detectFrom}`
            }
            break
        case 'summarize':
            systemPrompt = "You are a text summarizer, you can only summarize the text, don't interpret it."
            if (toChinese) {
                assistantPrompt = '用最简洁的语言使用中文总结此段文本'
            } else {
                assistantPrompt = `summarize this text in the most concise language and must use ${
                    lang.langMap.get(query.detectTo) || query.detectTo
                } language!`
            }
            break
        case 'analyze':
            systemPrompt = 'You are a translation engine and grammar analyzer.'
            if (toChinese) {
                assistantPrompt = `请用中文翻译此段文本并解析原文中的语法`
            } else {
                assistantPrompt = `translate this text to ${
                    lang.langMap.get(query.detectTo) || query.detectTo
                } and explain the grammar in the original text using ${
                    lang.langMap.get(query.detectTo) || query.detectTo
                }`
            }
            break
        case 'explain-code':
            systemPrompt =
                'You are a code explanation engine, you can only explain the code, do not interpret or translate it. Also, please report any bugs you find in the code to the author of the code.'
            if (toChinese) {
                assistantPrompt =
                    '用最简洁的语言使用中文解释此段代码、正则表达式或脚本。如果内容不是代码，请返回错误提示。如果代码有明显的错误，请指出。'
            } else {
                assistantPrompt = `explain the provided code, regex or script in the most concise language and must use ${
                    lang.langMap.get(query.detectTo) || query.detectTo
                } language! If the content is not code, return an error message. If the code has obvious errors, point them out.`
            }
            break
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const body: Record<string, any> = {
        model: settings.apiModel,
        temperature: 0,
        max_tokens: 1000,
        top_p: 1,
        frequency_penalty: 1,
        presence_penalty: 1,
        stream: true,
    }

    const apiKey = await utils.getApiKey()
    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
    }

    switch (settings.provider) {
        case 'OpenAI':
            headers['Authorization'] = `Bearer ${apiKey}`
            body['messages'] = [
                {
                    role: 'system',
                    content: systemPrompt,
                },
                {
                    role: 'user',
                    content: assistantPrompt,
                },
                { role: 'user', content: `"${query.text}"` },
            ]
            break

        case 'Azure':
            headers['api-key'] = `${apiKey}`
            body[
                'prompt'
            ] = `<|im_start|>system\n${systemPrompt}\n<|im_end|>\n<|im_start|>user\n${assistantPrompt}\n${query.text}\n<|im_end|>\n<|im_start|>assistant\n`
            body['stop'] = ['<|im_end|>']
            break
    }

    let isFirst = true

    await fetchSSE(`${settings.apiURL}${settings.apiURLPath}`, {
        method: 'POST',
        headers,
        body: JSON.stringify(body),
        signal: query.signal,
        onMessage: (msg) => {
            let resp
            try {
                resp = JSON.parse(msg)
                // eslint-disable-next-line no-empty
            } catch {
                query.onFinish('stop')
                return
            }
            const { choices } = resp
            if (!choices || choices.length === 0) {
                return { error: 'No result' }
            }
            const { finish_reason: finishReason } = choices[0]
            if (finishReason) {
                query.onFinish(finishReason)
                return
            }

            let targetTxt = ''
            switch (settings.provider) {
                case 'OpenAI': {
                    const { content = '', role } = choices[0].delta
                    targetTxt = content

                    if (trimFirstQuotation && isFirst && targetTxt && ['“', '"', '「'].indexOf(targetTxt[0]) >= 0) {
                        targetTxt = targetTxt.slice(1)
                    }

                    if (!role) {
                        isFirst = false
                    }

                    query.onMessage({ content: targetTxt, role })
                    break
                }
                case 'Azure':
                    targetTxt = choices[0].text
                    console.log(resp)

                    if (trimFirstQuotation && isFirst && targetTxt && ['“', '"', '「'].indexOf(targetTxt[0]) >= 0) {
                        targetTxt = targetTxt.slice(1)
                    }

                    query.onMessage({ content: targetTxt, role: '' })
                    break
            }
        },
        onError: (err) => {
            const { error } = err
            query.onError(error.message)
        },
    })
}
