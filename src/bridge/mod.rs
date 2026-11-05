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
    pub scheme: String, //Declares scheme as String (Usually http/https);
    pub host:   String, //Declares host as String (Usuall IP addr (ss -tuln IFYKYK));
    pub port:Option<u16>,//Declares Post [Ex: 192.69.6969":69"]*Everything after "" is port
    pub path:String,//Declares Path as string;*Usually http://192.69.6969:69/home/ztlfoundation*usually starts from Root
    pub query:Option<String>,//Declares Option as String;//Declares Query as a String
    pub fragment:Option<String>,//Declares fragment as String;//
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recognises_https(){
        assert_eq!(
            scheme("https://example.com"),
            Ok(String::from("https")));
    }
}