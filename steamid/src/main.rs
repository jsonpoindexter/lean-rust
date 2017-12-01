fn main() {
    let accountid: u64 = 13655102;
    println!("accountid: {}", accountid);

    fn convert_accountid_to_steamid(accountid: u64) -> String {
        let y;
        let mut z = if (accountid % 2) == 0 {
            y = 0;
            accountid
        } else {
            y = 1;
            accountid
        };
        z = z + 7_960_265_728;

        println!("z: {}", z);

        format!("{}{}", "7656119", z)
    }

    let steamid = convert_accountid_to_steamid(accountid);
    println!("steamid: {}", steamid);

    fn convert_steamid_to_accountid(steamid: String) -> String {

        let steamid = steamid.replace("7656119", "");
        let mut steamid = steamid.parse::<u64>().unwrap();
        steamid = steamid - 7_960_265_728;

        steamid.to_string()


    }

    println!("{}", convert_steamid_to_accountid(steamid));
}
