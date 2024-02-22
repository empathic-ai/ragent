<p align="center">
    <img src="splash.png" alt="Splash">
</p>
<div align="center">
    <a href="https://www.rust-lang.org"><img height=30em src="https://img.shields.io/badge/Rust-%2320232a?style=for-the-badge&logo=rust&logoColor=red&color=141414"></a>
    <a href="https://bevyengine.org"><img height=30em src="https://img.shields.io/badge/Bevy-%2320232a?style=for-the-badge&logo=bevy&logoColor=white&color=141414"></a>
    <a href="https://openai.com"><img height=30em src="https://img.shields.io/badge/OpenAI-%2320232a?style=for-the-badge&logo=openai&logoColor=white&color=141414"></a>
    <a href="https://azure.microsoft.com"><img height=30em src="https://img.shields.io/badge/Azure-%2320232a?style=for-the-badge&logo=microsoftazure&logoColor=0078D4&color=141414"></a>
</div>

# Ragent

**⚠️ Warning: Still in early development. ⚠️**

An open-source Rust-based framework for agents--designed for real-time, multi-modal, multi-agent workflows for the web, desktop devices, mobile devices and embedded systems.

# Usage

```Rust
let config = AgentConfig::new(
    "Butter Robot",
    "You pass butter.",
    vec![
        TaskConfig::new::<SpeakEventArgs>(true)
    ]
);

ChatGPTAgent::new(config);
```

# Integrations
Voice transcription:
- [Deepgram](https://deepgram.com/)
    
LLMs:
- [ChatGPT](https://openai.com/blog/chatgpt)

Speech synthesis:
- [OpenAI](https://platform.openai.com/docs/guides/text-to-speech/)
- [Microsoft Azure](https://azure.microsoft.com/en-us/products/cognitive-services/text-to-speech/)
- [Play.ht](https://play.ht)
- [Eleven Labs](https://elevenlabs.io/)

# Roadmap

- Add simple chat demo.
- Add 2D simulation demo, including multiple social agents.
- Add local integrations using [Candle](https://github.com/huggingface/candle) or something similar.
