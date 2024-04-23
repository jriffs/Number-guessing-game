use std::num::ParseIntError;


type Res = Result<i32, ParseIntError>;
pub fn convert_string_to_int(arg1: &str) -> Res {
    match arg1.parse::<i32>() {
        Ok(a) => Ok(a),
        Err(e) => Err(e) 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Should return Ok result with correct integer value when given a valid string input
    #[test]
    fn should_return_ok_with_correct_integer_value() {
        let arg1 = "123";
        let result = convert_string_to_int(&arg1);
        assert_eq!(result, Ok(123));
    }

    // Should return Err result when given an empty string input
    #[test]
    fn should_return_err_with_empty_string_input() {
        let arg1 = "";
        let result = convert_string_to_int(&arg1);
        assert!(result.is_err());
    }
}



