
# Github Assistant
Organize and manage your GitHub issues and pull requests with ease using our Rust-based application. Stay on top of your work by quickly accessing, filtering, and browsing issues/PRs directly from your terminal. With built-in functionality to display comments, assignments, and more, you'll have all the information you need at your fingertips.


## Get started
To use the application, you need to set your GitHub username and personal access token as environment variables:

```bash
export GITHUB_USERNAME=myusername
export GITHUB_TOKEN=mytoken
```
Replace myusername and mytoken with your GitHub username and personal access token, respectively.

Then either run it locally with
`cargo run`
or install it from homebrew:
`brew install github-assistant`

## Navigate:
**Up**: Move up in the list

**Down**: Move down in the list

**Right**: Display comments for the selected issue/PR

**Left**: Hide comments for the selected issue/PR


## Commands
**A** : show assignment

**C**: show closed

**Q**: close app

**ENTER**: open the issue in the browser

**p**: show actions

**1**: close issue

## Contributing
We welcome contributions to the project. Please feel free to open a pull request or an issue on the GitHub repository.

## Disclaimer
This application is not affiliated with, maintained, authorized, endorsed, or sponsored by GitHub Inc. or any of its subsidiaries or affiliates.
