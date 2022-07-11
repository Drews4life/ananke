use std::borrow::Cow;
use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;
use crate::task_runner::TaskRunner;
use ananke::get_child_path_from_string;

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

pub struct Microfrontends(pub Vec<Microfrontend>);

impl Microfrontend {
    pub fn get_branch(self: &Self) -> String {
            match self.version_type {
                VersionType::Latest => "master".to_owned(),
                VersionType::Tag => format!("tag/{}", self.version),
                _ => self.version.clone(),
            }
    }

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
        let version = if raw_version == ""  { "current".to_owned() } else { raw_version };
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
        } else if version == "" {
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

    fn build_repo_url(&self, target_host: &String) -> String {
        format!("git@{}:{}/{}.git", target_host, self.project_group, self.project_name)
    }
}

impl fmt::Debug for Microfrontend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {:?}", self.project_group, self.project_name, self.version, self.version_type)
    }
}

impl Microfrontends {
    pub fn create_fetch_projects_info_tasks(&self, target_host: &String, pull: bool) -> Vec<Box<(dyn Fn() + Send)>>{
        self.0
            .iter()
            .map(|mfe| {
                let repo_url = mfe.build_repo_url(target_host);
                let project_name = mfe.project_name.clone();
                let version = mfe.get_branch().to_owned();
                let was_fetched = get_child_path_from_string(&project_name).exists();
                let use_current_version = mfe.should_use_current_version();
                let new_branch = mfe.should_create_new_branch();

                Box::new(move || {
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
                }) as Box<(dyn Fn() + Send)>
            }).collect::<Vec<_>>()
    }

    pub fn create_install_dependency_tasks(&self, force_update_all: bool) -> Vec<Box<(dyn Fn() + Send)>> {
        self.0
            .iter()
            .filter(|mfe|
                !get_child_path_from_string(
                &format!("{}/node_modules", &mfe.project_name)
                ).exists() || force_update_all
            )
            .map(|mfe| {
                let project_name = mfe.project_name.clone();

                Box::new(move || {
                    TaskRunner::npm_install(&project_name);
                }) as Box<(dyn Fn() + Send)>
            }).collect::<Vec<_>>()
    }

    pub fn create_run_tasks(&self) -> Vec<Box<(dyn Fn() + Send)>> {
        self.0
            .iter()
            .map(|mfe| {
                let project_name = mfe.project_name.clone();

                Box::new(move || {
                    TaskRunner::npm_run(&project_name);
                }) as Box<(dyn Fn() + Send)>
            }).collect::<Vec<_>>()
    }

    pub fn log(&self) {
        println!("Microfrontends to fetch:");
        self.0.iter().for_each(|mf| println!("{:?}", mf));
    }
}

