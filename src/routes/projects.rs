use crate::JspResponse;
use crate::shows::get_showlist;
use rocket_contrib::json::Json;
use std::path::PathBuf;
use jsp::{cli, JGraph, JSPError, get_graph, get_graph_from_fn};
use log;
use levelspecter::{LevelSpec, LevelType};

/// Retrieve the list of projects
#[get("/projects")]
pub fn projects() -> Json<JspResponse<Vec<String>, String>> {
   Json(JspResponse::new(get_showlist()))
}

/// Retrieve a particular project
#[get("/projects/<project>")]
pub fn project(project: String) -> Json<JspResponse<JGraph, String>> {
    let r =  get_graph_from_fn(
                None,
                &vec![project.as_str()],//.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                |_| get_path_to_template(project.as_str()),
            );
        
    match r {
        Ok((graph, _keymap, _regexmap)) => {
            return Json(JspResponse::new(graph))
        }
        Err(e) => {return Json(JspResponse::error(e.to_string()))}
    }  
}


#[inline]
fn get_path_to_template(show_str: &str) -> Result<PathBuf, JSPError> {
    let (graph, keymap, _regexmap) = get_graph(None)?;
    let term = match LevelSpec::new(show_str) {
        Ok(ls) => {
            let show = ls.show();
            if show == &LevelType::Relative {
                std::env::var("DD_SHOW")?
            } else {
                show.to_str().to_owned()
            }
        }
        // we assume that a path was passed in as opposed to a levelspec
        Err(_) => show_str.to_string(),
    };
    let search = vec![term];
    // todo handle abs path
    let mut validpath = cli::validpath_from_terms(search, &graph, false, false)?;
    let idx = keymap.get("show").unwrap();
    log::trace!("got index {:?}", idx);
    validpath.remove_past(idx)?;

    let mut pathbuf = validpath.pathbuf();
    pathbuf.push("etc");
    pathbuf.push("template.jspt");
    log::info!("Returning template {:?}", pathbuf);
    Ok(pathbuf)
}