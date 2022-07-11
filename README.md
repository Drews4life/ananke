# Ananke
`ananke` is a cli microfrontend linking tool that allows you to run all microfrontends without manually setting up each and every of them.

## Installation 

**[Homebrew](https://brew.sh)**
```sh
$ brew install ananke
```

**[Cargo](https://doc.rust-lang.org/cargo) (TODO)**
```sh
$ cargo install ananke
```

## Usage
You can link microfrontends by calling `link` command. All fetched repos will be saved to current directory from which linking command was called.
```sh
$ ananke link
```

### flags
```sh
-m --microfrontends      <microfrontends>... list of MFEs to link. Pattern as follows <{project_group}/{project_name}@{version}>
-t --target-repo         <target>            from which repository microfrontends should be fetched
-f --force-update-all                        forcefully call npm install on each MFE
-p --pull                                    pull latest changes from source control for each MFE
```

## Example
In case if:
* Our source control host is `git.company-name.com`
* Project group is `sportsbook`
* Project names are `shell` of specifc-branch, `bet` tag version 1.4.2

Then command should be called as follows:
```sh
ananke link -t git.company-name.com -m sportsbook/shell@branch-name sportsbook/bet@1.4.2
```

### Note
Version can be of four types:
* git tag `sportsbook/bet@1.4.2`
* commit sha1 `sportsbook/bet@9eabf5b536662000f79978c4d1b6e4eff5c8d785`
* branch name `sportsbook/bet@feat/specific-branch`
* latest `sportsbook/bet@latest` (it will checkout to main project branch)
* Omitted `sportsbook/bet`. In this case it will run project either on current branch or on main branch, if it was not fetched previously.


## Improvements
See gitlab issues.