pub struct MonthItem<'n> {
    number: i16,
    name: &'n str,
}

pub struct Month<'m> {
    month: i16,
    months: Vec<MonthItem<'m>>,
}

impl Month<'_> {
    pub fn new<'mo>(current_month: i16) -> Month<'mo> {
        Month {
            month: current_month,
            months: vec![
                MonthItem { number: 1, name: "January" },
                MonthItem { number: 2, name: "February" },
                MonthItem { number: 3, name: "March" },
                MonthItem { number: 4, name: "April" },
                MonthItem { number: 5, name: "May" },
                MonthItem { number: 6, name: "June" },
                MonthItem { number: 7, name: "July" },
                MonthItem { number: 8, name: "August" },
                MonthItem { number: 9, name: "September" },
                MonthItem { number: 10, name: "October" },
                MonthItem { number: 11, name: "November" },
                MonthItem { number: 12, name: "December" },
            ],
        }
    }

    pub fn name<'a>(&mut self) -> Result<&str, String> {
        for month in &self.months {
            if self.month == month.number.clone() {
                return Ok(month.name);
            }
        }

        Err("Invalid month".to_string())
    }
}