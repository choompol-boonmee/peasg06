
pub async fn read() {
    print!("READ\n");
    let lys = ["N1","N2","N3","C1","C2","C3","NE1","NE2","NE3","S1","S2","S3"];
    for r in lys {
        print!("LY: {}\n", r);
                 let f_shp = format!("{}/{:?}.shp", fo, r);
            let vol = r.eq_volt();
            if let Ok(mut reader) = shapefile::Reader::from_path(&f_shp) {

    }
}

