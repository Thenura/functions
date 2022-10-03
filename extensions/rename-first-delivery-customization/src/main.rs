use serde::Serialize;

mod api;
use api::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Input = serde_json::from_reader(std::io::BufReader::new(std::io::stdin()))?;
    let mut out = std::io::stdout();
    let mut serializer = serde_json::Serializer::new(&mut out);
    function(input)?.serialize(&mut serializer)?;
    Ok(())
}

fn function(_input: Input) -> Result<FunctionResult, Box<dyn std::error::Error>> {
    let delivery_options = _input.delivery_options;

    let operations = delivery_options
        .iter()
        .filter_map(|delivery_option| {
            Some(Operation {
                rename:Some(RenameOperation {
                    delivery_option_id: delivery_option.id.clone(),
                    name: "Rename Function".to_string(),
                })
            })
        })
        .collect();

    Ok(FunctionResult { operations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_contains_no_operations() {
        let input = Input {
            delivery_options: vec![
                DeliveryOption {
                    id: "1".to_string(),
                    name: "Standard".to_string()
                },
                DeliveryOption {
                    id: "2".to_string(),
                    name: "Express".to_string()
                }
            ],
        };
        let operations = function(input).unwrap().operations;

        assert!(operations.is_empty());
    }
}
