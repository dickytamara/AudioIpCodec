


/// trait that's implements load and save to ini files
pub trait HelperFileSettings {
    fn load(&self, path: &str);
    fn save(&self, path: &str);
}
