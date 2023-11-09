use std::path::PathBuf;
use serde_json::json;

fn main() {
    use machlears::prelude::*;
    // CSV of housing data. Let's predict price by fireplaces
    // Linear Regression
    let mach = machlears::MachScaffold::setup(setup::Opts {
        
            database: DatabaseOpt::None,
            // datasource: DataSource::CsvFile(PathBuf::from("C:/Users/Noah3/OneDrive/Documents/Research/ADHD/Singh2008__Beyond_polemics.pdf")),
            // datasource: DataSource::JsonData(json!(
            //     {
            //         "data": [
            //             {"price": 6000, "fireplaces": 2}, {"price": 6000, "fireplaces": 4}
            //         ]
            //     }
            // ))
            datasource: DataSource::CsvFile(PathBuf::from("/foo")),
            print_style: PrintStyle::silent(),

        })
        .train(train::Opts {

        })
        .build(build::Opts {

        });


}
