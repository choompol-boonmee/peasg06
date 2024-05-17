use shapefile::{Point, PolygonRing};
use std::collections::HashMap;

fn utm_latlong(x: f32, y: f32) -> (f32, f32) {
    let e5 = x;
    let f5 = y;
    let c12 = 6378137.0_f32;
    let c13 = 6356752.31424518_f32;
    let _c15 = (c12 * c12 - c13 * c13).sqrt() / c12;
    let c16 = (c12 * c12 - c13 * c13).sqrt() / c13;
    let c17 = c16 * c16;
    let c18 = c12.powf(2.0) / c13;
    //System.out.println("C17: "+C17+" C18:"+C18);
    let c21 = 47.0;
    let c22 = 'N';
    let o5 = if c22 == 'S' { f5 - 10000000.0 } else { f5 };
    let k5 = o5 / (6366197.724 * 0.9996);
    // $C$17*(COS($K$5))^2
    let l7 = c17 * k5.cos().powf(2.0);
    let l8 = (1.0 + l7).powf(0.5);
    //=(1+L7)^(1/2)
    let l9 = c18 / l8 * 0.9996;
    //System.out.println("L7:"+L7+" L8:"+L8+" L9:"+L9);
    let l5 = l9;
    let p5 = (e5 - 500000.0) / l5;
    let aa5 = ((c17 * p5.powf(2.0)) / 2.0) * k5.cos().powf(2.0);
    let ab5 = p5 * (1.0 - (aa5 / 3.0));
    let ad5 = (ab5.exp() - (-ab5).exp()) / 2.0;
    let q5 = (2.0 * k5).sin();
    let r5 = q5 * k5.cos().powf(2.0);
    let s5 = k5 + (q5 / 2.0);
    let t5 = (3.0 * s5 + r5) / 4.0;
    //System.out.println("Q5: "+ Q5+" R5:"+R5+" S5:"+S5+" T5:"+T5);
    let u5 = (5.0 * t5 + r5 * k5.cos().powf(2.0)) / 3.0;
    let v5 = (0.75) * c17;
    let w5 = (5.0 / 3.0) * v5.powf(2.0);
    let x5 = (35.0 / 27.0) * v5.powf(3.0);
    //System.out.println("U5: "+ U5+" V5:"+V5+" W5:"+W5+" X5:"+X5);
    let y5 = 0.9996 * c18 * (k5 - (v5 * s5) + (w5 * t5) - (x5 * u5));
    let z5 = (o5 - y5) / l5;
    let ac5 = z5 * (1.0 - aa5) + k5;
    let ae5 = (ad5 / ac5.cos()).atan();
    //System.out.println("AA5:"+ AA5+" AB5:"+AB5+" AD5:"+AD5+" AC5:"+AC5+" AE5:"+AE5);
    let af5 = (ae5.cos() * ac5.tan()).atan();
    let m5 = k5
        + (1.0 + c17 * k5.cos().powf(2.0) - (3.0 / 2.0) * c17 * k5.sin() * k5.cos() * (af5 - k5))
            * (af5 - k5);
    let n5 = 6.0 * c21 - 183.0;
    let ag5 = m5 / std::f32::consts::PI * 180.0;
    let ah5 = ae5 / std::f32::consts::PI * 180.0 + n5;
    (ag5, ah5)
}

pub async fn read() {
    print!("READ\n");
    let lys = [
        "N1", "N2", "N3", "C1", "C2", "C3", "NE1", "NE2", "NE3", "S1", "S2", "S3",
    ];
    for r in lys {
        let mut sbgismp = HashMap::new();
        let f = format!("../sgdata/ShpMV/Shp{}/DS_T_Station.shp", r);
        print!("LY: {}\n", f);
        if let Ok(mut reader) = shapefile::Reader::from_path(&f) {
            print!(" OK\n");
            let (mut abbr, mut thnm, mut name) = ("".to_string(), "".to_string(), "".to_string());
            let (mut sub, mut own, mut btp) = ("".to_string(), "".to_string(), "".to_string());
            for result in reader.iter_shapes_and_records_as::<shapefile::Point, dbase::Record>() {
                if let Ok((pnt, rc)) = result {
                    for (nm, va) in rc {
                        /*
                        if let dbase::FieldValue::Character(Some(s)) = &va {
                            print!("{} = {}\n", nm, va);
                        }
                        */
                        if nm == "ABBRNAME" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                abbr = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "NAME_THAI" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                thnm = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "STATIONNAM" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                name = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "SUBSTATION" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                sub = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "OWNER" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                own = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        } else if nm == "BUSTYPE" {
                            if let dbase::FieldValue::Character(Some(s)) = &va {
                                btp = s.to_string();
                                //print!("   {} - {}\n", nm, s);
                            }
                        }
                    }
                    let (x, y) = utm_latlong(pnt.x as f32, pnt.y as f32);
                    print!(
                        "({},{}) {},{},{},{},{},{}\n",
                        x, y, abbr, thnm, name, sub, own, btp
                    );
                    sbgismp.insert(
                        abbr.to_string(),
                        (
                            x,
                            y,
                            thnm.to_string(),
                            name.to_string(),
                            sub.to_string(),
                            own.to_string(),
                            btp.to_string(),
                        ),
                    );
                }
            }
        }
        if let Ok(se) = bincode::serialize(&sbgismp) {
            std::fs::write(crate::sg::ldp::res("sbgismp.bin"), se).unwrap();
        }
    }
}
