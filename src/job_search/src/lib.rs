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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_jobs() {
        let jobs = vec!["Software Engineer", "Data Scientist", "Backend Developer"];
        let keyword = "Engineer";
        let result = search_jobs(keyword, jobs.iter().map(AsRef::as_ref).collect());
        assert_eq!(result, vec!["Software Engineer"]);
    }

    #[test]
    fn test_filter_jobs_by_location() {
        let jobs = vec![
            ("Software Engineer", "New York"),
            ("Data Scientist", "San Francisco"),
            ("Backend Developer", "New York"),
        ];
        let location = "New York";
        let result = filter_jobs_by_location(location, jobs.iter().map(|(title, loc)| (*title, *loc)).collect());
        assert_eq!(result, vec![
            ("Software Engineer", "New York"),
            ("Backend Developer", "New York"),
        ]);
    }

    #[test]
    fn test_filter_jobs_by_company() {
        let jobs = vec![
            ("Software Engineer", "New York", "Google"),
            ("Data Scientist", "San Francisco", "Facebook"),
            ("Backend Developer", "New York", "Google"),
        ];
        let company = "Google";
        let result = filter_jobs_by_company(company, jobs.iter().map(|(title, loc, comp)| (*title, *loc, *comp)).collect());
        assert_eq!(result, vec![
            ("Software Engineer", "New York", "Google"),
            ("Backend Developer", "New York", "Google"),
        ]);
    }

    #[test]
    fn test_filter_jobs_by_salary_range() {
        let jobs = vec![
            ("Software Engineer", 120000),
            ("Data Scientist", 130000),
            ("Backend Developer", 110000),
        ];
        let min_salary = 115000;
        let max_salary = 125000;
        let result = filter_jobs_by_salary_range(min_salary, max_salary, jobs.iter().map(|(title, salary)| (*title, *salary)).collect());
        assert_eq!(result, vec![
            ("Software Engineer", 120000),
        ]);
    }

    #[test]
    fn test_filter_jobs_by_job_type() {
        let jobs = vec![
            ("Software Engineer", "Full-time"),
            ("Data Scientist", "Full-time"),
            ("Backend Developer", "Part-time"),
        ];
        let job_type = "Full-time";
        let result = filter_jobs_by_job_type(job_type, jobs.iter().map(|(title, jt)| (*title, *jt)).collect());
        assert_eq!(result, vec![
            ("Software Engineer", "Full-time"),
            ("Data Scientist", "Full-time"),
        ]);
    }

    #[test]
    fn test_filter_jobs_by_experience_level() {
        let jobs = vec![
            ("Software Engineer", "Mid-level"),
            ("Data Scientist", "Senior"),
            ("Backend Developer", "Junior"),
        ];
        let experience_level = "Senior";
        let result = filter_jobs_by_experience_level(experience_level, jobs.iter().map(|(title, exp)| (*title, *exp)).collect());
        assert_eq!(result, vec![
            ("Data Scientist", "Senior"),
        ]);
    }
}
