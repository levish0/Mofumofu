use mofumofu_dto::reports::{
    CreateReportRequest, GetReportsRequest, ReportIdPath, ReportListResponse, ReportResponse,
    UpdateReportRequest,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_report::create_report,
        super::get_reports::get_reports,
        super::update_report::update_report,
    ),
    components(
        schemas(
            ReportResponse,
            ReportListResponse,
            CreateReportRequest,
            GetReportsRequest,
            UpdateReportRequest,
            ReportIdPath,
        )
    ),
    tags(
        (name = "Reports", description = "Report endpoints")
    )
)]
pub struct ReportsApiDoc;
