pub fn init() {
    println! ("Common cache Init::Success");
}
//Parser *This thing is Important
//For the Poor Souls 
//scheme: http or https , host : Domain or IP(Usually);
//port: Wheretf Connection is Living;
//path: Usually root (For Linux Bros),can be absolute or relative;
//Path can be absolute/relative,can be changed upon Interaction;
//query: Usually starts with ?(Shit makes DB goes Brr);
//fragment: Generally '#',Usually a Section of same text;
pub struct Url {
    pub scheme: String,
    pub host:   String,
    pub port:Option<u16>,
    pub path:String,
    pub query:Option<String>,
    pub fragment:Option<String>,
}
//Scheme Parsing
pub fn  scheme(input: &str) -> Result <String,String> {
if input.starts_with("https://"){
    Ok(String::from("https"))
}else if input.starts_with("http://") {
    Ok(String::from("http"))    
}else {
    Err(String::from(input))
}
}
//Verify the HTTPS returns the same expected results (I Dont have any Idea BTW).
