# Changelog

## disma-cli/v0.8.9 (2023-08-20)

*No changelog available for this tag.* 

## disma/v0.13.2 (2023-08-20)

- feat: added permissions check when listing guilds (#101) [80ea724]
- chore: updating dependencies [0b3caa3] 

## disma-cli/v0.8.8 (2023-08-18)

*No changelog available for this tag.* 

## disma/v0.13.1 (2023-08-18)

- chore: added build step in deploy_master [dfcde7d]
- feat: Added members count when listing guilds (#98) [137eecc] 

## disma-cli/v0.8.7 (2023-08-18)

*No changelog available for this tag.* 

## disma/v0.13.0 (2023-08-18)

- Refactor: New change events for channels (#88) [56d16d2]
- refactor: added new role commands (#89) [0ee866e]
- chore: Ignoring tests dependencies in code coverage (#91) [7811ea6]
- Refactor: New category commands (#92) [68ace31]
- refactor: Using new commands for extra items (#94) [586c927]
- chore: building crates before publishing [fafc5e2] 

## disma-cli/v0.8.6 (2023-08-10)

*No changelog available for this tag.* 

## disma/v0.12.0 (2023-08-10)

- fix: updated schema.json [d6f4233]
- refactor: returning references instead of owned types [f27c521]
- refactor: implemented comparison for roles lists [cead16d]
- refactor: created channel unique name [6244007]
- chore: removed partial eq implementation that is no longer usefull [9cf5e0f]
- refactor: added comparing logic for roles list + simplified update command creation [c9ebb46]
- refactor: added logic to categories lists for comparing [3074166]
- doc: added cli code coverage badge [6277903]
- chore(deps-dev): Bump vite from 4.3.5 to 4.3.9 in /site (#69) [30eb4a1]
- refactor: added new usecases for listing and applying changes for roles (#74) [c7d759f]
- feat: New API usecase for applying changes - roles (#75) [71124b6]
- refactor: new list changes usecase for categories (#76) [b4a099c]
- refactor: implemented  new apply changes usecase for categories (#77) [04aca5e]
- refactor: new list changes usecase for channels (#78) [c18ae4e]
- refactor: implemented new apply changes usecase for channels (#79) [0f28b94]
- refactor: modules (#82) [d35d92e]
- refactor: commander returning results (#84) [37026a8]
- refactor: Using new services (#83) [bf8e8f0]
- fix: better error message for clashing channels names [8101000] 

## disma-cli/v0.8.5 (2023-07-27)

*No changelog available for this tag.* 

## disma/v0.11.4 (2023-07-27)

- chore: added release badge [cf16673]
- chore: added crates badges on homepage [ad455ef]
- added site [6732bcd]
- added gh action for deploying site [3193fbe]
- chore: updated rust [457ebd5]
- chore: added website badge [abad03f]
- feat(site): added auto-typing command line preview [647a8d3]
- feat(site): added powers + fixed styling [531e969]
- feat(site): added favicons [d87ff6b]
- feat(site): added og-image + fixed apple icon [67c63d6]
- fix(site): made og image remote [060e052]
- fix(site): adjusted favicons quality [4cf635f]
- fix: added missing properties for channels in json schema [9f61ebd]
- chore(deps): bump openssl from 0.10.45 to 0.10.48 (#57) [7c15ecb]
- chore: updated rust + dependencies [afbff9b]
- chore(deps): bump h2 from 0.3.16 to 0.3.17 (#58) [a23db8f]
- fix: footer links [51caab9]
- chore: updated site dependencies [2c104c7]
- fix: clippy errors with clap [d889a85]
- chore: removing unused duplicated method (#62) [3f1f490]
- chore: Better segregated workflows (#63) [4662081]
- fix: duplicated Workflow names [5c04fb3]
- refactor: Replaced unwraps with Results for discord api implementation (#64) [cb6ad70]
- chore: Updated badges in README [cfd7105]
- refactor: using new github pages [2c1281e]
- chore: moved tests to the right file [f7b7e71]
- chore(deps): Bump openssl from 0.10.52 to 0.10.55 (#65) [86e6002]
- feat: Added new permissions (#66) [25e5c1e] 

## disma-cli/v0.8.4 (2023-01-23)

*No changelog available for this tag.* 

## disma/v0.11.3 (2023-01-23)

- Chore: Improved and updated doc (#53) [e4891a2]
- chore: Fixed READMEs [7585d96] 

## disma-cli/v0.8.3 (2023-01-22)

- chore: removing coveralls [f968736]
- chore: update doc [9458cb1]
- chore: added github release [d01e563] 

## disma-cli/v0.8.2 (2023-01-21)

*No changelog available for this tag.* 

## disma/v0.11.2 (2023-01-21)

- Fix: coverage upload (#50) [a6395ce]
- chore: added coverage badge [4f43cf7]
- Feat: Extra channels sync permissions (#51) [87a69ca] 

## disma-cli/v0.8.1 (2023-01-17)

*No changelog available for this tag.* 

## disma/v0.11.1 (2023-01-17)

- Fix: Updated doc + small fixes following #47 (#48) [fe55b91] 

## disma-cli/v0.8.0 (2023-01-17)

*No changelog available for this tag.* 

## disma/v0.11.0 (2023-01-17)

- Feat: Sync permissions (#47) [336eeeb] 

## disma-cli/v0.7.3 (2023-01-16)

- Fix: Unrecognized CLI characters (#46) [0d89e02] 

## disma-cli/v0.7.2 (2023-01-16)

*No changelog available for this tag.* 

## disma/v0.10.0 (2023-01-16)

- Refact: Moved config data and transformation to core lib (#45) [1ee1fb3] 

## disma-cli/v0.7.1 (2023-01-15)

*No changelog available for this tag.* 

## disma/v0.9.1 (2023-01-15)

- Fix: Keep extra channels in categories (#43) [17b0f33] 

## disma-cli/v0.7.0 (2023-01-11)

*No changelog available for this tag.* 

## disma/v0.9.0 (2023-01-11)

- chore: Added concurrency check to prevent concurrent deploys [040c41e]
- triggering CI on pull requests [b99706e]
- Fix is_mentionable in the examples (#23) [943ae4c]
- chore: updating disma-cli and rust [72fe554]
- Moved ci scripts to 'scripts' folder (#30) [629ba3e]
- Chore: Code coverage (#32) [527dbaa]
- Chore: switching to coveralls (#33) [e0562d8]
- Bump tokio from 1.20.1 to 1.24.1 (#31) [54212ff]
- Fixed codecov flags [af141ce]
- Feat: Handle extra items in existing guild (#24) [7f2a76c]
- [disma-cli] Chore: Adding critical tests (#27) [7ee9747] 

## disma-cli/v0.6.5 (2022-11-22)

*No changelog available for this tag.* 
