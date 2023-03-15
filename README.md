<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/1651790/224081217-86521beb-1b69-4071-b195-f2ce0bb33db7.png">
  <img alt="NebulaGraph Data Intelligence Suite(ngdi)" src="https://user-images.githubusercontent.com/1651790/224081979-d3aa7867-94a6-4a85-a5d7-603e02360cee.png">
</picture>
<p align="center">
    <br> English | <a href="README-CN.md">中文</a>
</p>
<p align="center">
    <em>The translator that does more than just translation - powered by OpenAI.</em>
</p>

<p align="center">
<a href="LICENSE" target="_blank">
    <img alt="MIT License" src="https://img.shields.io/github/license/yetone/openai-translator.svg?style=flat-square" />
</a>

<!-- TypeScript Badge -->
<img alt="TypeScript" src="https://img.shields.io/badge/-TypeScript-blue?style=flat-square&logo=typescript&logoColor=white" />

<!-- Rust Badge -->
<img alt="Rust" src="https://img.shields.io/badge/-Rust-orange?style=flat-square&logo=rust&logoColor=white" />

<a href="https://chrome.google.com/webstore/detail/openai-translator/ogjibjphoadhljaoicdnjnmgokohngcc" target="_blank">
<img alt="Chrome" src="https://img.shields.io/badge/-Chrome-green?style=flat-square&logo=google-chrome&logoColor=white" />
</a>

<a href="https://github.com/yetone/openai-translator/releases" target="_blank">
<img alt="Firefox" src="https://img.shields.io/badge/-Firefox-orange?style=flat-square&logo=firefox&logoColor=white" />
</a>

<a href="https://github.com/yetone/openai-translator/releases" target="_blank">
<img alt="macOS" src="https://img.shields.io/badge/-macOS-black?style=flat-square&logo=apple&logoColor=white" />
</a>

<a href="https://github.com/yetone/openai-translator/releases" target="_blank">
<img alt="Windows" src="https://img.shields.io/badge/-Windows-blue?style=flat-square&logo=windows&logoColor=white" />
</a>

<a href="https://github.com/yetone/openai-translator/releases" target="_blank">
<img alt="Linux" src="https://img.shields.io/badge/-Linux-yellow?style=flat-square&logo=linux&logoColor=white" />
</a>

</p>

# Why Yet another Translator

I have developed a [Bob](https://bobtranslate.com/) [plugin](https://github.com/yetone/bob-plugin-openai-translator) that utilizes ChatGPT API to provide global word translation on macOS. However, since not all users have access to macOS to benefit from the plugin, I have created this project!

# More than just a browser extension

What began as a Chrome extension has now evolved into a multi-platform desktop app that I am currently developing.

<img width="560" src="https://user-images.githubusercontent.com/1206493/223899374-ff386436-63b8-4618-afdd-fed2e6b48d56.png" />

# More than just translation

What began as a translation tool has now evolved to include surprisingly effective word polishing and summarization capabilities, ~~accidentally~~.

# How to use

<img width="800" src="https://user-images.githubusercontent.com/1206493/223200182-6a1d2a02-3fe0-4723-bdae-99d8b7212a33.gif" />

# Features

1. It offers three modes: translation, polishing and summarization.
2. Our tool allows for mutual translation, polishing and summarization across 55 different languages.
3. Streaming mode is supported!
4. It allows users to customize their translation text.
5. One-click copying
6. Text-to-Speech (TTS)
7. Available on all platforms (Windows, macOS, and Linux) for both browsers and Desktop

# Preparation

-   (required) Apply for an OpenAI API key [here](https://platform.openai.com/account/api-keys)
-   (optional) If you cannot access OpenAI, you can use the OpenAI API Proxy.

# Installation

## Windows

### Install via [winget](https://github.com/microsoft/winget-cli)

```sh
winget install yetone.OpenAITranslator
```

### Install Manually

1. Download the installation package ending in `.msi` from the [Latest Release](https://github.com/yetone/openai-translator/releases/latest) page.
2. Double click the downloaded file to install it.
3. If prompted as unsafe, you can click on `More Info` -> `Run Anyway` to proceed with the installation.
4. Ready to use!

## MacOS

### Install Manually

1. Go to the [Latest Release](https://github.com/yetone/openai-translator/releases/latest) page and download the corresponding chip's `.dmg` installation package.
2. Double click the downloaded file to install it.
3. Ready to use!

### Troubleshooting

-   "OpenAI Translator" can’t be opened because the developer cannot be verified.
    <img width="300" src="https://user-images.githubusercontent.com/1206493/223916804-45ce3f34-6a4a-4baf-a0c1-4ab5c54c521f.png" />

    -   Click the `Cancel` button, then go to the `Settings` -> `Privacy and Security` page, click the `Still Open` button, and then click the `Open` button in the pop-up window. After that, there will be no more pop-up warnings when opening `OpenAI Translator`. 🎉
        <img width="500" src="https://user-images.githubusercontent.com/1206493/223916970-9c99f15e-cf61-4770-b92d-4a78f980bb26.png" /> <img width="200" src="https://user-images.githubusercontent.com/1206493/223917449-ed1ac19f-c43d-4b13-9888-79ba46ceb862.png" />

    -   If you cannot find the above options in `Privacy & Security`. Open `Terminal.app` and enter the following command (you may need to enter a password halfway through), then restart `OpenAI Translator`:

        ```sh
        sudo xattr -d com.apple.quarantine /Applications/OpenAI\ Translator.app
        ```

-   If you encounter a permission prompt every time you open it, or if you cannot perform a shortcut translation, please go to `Settings` -> `Privacy & Security` -> `Supporting Features` to remove OpenAI Translator, and then re-add OpenAI Translator.

    <img width="500" src="https://user-images.githubusercontent.com/1206493/224536148-eec559bf-4d99-48c1-bbd3-2cc105aff084.png" />
    <img width="600" src="https://user-images.githubusercontent.com/1206493/224536277-4200f58e-8dc0-4c01-a27a-a30d7d8dc69e.gif" />

## Browser Extension

1. Visit the [Chrome Web Store](https://chrome.google.com/webstore/detail/openai-translator/ogjibjphoadhljaoicdnjnmgokohngcc) to install this plugin.
2. Click on the OpenAI Translator icon in the browser plugin list, and enter the obtained API KEY into the configuration interface that pops up from this plugin.
   <img width="600" src="https://user-images.githubusercontent.com/1206493/222958165-159719b4-28a5-44a4-b700-567786df7f03.png" />
3. Refresh the page in the browser to enjoy the smooth translation experience 🎉!

# License

[LICENSE](./LICENSE)

# Buy me a coffee

<div align="center">
<img height="360" src="https://user-images.githubusercontent.com/1206493/220753437-90e4039c-d95f-4b6a-9a08-b3d6de13211f.png" />
<img height="360" src="https://user-images.githubusercontent.com/1206493/220756036-d9ac4512-0375-4a32-8c2e-8697021058a2.png" />
</div>

# Star History

[![Star History Chart](https://api.star-history.com/svg?repos=yetone/openai-translator&type=Date)](https://star-history.com/#yetone/openai-translator&Date)
