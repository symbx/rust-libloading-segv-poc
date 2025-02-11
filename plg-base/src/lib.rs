use plg::PluginTrait;

struct PluginImpl;

impl PluginTrait for PluginImpl {
    fn run(&self) {
        println!("Plugin is running!");
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn PluginTrait {
    let plugin = Box::new(PluginImpl);
    Box::into_raw(plugin) // Here is SEGV
}
