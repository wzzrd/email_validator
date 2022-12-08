pub fn env_var(var_name: &str) -> String {
    match std::env::var(var_name) {
        Ok(v) => v,
        Err(e) => panic!("Environment variable {var_name} not set: {e}"),
    }
}

pub fn list_into_vector(my_list: String) -> Vec<String> {
    my_list.split(',').map(|s| s.trim().to_owned()).collect()
}
