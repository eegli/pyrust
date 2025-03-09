pub mod pi;

pub fn num_cpus_available() -> usize {
    num_cpus::get()
}
