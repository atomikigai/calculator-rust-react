#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


#[tauri::command]
fn calculator(invoke_message: String) -> String{

  let mut operator = String::from("");

  let operators = ['+', '-', '÷', 'x', '%'];

  let mut num1 = String::from("");
 
  let mut num2 = String::from("");




  for e in invoke_message.chars(){

    println!("Valores: {}", e.to_string());
    
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

  println!("final result: {}", final_result);


  final_result.to_string().into()

 }else{
   "SyntaxError".into()
 }



}

#[tauri::command]
fn porcentaje(invoke_message2: String) -> String{

  println!("Porcentaje {}", invoke_message2);
  let mut final_result2 = 0.0;

  if(invoke_message2 != ""){
    let numerofinal :f32 = invoke_message2.parse().unwrap();
    final_result2 = numerofinal / 100.00;
    return final_result2.to_string().into()
  }else{
    return "SyntaxError".into()
  }

  final_result2.to_string().into()
}


fn main() {
  tauri::Builder::default() 
    .invoke_handler(tauri::generate_handler![calculator, porcentaje])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
