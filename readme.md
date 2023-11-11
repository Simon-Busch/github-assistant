
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
And run it with
`github-assistant`

## Navigate:
**Up**: Move up in the list

**Down**: Move down in the list

**Right**: Display comments for the selected issue/PR

**Left**: Hide comments for the selected issue/PR


## Commands
**CTRL + a** : show assignment

**CTRL + c**: show closed

**CTRL + h**: home

**q**: close app

**CTRL + r**: reload content

**ENTER**: open the issue in the browser

**CTRL + p**: show actions

**1**: close issue

## Contributing
Contributions are more than welcome. Please feel free to open a pull request or an issue on the GitHub repository.

## Disclaimer
This application is not affiliated with, maintained, authorized, endorsed, or sponsored by GitHub Inc. or any of its subsidiaries or affiliates.

🚧 This is an alpha version, it's possible that you experience some bugs, still actively working on it. If you do, don't hesitate to open an issue.
