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

pub fn filter_jobs_by_salary_range(min_salary: u32, max_salary: u32, jobs: Vec<(&str, u32)>) -> Vec<(&str, u32)> {
    jobs.into_iter()
        .filter(|(_, salary)| *salary >= min_salary && *salary <= max_salary)
        .collect()
}

pub fn filter_jobs_by_job_type(job_type: &str, jobs: Vec<(&str, &str)>) -> Vec<(&str, &str)> {
    jobs.into_iter()
        .filter(|(_, jt)| jt.contains(job_type))
        .collect()
}

pub fn filter_jobs_by_experience_level(experience_level: &str, jobs: Vec<(&str, &str)>) -> Vec<(&str, &str)> {
    jobs.into_iter()
        .filter(|(_, exp)| exp.contains(experience_level))
        .collect()
}
