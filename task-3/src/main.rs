extern crate reqwest;
extern crate select;
use std::time::Duration;
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

    use std::io::Read;

    fn main() {
        worldmeters("https://www.worldometers.info/coronavirus/#countries");        
    }
 
   fn worldmeters(url::&str){
       
        let mut resp = reqwest::get(url).unwrap();
        assert!(resp.status().is_success());

        let document = Document::from_read(resp).unwrap();

  
    for node in document.find(Class("even")) {
      
        let total_cases = node.find(Class("sorting_1")).next().unwrap();
        let total_deaths = node.find(Class("#main_table_countries_today > tbody:nth-child(2) > tr:nth-child(4) > td.sorting_1")).next().unwrap();
        let total_recovered = node.find(Class("#main_table_countries_today > tbody:nth-child(2) > tr:nth-child(4) > td:nth-child(7)")).next().unwrap();
   
       
       
        println!("\n | {} | {} | {} \n", total_cases.text(),total_deaths.text(),total_recovered.text());
        
}
for node in document.find(Class("odd")) {
      
    let total_cases = node.find(Class("sorting_1")).next().unwrap();
    let total_deaths = node.find(Class("#main_table_countries_today > tbody:nth-child(2) > tr:nth-child(7) > td:nth-child(5)").next().unwrap();
    let total_recovered = node.find(Class("#main_table_countries_today > tbody:nth-child(2) > tr:nth-child(7) > td:nth-child(7)").next().unwrap();
   
   
    println!("\n | {} | {} | {} \n", total_cases.text(),total_deaths.text(),total_recovered.text());
    
}


