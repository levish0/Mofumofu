pub mod request;
pub mod response;

pub use request::{CreateReportRequest, GetReportsRequest, ReportIdPath, UpdateReportRequest};
pub use response::{ReportListResponse, ReportResponse};
