pub mod pi;

pub fn num_cpus_available(physical_only: bool) -> usize {
    if physical_only {
        num_cpus::get_physical()
    } else {
        num_cpus::get()
    }
}
