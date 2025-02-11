// Make sure it's compiled as an `rlib`, so both the host and plugin link to the same definition.
pub trait PluginTrait {
    fn run(&self);
}
