pub mod browser {
  pub use web_sys::{Window, Document, HtmlElement};
  
  pub struct Browser {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement
  }

  impl Browser {
    pub fn new() -> Self {
      let window = web_sys::window().expect("no global `window` exists");
      let document = window.document().expect("should have a document on window");
      let body = document.body().expect("document should have a body");

      Self {
        window: window,
        document: document,
        body: body
      }
    }
  }
}
