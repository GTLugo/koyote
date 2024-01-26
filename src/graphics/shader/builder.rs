use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::path::PathBuf;
use crate::graphics::shader::lang::Source;
use crate::graphics::shader::ShaderSource;
use crate::graphics::shader::stage::Stage;

#[derive(Default, Clone)]
pub struct ShaderStagesMissing;

#[derive(Default, Clone)]
pub struct ShaderStagesSpecified;

pub struct ShaderBuilder<S> {
  stage_specified: PhantomData<S>,
  path: PathBuf,
  stages: HashSet<Stage>,
}

impl ShaderBuilder<ShaderStagesMissing> {
  pub fn new(path: PathBuf) -> Self {
    Self {
      stage_specified: PhantomData,
      path,
      stages: Default::default(),
    }
  }
}

#[allow(unused)]
impl<S> ShaderBuilder<S> {
  pub fn with_stage(self, stage: Stage) -> ShaderBuilder<ShaderStagesSpecified> {
    let mut stages = self.stages.clone();
    stages.insert(stage);

    ShaderBuilder {
      stage_specified: PhantomData,
      path: self.path,
      stages,
    }
  }

  pub fn with_stages(self, stages: &[Stage]) -> ShaderBuilder<ShaderStagesSpecified> {
    let mut stages_map = self.stages.clone();
    stages_map.extend(stages);

    ShaderBuilder {
      stage_specified: PhantomData,
      path: self.path,
      stages: stages_map,
    }
  }
}

impl ShaderBuilder<ShaderStagesSpecified> {
  pub fn build_hlsl(self) -> Result<ShaderSource> {
    let mut stages = HashMap::new();
    for stage in self.stages {
      let source = Source::read_hlsl(stage, &self.path)?;
      stages.insert(stage, source);
    }

    Ok(ShaderSource {
      path: self.path,
      stages,
    })
  }
}
