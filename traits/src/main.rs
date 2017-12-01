fn main() {
    #[derive(Debug)]
    struct DateBased{
        series: String,
        date: i32
    }
    #[derive(Debug)]
    struct SeasonBased {
        series: String,
        season: i32,
        episode: i32
    }

    enum ParsedFile{
        Date(DateBased),
        Season(SeasonBased),
    }

    fn populate_datebased(file: DateBased){
        println!("{:?}", file);
    };

    fn populate_seasonbased(file: SeasonBased){
        println!("{:?}", file);
    };

    fn populate(parsed_file: ParsedFile){
        match parsed_file {
            ParsedFile::Date(data) => populate_datebased(data),
            ParsedFile::Season(data) => populate_seasonbased(data),

        }
    }

    let data_based = DateBased {
        series: "foo".to_string(),
        date: 1,
    };

    let seasoned_based = SeasonBased {
        series: "2".to_string(),
        season: 2,
        episode: 2,
    };

    let parsed_file = ParsedFile::Date(data_based);
    let parsed_file = ParsedFile::Season(seasoned_based);

    populate(parsed_file);
}
