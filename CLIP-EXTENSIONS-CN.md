桌面端应用全局划词插件
----------------------

<p align="center">
    <br> <a href="CLIP-EXTENSION.md">English</a> | 中文
</p>

划词翻译是本软件的杀手锏功能，对于浏览器插件来说，浏览器提供了简单的 API 来获得选中的文本，但是对于桌面端应用来说，各个操作系统都没有统一的 API 来获得选中的文本。

通常都是使用剪切板来获得选中的文本，但是这样会在一些应用中造成剪切板混乱等 bug，在 macOS 中还会因为没有选中文本就按下 cmd+c 快捷键导致发出警告声。幸运的是，各个操作系统中已经有很多成熟的划词软件，而且它们有完善的插件机制，所以 OpenAI Translator 特地开发了针对这些划词软件的插件来让用户无痛地使用划词翻译。

# macOS

## PopClip

[PopClip](https://pilotmoon.com/popclip/) 是 macOS 上成熟的划词软件，它提供了完善的插件机制，我们提供了它的插件，安装步骤如下：

* 1. 下载并安装 [PopClip](https://pilotmoon.com/popclip/)
* 2. 下载 [openai-translator.popclipextz](https://github.com/yetone/openai-translator/releases/latest/download/openai-translator.popclipextz)
* 3. 双击下载完毕的 openai-translator.popclipextz，点击弹出窗口中的 Install "OpenAI Translator" 按钮即可安装完毕
    
    <img src="https://user-images.githubusercontent.com/1206493/240260692-8af6141a-3dba-4775-921d-505223addf9e.png" />

* 4. 在 PopClip 中开启 OpenAI Translator
    
    <img src="https://user-images.githubusercontent.com/1206493/240258859-c4f2ec91-255f-414c-a4a4-aca25fceb0b5.png" />

# Windows

## SnipDo

* 1. 下载并安装 [SnipDo](https://apps.microsoft.com/store/detail/snipdo/9NPZ2TVKJVT7)
* 2. 下载 [openai-translator.pbar](https://github.com/yetone/openai-translator/releases/latest/download/openai-translator.pbar)
* 3. 双击下载完毕的 openai-translator.pbar 即可安装
* 4. 在 SnipDo 的设置页面中启用 OpenAI Translator

