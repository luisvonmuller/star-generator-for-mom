/* Will Draw Stuff */
/* TODO:
* Function that receives the Student Structure And Just Draw it from the Base SVG *
*/
use crate::Student;

pub async fn process(info: &Student) -> Result<String, ()> {
    println!("I'm done Drawing the Star of: {:#?}", info);
    Ok("Thread done.".to_string())
}
