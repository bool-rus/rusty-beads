extern crate quick_csv;
use crate::grid::Grid;
use crate::entities::Color;
use std::fs::File;
use std::io::Write;
use self::quick_csv::Csv;
use std::str::FromStr;
use std::num::NonZeroUsize;

pub fn write(file: &str, content: &Grid<Color>) -> std::io::Result<()> {

    let mut file = File::create(file)?;
    for row in content.as_table().into_iter() {
        let mut first = true;
        for item in row.iter() {
            if first {
                first = false;
            } else {
                write!(&mut file, ",")?;
            }
            write!(&mut file, "{:X}", item)?;
        }
        write!(&mut file, "\n")?;
    }

    Ok(())
}


pub fn read(file: &str) -> Result<Grid<Color>,quick_csv::error::Error> {
    let mut data = Vec::with_capacity(10000usize);
    let csv = Csv::from_file(file)?;
    let mut first = true;
    let mut width = 0usize;
    let mut counter = 0usize;
    for row in csv.into_iter() {
        let row = row?;
        if first {
            first = false;
            width = row.len();
        }
        counter += 1;
        row.columns()?.for_each(|item| {
            data.push(Color::from_str(item).unwrap())
        })
    }
    Ok(Grid::frow_raw(
        NonZeroUsize::new(width).unwrap(),
        NonZeroUsize::new(counter).unwrap(),
        data
    ).unwrap())
}