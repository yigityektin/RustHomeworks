struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: i32) -> bool{
        item == self.value
    }
}

fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    collection
        .iter()
        .cloned()
        .filter(|item| condition.is_match(*item))
        .collect()
}

fn main() {
    let collection: Vec<i32> = vec![3, 5, 7, 11, 13];
    let condition = FilterCondition {value: 5};

    let result = custom_filter(&collection, &condition);
    println!("Result: {:?}", result);
}