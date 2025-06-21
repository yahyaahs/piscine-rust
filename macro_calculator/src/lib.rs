use json::JsonValue;
pub struct Food {
    pub name : String,
    pub calories : (String, String),
    pub fats : f64,
    pub carbs : f64,
    pub proteins : f64,
    pub nbr_of_portions : f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut json = JsonValue::new_object();
    let mut calories = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
    for i in foods.iter(){
        calories+= match i.calories.1.replace("Kcal", "").parse::<f64>() {
            Ok(nbr) => nbr,
            Err(_)=> 0.0,
            
        };
        carbs+= i.carbs *i.nbr_of_portions;
        proteins+= i.proteins*i.nbr_of_portions;
        fats+= i.fats*i.nbr_of_portions;
    }
    json["cals"]= ((calories*100.0).round()/100.0).into();
    json["carbs"]= ((carbs*100.0).round()/100.0).into();
    json["proteins"]= ((proteins*100.0).round()/100.0).into();
    json["fats"]= ((fats* 100.0).round()/100.0).into();
    return json;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
   let foods = [
        Food {
            name: "big mac".to_owned(),
            calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
            proteins: 27.,
            fats: 26.,
            carbs: 41.,
            nbr_of_portions: 2.,
        },
        Food {
            name: "pizza margherita".to_owned(),
            calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    println!("{:#}", calculate_macros(&foods));
    }
}
