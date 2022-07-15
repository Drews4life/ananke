use std::borrow::Cow;
use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;
use crate::task_runner::TaskRunner;
use ananke::get_child_path_from_string;
use crate::configuration::Configuration;

#[derive(Debug, PartialEq)]
pub enum VersionType {
    Latest,
    CommitHash,
    Tag,
    Branch,
    Current,
}

pub struct Microfrontend {
    pub project_name: String,
    pub project_group: String,
    pub version: String,
    pub version_type: VersionType,
}

impl Microfrontend {
    pub fn from_raw_format(raw: String) -> Self {
        // mfe consist of `group` and `name` separate by forward slash,
        // and `version` separated by `@` (i.e. `betbook/shell@1.4.2`)
        // Version can be absent.
        let mut unparsed_mfe = raw.split(&['/', '@'][..]);
        let project_group = unparsed_mfe.next().unwrap().to_owned();
        let project_name = unparsed_mfe.next().unwrap().to_owned();
        // joining potentially split branch name
        let raw_version = unparsed_mfe
            .enumerate()
            .fold("".to_owned(), |curr, (idx, next)| {
                let branch_name_separator = "/";

                if idx == 0 {
                    curr + next
                } else {
                    curr + branch_name_separator + next
                }
            });
        let version = if raw_version.is_empty()  { "current".to_owned() } else { raw_version };
        let version_type = Self::get_version_type(&version);

        Microfrontend {
            project_name,
            project_group,
            version,
            version_type
        }
    }

    fn get_version_type<'a, T>(version: T) -> VersionType where T: Into<Cow<'a, str>>, {
        lazy_static! {
            static ref COMMIT_SHA1_REGEX: Regex = Regex::new(r"\b[0-9a-f]{5,40}\b").unwrap();
            static ref TAG_REGEX: Regex = Regex::new(r"[0-9.].+$").unwrap();
        }

        let version: Cow<'a, str> = version.into();

        if version == "latest" {
            VersionType::Latest
        } else if version.is_empty() {
            VersionType::Current
        } else if COMMIT_SHA1_REGEX.is_match(version.as_ref()) {
            VersionType::CommitHash
        } else if TAG_REGEX.is_match(version.as_ref()){
            VersionType::Tag
        } else {
            // Arbitrary string will be treated as branch
            VersionType::Branch
        }
    }

    pub fn init(&self, config: &Configuration) {
        self.fetch(&config.target_host, config.pull);
        self.install_dependencies(config.force_update_all);
        self.run()
    }

    fn fetch(&self, target_host: &str, pull: bool) {
        let repo_url = self.build_repo_url(target_host);
        let project_name = self.project_name.clone();
        let version = self.get_branch();
        let was_fetched = get_child_path_from_string(&project_name).exists();
        let use_current_version = self.should_use_current_version();
        let new_branch = self.should_create_new_branch();

        if !was_fetched {
            TaskRunner::git_clone(&repo_url);
        }

        TaskRunner::git_fetch(&project_name);

        if pull {
            TaskRunner::git_pull(&project_name);
        }

        if !use_current_version {
            TaskRunner::git_checkout(&project_name, &version, new_branch);
        }
    }

    fn install_dependencies(&self, force_update_all: bool) {
        if !get_child_path_from_string(
            &format!("{}/node_modules", &self.project_name)
        ).exists() || force_update_all {
            TaskRunner::npm_install(&self.project_name);
        }
    }

    fn run(&self) {
        TaskRunner::npm_run(&self.project_name);
    }

    fn get_branch(&self) -> String {
        match self.version_type {
            VersionType::Latest => "master".to_owned(),
            VersionType::Tag => format!("tag/{}", self.version),
            _ => self.version.clone(),
        }
    }

    fn should_create_new_branch(&self) -> bool {
        match self.version_type {
            VersionType::Current => false,
            VersionType::Latest => false,
            VersionType::Branch => false,
            VersionType::CommitHash => true,
            VersionType::Tag => true,
        }
    }

    fn should_use_current_version(&self) -> bool {
        self.version_type == VersionType::Current
    }

    fn build_repo_url(&self, target_host: &str) -> String {
        format!("git@{}:{}/{}.git", target_host, self.project_group, self.project_name)
    }
}

impl fmt::Debug for Microfrontend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {:?}", self.project_group, self.project_name, self.version, self.version_type)
    }
}