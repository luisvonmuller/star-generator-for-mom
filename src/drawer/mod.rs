/* Will Draw Stuff */
/* TODO:
* Function that receives the Student Structure And Just Draw it from the Base SVG *
*/
use crate::Student;
use std::error::Error;

pub async fn process(info: &Student) -> Result<(), Box<dyn Error>> {
    println!("{:?}", info);
    Ok(())
}
