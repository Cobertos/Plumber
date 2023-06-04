#![warn(clippy::all, clippy::pedantic, clippy::multiple_crate_versions)]
// these are triggered by pyo3
#![allow(clippy::used_underscore_binding, clippy::borrow_deref_ref)]
// this doesn't matter
#![allow(clippy::cast_precision_loss)]
// this is annoying
#![allow(clippy::module_name_repetitions)]

mod asset;
mod filesystem;
mod importer;

use std::io::Write;

use asset::{entities::PyUnknownEntity, model::PyBoneRestData};
use filesystem::{PyFileBrowser, PyFileBrowserEntry, PyFileSystem};

use log::{error, info, Level, LevelFilter};
use plumber_core::asset_core::current_thread;
use pyo3::prelude::*;

use crate::{
    asset::{
        brush::{PyBuiltBrushEntity, PyBuiltSolid, PyMergedSolids},
        entities::{PyEnvLight, PyLight, PyLoadedProp, PySkyCamera, PySpotLight},
        material::{
            BuiltMaterialData, BuiltNode, BuiltNodeSocketRef, Material, Texture, TextureRef,
        },
        model::{
            PyBoneAnimationData, PyLoadedAnimation, PyLoadedBone, PyLoadedMesh, PyModel,
            QuaternionData, VectorData,
        },
        overlay::PyBuiltOverlay,
        sky::PySkyEqui,
    },
    importer::PyImporter,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[pymodule]
fn plumber(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyFileSystem>()?;
    m.add_class::<PyFileBrowser>()?;
    m.add_class::<PyFileBrowserEntry>()?;
    m.add_class::<PySkyEqui>()?;
    m.add_class::<Texture>()?;
    m.add_class::<Material>()?;
    m.add_class::<BuiltMaterialData>()?;
    m.add_class::<BuiltNode>()?;
    m.add_class::<BuiltNodeSocketRef>()?;
    m.add_class::<TextureRef>()?;
    m.add_class::<QuaternionData>()?;
    m.add_class::<VectorData>()?;
    m.add_class::<PyBoneAnimationData>()?;
    m.add_class::<PyBoneRestData>()?;
    m.add_class::<PyLoadedAnimation>()?;
    m.add_class::<PyLoadedBone>()?;
    m.add_class::<PyLoadedMesh>()?;
    m.add_class::<PyModel>()?;
    m.add_class::<PyMergedSolids>()?;
    m.add_class::<PyBuiltSolid>()?;
    m.add_class::<PyBuiltBrushEntity>()?;
    m.add_class::<PyBuiltOverlay>()?;
    m.add_class::<PyLoadedProp>()?;
    m.add_class::<PyLight>()?;
    m.add_class::<PySpotLight>()?;
    m.add_class::<PyEnvLight>()?;
    m.add_class::<PySkyCamera>()?;
    m.add_class::<PyUnknownEntity>()?;
    m.add_class::<PyImporter>()?;

    #[pyfn(m)]
    fn discover_filesystems() -> Vec<PyFileSystem> {
        filesystem::discover()
    }

    #[pyfn(m)]
    fn filesystem_from_gameinfo(path: &str) -> PyResult<PyFileSystem> {
        filesystem::from_gameinfo(path)
    }

    #[pyfn(m)]
    fn log_error(error: &str) {
        error!("{}", error);
    }

    #[pyfn(m)]
    fn log_info(info: &str) {
        info!("{}", info);
    }

    #[pyfn(m)]
    fn version() -> &'static str {
        VERSION
    }

    initialize_logger();

    Ok(())
}

fn initialize_logger() {
    let _ = env_logger::Builder::new()
        .format(|buf, record| {
            if record.level() == Level::Debug {
                writeln!(
                    buf,
                    "[Plumber] [{}] (thread {}) {}",
                    record.level(),
                    current_thread(),
                    record.args()
                )
            } else {
                writeln!(buf, "[Plumber] [{}] {}", record.level(), record.args())
            }
        })
        .filter_level(LevelFilter::Debug)
        .try_init();
}
