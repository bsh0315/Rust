fn main(){
        let my_city = "Seoul";
        let year = 2002;
        let population = 9_987_987;
        println!(
            "The city of {city} in {year} had a population of {population}.",
            city = my_city,
            year = year,
            population = population
        );
        println!( "The city of {} in {} had a population of {}. I love {}!",my_city, year, population, my_city);
        println!( "The city of {0} in {1} had a population of {2}. I love {0}!",my_city, year, population);
}
            
