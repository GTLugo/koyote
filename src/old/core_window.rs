use windows::ApplicationModel::Core::{CoreApplicationView, IFrameworkView, IFrameworkView_Impl, IFrameworkViewSource, IFrameworkViewSource_Impl};
use windows::core::{HSTRING, implement};
use windows::UI::Core::CoreWindow;

#[implement(IFrameworkViewSource)]
pub struct CoreApp {}

impl IFrameworkViewSource_Impl for CoreApp {
  fn CreateView(&self) -> windows::core::Result<IFrameworkView> {
    Ok(CoreAppView().into())
  }
}

#[implement(IFrameworkView)]
struct CoreAppView();

impl IFrameworkView_Impl for CoreAppView {
  fn Initialize(&self, applicationview: Option<&CoreApplicationView>) -> windows::core::Result<()> {
    Ok(())
  }

  fn SetWindow(&self, window: Option<&CoreWindow>) -> windows::core::Result<()> {
    Ok(())
  }

  fn Load(&self, entrypoint: &HSTRING) -> windows::core::Result<()> {
    Ok(())
  }

  fn Run(&self) -> windows::core::Result<()> {
    Ok(())
  }

  fn Uninitialize(&self) -> windows::core::Result<()> {
    Ok(())
  }
}