use nom::{
  bytes::complete::tag,
  character::complete::{alphanumeric1 as field, line_ending},
  multi::separated_list1,
  sequence::separated_pair,
  IResult,
};

fn csv_line(input: &str) -> IResult<&str, Vec<&str>> {
  separated_list1(tag(","), field)(input)
}

fn csv(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
  separated_list1(line_ending, csv_line)(input)
}

fn main() {
  let input = "field1,field2,field3\nfield4,field5,field6";
  let result = csv(input);
  match result {
    Ok((_, parsed)) => {
      for line in parsed {
        println!("{:?}", line);
      }
    }
    Err(e) => println!("Error: {:?}", e),
  }
}