use actix_web::{web, App, HttpServer, Responder};
use job_search::{search_jobs, filter_jobs_by_salary_range, filter_jobs_by_job_type, filter_jobs_by_experience_level};

async fn search_jobs_handler(keyword: web::Path<String>, jobs: web::Json<Vec<String>>) -> impl Responder {
    let keyword = keyword.into_inner();
    let jobs = jobs.into_inner();
    let filtered_jobs = search_jobs(&keyword, jobs.iter().map(AsRef::as_ref).collect());
    web::Json(filtered_jobs)
}

async fn filter_jobs_by_salary_handler(params: web::Path<(u32, u32)>, jobs: web::Json<Vec<(String, u32)>>) -> impl Responder {
    let (min_salary, max_salary) = params.into_inner();
    let jobs = jobs.into_inner();
    let filtered_jobs = filter_jobs_by_salary_range(min_salary, max_salary, jobs.iter().map(|(title, salary)| (title.as_str(), *salary)).collect());
    web::Json(filtered_jobs)
}

async fn filter_jobs_by_job_type_handler(job_type: web::Path<String>, jobs: web::Json<Vec<(String, String)>>) -> impl Responder {
    let job_type = job_type.into_inner();
    let jobs = jobs.into_inner();
    let filtered_jobs = filter_jobs_by_job_type(&job_type, jobs.iter().map(|(title, jt)| (title.as_str(), jt.as_str())).collect());
    web::Json(filtered_jobs)
}

async fn filter_jobs_by_experience_handler(experience_level: web::Path<String>, jobs: web::Json<Vec<(String, String)>>) -> impl Responder {
    let experience_level = experience_level.into_inner();
    let jobs = jobs.into_inner();
    let filtered_jobs = filter_jobs_by_experience_level(&experience_level, jobs.iter().map(|(title, exp)| (title.as_str(), exp.as_str())).collect());
    web::Json(filtered_jobs)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/search_jobs/{keyword}", web::post().to(search_jobs_handler))
            .route("/filter_jobs_by_salary/{min_salary}/{max_salary}", web::post().to(filter_jobs_by_salary_handler))
            .route("/filter_jobs_by_job_type/{job_type}", web::post().to(filter_jobs_by_job_type_handler))
            .route("/filter_jobs_by_experience/{experience_level}", web::post().to(filter_jobs_by_experience_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
