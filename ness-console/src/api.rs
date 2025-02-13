use dropshot::PaginationOrder::{Ascending, Descending};
use dropshot::{
    HttpResponseOk, PaginationOrder, PaginationParams,
    Query, RequestContext, ResultsPage, WhichPage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::installs::{Install, Installs};
use crate::instances::{Instance, Instances};
use crate::error::ConsoleError;

trait HasIdentity {
    fn id(&self) -> &Uuid;
}

macro_rules! impl_HasIdentity {
    ($T:ident) => {
        impl HasIdentity for $T {
            fn id(&self) -> &Uuid {
                &self.id
            }
        }
    };
}

impl_HasIdentity!(Instance);
impl_HasIdentity!(Install);

#[derive(Deserialize, Clone, JsonSchema, Serialize)]
pub struct ExScanParams {
    #[serde(default = "default_sort_mode")]
    sort: ExSortMode,
}

fn default_sort_mode() -> ExSortMode {
    ExSortMode::ByIdAscending
}

#[derive(Deserialize, Clone, JsonSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExSortMode {
    ByIdAscending,
    ByIdDescending,
}

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExPageSelector {
    Id(PaginationOrder, Uuid),
}

fn page_selector<T: HasIdentity>(item: &T, scan_params: &ExScanParams) -> ExPageSelector {
    match scan_params {
        ExScanParams {
            sort: ExSortMode::ByIdAscending,
        } => ExPageSelector::Id(Ascending, *item.id()),
        ExScanParams {
            sort: ExSortMode::ByIdDescending,
        } => ExPageSelector::Id(Descending, *item.id()),
    }
}

fn scan_params(p: &WhichPage<ExScanParams, ExPageSelector>) -> ExScanParams {
    ExScanParams {
        sort: match p {
            WhichPage::First(ExScanParams { sort }) => sort.clone(),

            WhichPage::Next(ExPageSelector::Id(Ascending, ..)) => ExSortMode::ByIdAscending,
            WhichPage::Next(ExPageSelector::Id(Descending, ..)) => ExSortMode::ByIdDescending,
        },
    }
}

#[dropshot::api_description]
pub trait ConsoleApi {
    type Context: Instances + Installs;

    #[endpoint { method = GET, path = "/instances" }]
    async fn instance_list(
        rqctx: RequestContext<Self::Context>,
        query: Query<PaginationParams<ExScanParams, ExPageSelector>>,
    ) -> Result<HttpResponseOk<ResultsPage<Instance>>, ConsoleError> {
        let cx = rqctx.context();

        let params = query.into_inner();
        let limit = rqctx.page_limit(&params).unwrap().get() as usize;
        let scan_params = scan_params(&params.page);

        // dbg!(params, limit, scan_params);

        // let instances = cx.instance_list(params).await;

        let page = ResultsPage::new(vec![], &scan_params, page_selector).unwrap();

        Ok(HttpResponseOk(page))
    }
}

pub fn generate_openapi_spec() -> String {
    let api = console_api_mod::stub_api_description().unwrap();
    let spec = api.openapi("Counter Server", semver::Version::new(1, 0, 0));
    serde_json::to_string_pretty(&spec.json().unwrap()).unwrap()
}
