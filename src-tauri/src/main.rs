#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


#[tauri::command]
fn calculator(invoke_message: String) -> String{

  let mut operator = String::from("");

  let operators = ['+', '-', '÷', 'x'];

  let mut num1 = String::from("");
 
  let mut num2 = String::from("");


  for e in invoke_message.chars(){

      for opes in operators{
        if e == opes {
          operator = e.to_string();
        }
      }

      if operator == "" {
      
        num1.push(e);
      
      }else{

        if e.to_string() == operator{

        }else{
          
          num2.push(e);

        }
      }


  }


 if(num1 != "" && num2 != ""){
  let number1 :f32 = num1.parse().unwrap();
  let number2 :f32 = num2.parse().unwrap();

  let mut final_result = 0.0;

 match operator.as_str() {
    // The arms of a match must cover all the possible values
    "+" => final_result = number1 + number2,
    "-" => final_result = number1 - number2,
    "x" => final_result = number1 * number2,
    "÷" => final_result = number1 / number2,
    _=> println!("Algo mas"),
    // TODO ^ Try commenting out one of these arms
  };

  final_result.to_string().into()
 }else{
   "SyntaxError".into()
 }

}


fn main() {
  tauri::Builder::default() 
    .invoke_handler(tauri::generate_handler![calculator])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
