<div align="center">

# Disma

[![Build](https://github.com/vigenere23/disma/actions/workflows/build_pr.yml/badge.svg?branch=master)](https://github.com/vigenere23/disma/actions/workflows/build.yml)
[![Deploy](https://github.com/vigenere23/disma/actions/workflows/deploy_tag.yml/badge.svg)](https://github.com/vigenere23/disma/actions/workflows/deploy.yml)
[![Website](https://img.shields.io/website?down_color=red&down_message=down&up_color=blue&up_message=up&url=https%3A%2F%2Fvigenere23.github.io%2Fdisma%2F)](https://vigenere23.github.io/disma/)
[![disma](https://img.shields.io/crates/v/disma?label=disma)](https://crates.io/crates/disma)
[![disma-cli](https://img.shields.io/crates/v/disma-cli?label=disma-cli)](https://crates.io/crates/disma-cli)

**👨🏼‍🔧 Discord server management has never been easier!**

</div>

## Why ?

In the new context of the pandemic, many educational institutions have shifted their courses online, with the use of communication platforms like Discord. However, managing multiple roles and channels across a Discord server is challenging : there are no way to centrally visualize the information nor to apply synchronized permissions updates. This tool allows you to define a single configuration file to be applied to a server, and it will automatically find the changes that needs to be made, ensuring that your Discord will always be in sync with your config.

## Features

- 📜 **Diff current Discord server config with your desired one**
- 🏗️ **Apply large scale changes to your Discord server**
- ⚡ **Fast, secure and reliable**
- 📋 **Advanced templating techniques**

## Prerequesites

Your will need to setup a [Discord bot](./docs/bot.md) in order to use the CLI tool.

## Modules

- [disma](./disma) : Core Rust library for defining configuration as code and controlling your own orchestrations and implementations.
- [disma-cli](./disma-cli) : A stable and easy to use command line interface for defining configuration as simple YAML files. Still allows for templating to simplify the configuration.

## Documentation

- [Configuration file format](./docs/config.md)
- [Roadmap](https://github.com/vigenere23/disma/milestones)
