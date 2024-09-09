use r2r;
{% if use_additonal_msgs -%}
use r2r::{{additonal_msgs}};
{% endif -%}
use futures::{StreamExt, future};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "testnode", "")?;

    println!("node name: {}", node.name()?);
    println!("node fully qualified name: {}", node.fully_qualified_name()?);

    node.params.lock().unwrap().iter().for_each(|(k,v)| {
        println!("{} - {:?}", k, v.value);
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
