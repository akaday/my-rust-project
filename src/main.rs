extern crate job_search;

fn main() {
    let jobs = vec![
        ("Software Engineer at Google", 120000, "Full-time", "Mid-level"),
        ("Data Scientist at Facebook", 130000, "Full-time", "Senior"),
        ("Backend Developer at Amazon", 110000, "Part-time", "Junior"),
    ];

    let min_salary = 115000;
    let max_salary = 125000;
    let filtered_by_salary = job_search::filter_jobs_by_salary_range(min_salary, max_salary, jobs.iter().map(|(title, salary, _, _)| (*title, *salary)).collect());

    let job_type = "Full-time";
    let filtered_by_job_type = job_search::filter_jobs_by_job_type(job_type, jobs.iter().map(|(title, _, jt, _)| (*title, *jt)).collect());

    let experience_level = "Senior";
    let filtered_by_experience = job_search::filter_jobs_by_experience_level(experience_level, jobs.iter().map(|(title, _, _, exp)| (*title, *exp)).collect());

    println!("Filtered Jobs by Salary Range ({} - {}): {:?}", min_salary, max_salary, filtered_by_salary);
    println!("Filtered Jobs by Job Type ({}): {:?}", job_type, filtered_by_job_type);
    println!("Filtered Jobs by Experience Level ({}): {:?}", experience_level, filtered_by_experience);
}
