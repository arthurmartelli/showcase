use common::prelude::*;
use google_sheets4 as sheets4;
use google_sheets4::api::Sheet;
use hyper;
use hyper_rustls;
use sheets4::Sheets;

pub async fn get_data(
    auth: &Auth,
    spreadsheet_id: &str,
    range: &str,
    columns: usize,
) -> Result<DataSet<Receiver>, DynError> {
    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth.clone(),
    );

    let (_, sheet) = hub
        .spreadsheets()
        .get(spreadsheet_id)
        .add_ranges(range)
        .include_grid_data(true)
        .doit()
        .await?;

    let sheets = sheet.sheets.unwrap();
    let values = extract_values(sheets);
    let groups = data::subgroup_vector(values, columns);
    let result = create_receivers(groups);

    Ok(result)
}

fn create_receivers(mut data: Vec<Vec<String>>) -> DataSet<Receiver> {
    let mut receivers = DataSet::new();
    let labels: Vec<String> = data
        .remove(0)
        .iter()
        .map(|k| k.to_ascii_lowercase())
        .collect();

    for row in data {
        let mut info = DataMap::new();

        for (lab, r) in labels.iter().zip(row.iter()) {
            info.insert(lab.clone().into(), r.clone().into());
        }

        let name = info.get(&Data::from("name")).unwrap().clone();
        let email = info.get(&Data::from("email")).unwrap().clone();

        let email = EmailAddress::new(name, email);
        let receiver = Receiver::new(email, Some(info));

        receivers.insert(receiver);
    }

    receivers
}

fn extract_values(sheets: Vec<Sheet>) -> Vec<String> {
    let grids_data = {
        let mut temp = Vec::new();
        for sheet in sheets {
            temp.extend(sheet.data.unwrap());
        }
        temp
    };

    let rows_data = {
        let mut temp = Vec::new();
        for grid_data in grids_data {
            temp.extend(grid_data.row_data.unwrap());
        }
        temp
    };

    let cells_data = {
        let mut temp = Vec::new();
        for row_data in rows_data {
            temp.extend(row_data.values.unwrap());
        }
        temp
    };

    let values = {
        let mut temp = Vec::new();
        for cell_data in cells_data {
            temp.extend(cell_data.formatted_value);
        }
        temp
    };

    values
}
