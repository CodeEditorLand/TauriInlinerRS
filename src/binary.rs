use kuchiki::NodeRef;

use std::path::PathBuf;

pub fn inline_base64(
  config: &super::Config,
  root_path: &PathBuf,
  document: &NodeRef,
) -> crate::Result<()> {
  for target in document
    .select("video, img, link[rel=icon], link[rel=shortcut]")
    .unwrap()
  {
    let node = target.as_node();
    let element = node.as_element().unwrap();
    let attr = match element.name.local.to_string().as_str() {
      "video" | "img" => "src",
      "link" => "href",
      _ => panic!("tag not implemented"),
    };
    let mut attributes = element.attributes.borrow_mut();
    if let Some(source) = attributes.get(attr) {
      if let Some(resolve_source) = crate::get(source, &config, &root_path)? {
        attributes.insert("src", resolve_source);
      }
    }
  }

  Ok(())
}
