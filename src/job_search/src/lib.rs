
pub fn search_jobs(keyword: &str, jobs: Vec<&str>) -> Vec<&str> {
    jobs.into_iter()
        .filter(|job| job.contains(keyword))
        .collect()
}

pub fn filter_jobs_by_location(location: &str, jobs: Vec<(&str, &str)>) -> Vec<(&str, &str)> {
    jobs.into_iter()
        .filter(|(_, loc)| loc.contains(location))
        .collect()
}

pub fn filter_jobs_by_company(company: &str, jobs: Vec<(&str, &str, &str)>) -> Vec<(&str, &str, &str)> {
    jobs.into_iter()
        .filter(|(_, _, comp)| comp.contains(company))
        .collect()
}
