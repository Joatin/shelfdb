

pub fn is_collection_file(file_name: &str) -> bool {
    file_name.contains('_') && file_name.contains(".gz") && !file_name.contains("~old")
}
