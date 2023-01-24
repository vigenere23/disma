# Roadmap

## V1

### Features

- [x] Roles: add, update or remove roles
- [x] Categories: add, update or remove categories
- [x] Channels: add, update or remove channels
- [x] Allow additional roles, categories or members
- [ ] Members: invite or kick members ([#25](https://github.com/vigenere23/disma/issues/25))
- [ ] Other small but important features (see the [Stable features milestone](https://github.com/vigenere23/disma/milestone/2))

### Technical

- [ ] Bulletproof test all important concepts :
  - [ ] Diffs (comparisons)
  - [ ] Diff commands
  - [ ] Commands to requests convertion
  - [ ] Responses to existing guild convertion
  - [x] Config to awaiting guild convertion
  - [x] Existing guild to config convertion
- [ ] Exception handling strategies for :
  - [ ] Discord API
    - [ ] Server errors (Discord's end)
    - [ ] Permission errors (user's bot configuration) - see bug [#26](https://github.com/vigenere23/disma/issues/26)
  - [ ] Config validation
  - [ ] Runtime errors (ex: changes in existing state)
    - [ ] Fatal
    - [ ] Non-fatal

### Documentation

- [ ] Library
  - [ ] Rust doc (with doc-tests if needed)
- [ ] Cli
  - [ ] Examples
  - [ ] Guide
  - [x] JSON and YAML schemas for config
