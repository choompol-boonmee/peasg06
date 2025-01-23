mod sg;
mod web;
mod ws;

use std::env;

#[tokio::main]
async fn main() {
    let ath = env::args().nth(1).unwrap_or("P2".to_string());
    match ath.as_str() {
        "P1" => crate::sg::init1::run().await,
        "P2" => crate::sg::init2::run().await,
        "P3" => crate::sg::init3::run().await,
        "P4" => crate::sg::tranx::run().await,

        "I1" => crate::sg::mvline::read().await,
        "I2" => crate::sg::mvline::read_trans_lv().await,
        "I3" => crate::sg::mvline::read_lv_line().await,
        "I4" => crate::sg::mvline::excel().await.unwrap(),

        "G1" => crate::sg::gis1::read_shp().await,
        "G2" => crate::sg::gis1::read_prov().await,
        "G3" => crate::sg::gis1::read_aoj().await,

        "I5" => crate::sg::mvline::pea_sub_excel().await.expect("ERROR"),
        "I6" => crate::sg::mvline::pea_sub_read().await.expect("ERROR"),

        "I01" => crate::sg::imp::pea_spp_lp_read().await.expect("ERROR"),
        "I02" => crate::sg::imp::pea_vspp_lp_read().await.expect("ERROR"),

        "I03" => crate::sg::imp::pea_sub_xml_read().await.expect("ERROR"),
        "I04" => crate::sg::imp::pea_sub_excel().await.expect("ERROR"),

        "I05" => crate::sg::imp::pea_vspp_excel().await.expect("ERROR"),
        "I06" => crate::sg::imp::pea_lv_solar_xlsx().await.expect("ERROR"),

        "I07" => crate::sg::imp::pea_der_xlsx().await.expect("ERROR"),

        "I08" => crate::sg::imp::pea_bizme().await.expect("ERROR"),

        "I09" => crate::sg::imp::pea_sub_plan().await.expect("ERROR"),
        "I10" => crate::sg::imp::pea_load_fore().await.expect("ERROR"),

        "I11" => crate::sg::imp::pea_evcs().await.expect("ERROR"),
        "I12" => crate::sg::imp::pea_bess_plan().await.expect("ERROR"),
        "I13" => crate::sg::imp::pea_meter_read().await.expect("ERROR"),
        "I14" => crate::sg::imp::pea_re_plan().await.expect("ERROR"),

        "R03" => crate::sg::imp::pea_sub_read().await.expect("ERROR"),
        "R04" => crate::sg::imp::pea_sub_do().await.expect("ERROR"),
        "R05" => crate::sg::imp::pea_bess_ana().await.expect("ERROR"),

        "C01" => crate::sg::prc1::proc1().await.expect("?"),
        "C02" => crate::sg::prc1::proc2().await.expect("?"),
        "C03" => crate::sg::prc1::proc3().await.expect("?"),
        "C04" => crate::sg::prc1::proc4().await.expect("?"),
        "C05" => crate::sg::prc1::proc5().await.expect("?"), // meter
        "C06" => crate::sg::prc1::proc6().await.expect("?"), // outage
        "C07" => crate::sg::prc1::proc7().await.expect("?"),
        "C08" => crate::sg::prc1::proc8().await.expect("?"),

        "P21" => crate::sg::prc2::prc21().await.expect("?"),
        "P22" => crate::sg::prc2::prc22().await.expect("?"),
        "P23" => crate::sg::prc2::prc23().await.expect("?"),

        "P31" => crate::sg::prc3::prc31().await.expect("?"),
        "P32" => crate::sg::prc3::prc32().await.expect("?"),
        "P33" => crate::sg::prc3::prc33().await.expect("?"), //
        "P34" => crate::sg::prc3::prc34().await.expect("?"), //
        "P35" => crate::sg::prc3::prc35().await.expect("?"), //
        "P37" => crate::sg::prc3::prc37().await.expect("?"), //
        "P38" => crate::sg::prc3::prc38().await.expect("?"), //
        "P39" => crate::sg::prc3::prc39().await.expect("?"), //

        "P41" => crate::sg::prc4::prc41().await.expect("?"), //
        "P42" => crate::sg::prc4::prc42().await.expect("?"), //
        "P43" => crate::sg::prc4::prc43().await.expect("?"), //
        "P44" => crate::sg::prc4::prc44().await.expect("?"), //
        "P45" => crate::sg::prc4::prc45().await.expect("?"), //
        "P46" => crate::sg::prc4::prc46().await.expect("?"), //
        "P47" => crate::sg::prc4::prc47().await.expect("?"), //

        "P51" => crate::sg::prc5::prc51().await.expect("?"), //
        "P52" => crate::sg::prc5::prc52().await.expect("?"), //
        "P53" => crate::sg::prc5::prc53().await.expect("?"), //
        "P54" => crate::sg::prc5::prc54().await.expect("?"), //

        "P61" => crate::sg::prc6::prc61().await.expect("?"), //
        "P62" => crate::sg::prc6::prc62().await.expect("?"), //
        "P63" => crate::sg::prc6::prc63().await.expect("?"), //
        "P64" => crate::sg::prc6::prc64().await.expect("?"), //
        "P65" => crate::sg::prc6::prc65().await.expect("?"), //
        "P66" => crate::sg::prc6::prc66().await.expect("?"), //
        "P67" => crate::sg::prc6::prc67().await.expect("?"), //
        "P68" => crate::sg::prc6::prc68().await.expect("?"), //
        "P69" => crate::sg::prc6::prc69().await.expect("?"), //
        //
        "P81" => crate::sg::prc8::prc81().await.expect("?"), //
        "P82" => crate::sg::prc8::prc82().await.expect("?"), //

        _ => println!("NG"),
    }
}
