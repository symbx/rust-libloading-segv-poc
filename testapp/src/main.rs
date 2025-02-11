use libloading::{Library, Symbol};
use plg::PluginTrait;

type CreatePluginFn = unsafe fn() -> *mut dyn PluginTrait;

fn main() {
    unsafe {
        let lib = Library::new("./libplg_base.so").expect("Failed to load plugin");

        let create_plugin: Symbol<CreatePluginFn> = lib.get(b"create_plugin").expect("Failed to find function");
        let plugin_ptr = create_plugin();

        if plugin_ptr.is_null() {
            panic!("Plugin returned a null pointer");
        }

        let plugin = &*plugin_ptr;
        plugin.run();
    }
}
