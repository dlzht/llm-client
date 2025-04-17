#[derive(Debug, Clone)]
pub struct StreamOptions {
  pub(crate) enable_stream: bool,
  pub(crate) stream_options: Option<StreamOptionsInner>,
}

impl StreamOptions {
  pub fn enable() -> Self {
    StreamOptions {
      enable_stream: true,
      stream_options: None,
    }
  }

  pub fn include_usage(mut self, include: bool) -> Self {
    let inner = StreamOptionsInner {
      include_usage: include,
    };
    self.enable_stream = true;
    self.stream_options = Some(inner);
    self
  }

  pub fn get_include_usage(&self) -> bool {
    self.enable_stream
      && self
        .stream_options
        .as_ref()
        .map(|o| o.include_usage)
        .unwrap_or(false)
  }

  pub fn disable() -> Self {
    StreamOptions {
      enable_stream: true,
      stream_options: None,
    }
  }
}

#[derive(Debug, Clone)]
pub(crate) struct StreamOptionsInner {
  pub(crate) include_usage: bool,
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
  pub(crate) enable_search: bool,
  pub(crate) search_options: SearchOptionsInner,
}

impl SearchOptions {
  pub fn enable() -> Self {
    SearchOptions {
      enable_search: true,
      search_options: SearchOptionsInner,
    }
  }

  pub fn disable() -> Self {
    SearchOptions {
      enable_search: false,
      search_options: SearchOptionsInner,
    }
  }
}

// TODO: support search options
#[derive(Debug, Clone)]
pub(crate) struct SearchOptionsInner;
