use git2::{BranchType, Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::PrismResult;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitContext {
    pub branch: Option<String>,
    pub has_conflict: bool,
    pub ahead: usize,
    pub behind: usize,
    pub dirty: bool,
}

pub fn detect_git_context(path: &Path) -> PrismResult<GitContext> {
    let repo = Repository::discover(path)?;
    let head = repo.head().ok();
    let branch = head
        .as_ref()
        .and_then(|r| r.shorthand())
        .map(|s| s.to_string());

    let statuses = repo.statuses(None)?;
    let has_conflict = statuses.iter().any(|s| s.status().is_conflicted());
    let dirty = statuses.iter().any(|status| {
        let s = status.status();
        s.is_wt_modified() || s.is_index_modified() || s.is_wt_new()
    });

    let (ahead, behind) = compute_ahead_behind(&repo, head.as_ref())?;

    Ok(GitContext {
        branch,
        has_conflict,
        ahead,
        behind,
        dirty,
    })
}

fn compute_ahead_behind(
    repo: &Repository,
    head: Option<&git2::Reference<'_>>,
) -> PrismResult<(usize, usize)> {
    if let Some(head) = head {
        if let Some(name) = head.shorthand() {
            if let Ok(local) = repo.find_branch(name, BranchType::Local) {
                if let Ok(upstream) = local.upstream() {
                    if let (Some(local_oid), Some(remote_oid)) = (
                        local.into_reference().target(),
                        upstream.into_reference().target(),
                    ) {
                        let (ahead, behind) = repo.graph_ahead_behind(local_oid, remote_oid)?;
                        return Ok((ahead, behind));
                    }
                }
            }
        }
    }
    Ok((0, 0))
}
